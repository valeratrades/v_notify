use clap::{Args, Parser, Subcommand};
use v_utils::io::ExpandedPath;

#[derive(Parser, Default)]
#[command(author, version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
	#[arg(long, default_value = "~/.config/what.toml")]
	config: ExpandedPath,
}
#[derive(Subcommand, Debug)]
enum Commands {
	Send(SendArgs),
}
impl Default for Commands {
	fn default() -> Self {
		Commands::Send(SendArgs::default())
	}
}

#[derive(Clone, Debug, Default, Copy)]
struct Importance(pub u8);
impl std::str::FromStr for Importance {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		range_value_from_str(s).map(Importance)
	}
}

#[derive(Clone, Debug, Default, derive_new::new, Copy)]
struct Urgency(pub u8);
impl std::str::FromStr for Urgency {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		range_value_from_str(s).map(Urgency)
	}
}

fn range_value_from_str(s: &str) -> Result<u8, String> {
	let value = s.parse().map_err(|_| "Value must be a number")?;
	if !(1..=5).contains(&value) {
		Err("Value must be between 0 and 10".to_string())
	} else {
		Ok(value)
	}
}

#[derive(Args, Default, Debug)]
struct SendArgs {
	text: String,
	#[arg(short, long)]
	importance: Option<Importance>,
	#[arg(short, long)]
	urgency: Option<Urgency>,
}

fn main() {
	let cli = Cli::parse();
	match cli.command {
		Commands::Send(args) => start(args),
	}
}

fn start(args: SendArgs) {
	send_the_message(args.text, args.importance, args.urgency);
}

fn send_the_message(text: String, importance: Option<Importance>, urgency: Option<Urgency>) {
	eprintln!("Ignoring importance for now.\nImportance: {}", importance.unwrap_or_default().0);
	let notify_send_urgency = match urgency {
		Some(u) => match u.0 {
			1..=3 => "normal",
			_ => "critical",
		},
		None => "low",
	};
	std::process::Command::new("notify-send")
		.arg(text)
		.arg("-i")
		.arg("dialog-information")
		.arg("-t")
		.arg("5000")
		.arg("-u")
		.arg(notify_send_urgency)
		.spawn()
		.expect("Failed to send message");
}
