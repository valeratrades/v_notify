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
	level: Importance,
	text: String,
}

fn main() {
	v_utils::clientside!();
	if let Err(e) = run(Cli::parse()) {
		eprintln!("Error: {e}");
		for cause in e.chain().skip(1) {
			eprintln!("  Caused by: {cause}");
		}
		std::process::exit(1);
	}
}

fn run(cli: Cli) -> Result<()> {
	let settings = LiveSettings::new(cli.settings, Duration::from_secs(5))?;
	let config = settings.config().map_err(|e| color_eyre::eyre::eyre!("reading config: {e}"))?;
	let target = config.resolve(cli.adapter, cli.level)?;
	v_notify::send(&target, Color::from(cli.level), &cli.text)
}
