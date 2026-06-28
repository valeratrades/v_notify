use std::time::Duration;

use clap::Parser;
use color_eyre::eyre::Result;
use v_notify::{Adapter, Color, Importance, config::LiveSettings};

#[derive(Parser)]
#[command(author, version = concat!(env!("CARGO_PKG_VERSION"), " (", env!("GIT_HASH"), ")"), about, long_about = None)]
struct Cli {
	#[command(flatten)]
	settings: v_notify::config::SettingsFlags,
	#[arg(short, long, value_enum, default_value_t = Adapter::Tg)]
	adapter: Adapter,
	#[arg(short, long)]
	level: Option<Importance>,
	text: Option<String>,
	#[command(subcommand)]
	command: Option<Command>,
}

#[derive(clap::Subcommand)]
enum Command {
	/// Manage configuration: write defaults, diff against defaults.
	Config {
		#[command(subcommand)]
		cmd: v_notify::config::SettingsCommand,
	},
}

fn main() {
	v_utils::clientside!();
	let cli = Cli::parse();
	if let Some(Command::Config { cmd }) = cli.command {
		// Never returns — performs the requested config operation and exits.
		v_notify::config::AppConfig::handle_settings_command(cmd, cli.settings);
	}
	if let Err(e) = run(cli) {
		eprintln!("Error: {e}");
		for cause in e.chain().skip(1) {
			eprintln!("  Caused by: {cause}");
		}
		std::process::exit(1);
	}
}

fn run(cli: Cli) -> Result<()> {
	let (Some(level), Some(text)) = (cli.level, cli.text) else {
		color_eyre::eyre::bail!("both --level and <TEXT> are required when not running a subcommand");
	};
	let settings = LiveSettings::new(cli.settings, Duration::from_secs(5))?;
	let config = settings.config().map_err(|e| color_eyre::eyre::eyre!("reading config: {e}"))?;
	let target = config.resolve(cli.adapter, level)?;
	v_notify::send(&target, Color::from(level), &text)
}
