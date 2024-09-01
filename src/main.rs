use clap::{Args, Parser, Subcommand};
use eyre::{bail, Result};
use v_utils::io::ExpandedPath;

#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
	#[arg(long, default_value = "~/.config/v_notify.toml")]
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

/// Importance of the Message
///
/// Values 1-5 in rising order of importance.
/// Guaranteed to be forever backwards-compatible.
#[derive(Clone, Debug, Default, Copy)]
struct Importance(pub u8);
impl std::str::FromStr for Importance {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		range_value_from_str(s).map(Importance)
	}
}

/// Urgency of the Message
//
/// Values 1-5 in rising order of urgency.
/// Guaranteed to be forever backwards-compatible.
#[derive(Clone, Debug, Default, derive_new::new, Copy)]
struct Urgency(pub u8);
impl std::str::FromStr for Urgency {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		range_value_from_str(s).map(Urgency)
	}
}

fn range_value_from_str(s: &str) -> Result<u8, String> {
	fn from_word_color(s: &str) -> Result<u8> {
		match s {
			"blue" => Ok(1),
			"green" => Ok(2),
			"yellow" => Ok(3),
			"orange" => Ok(4),
			"red" => Ok(5),
			_ => bail!("Invalid color"),
		}
	}
	if let Ok(value) = from_word_color(s) {
		return Ok(value);
	}

	fn from_word_tracing(s: &str) -> Result<u8> {
		match s {
			"trace" => Ok(1),
			"info" => Ok(2),
			"debug" => Ok(3),
			"warn" => Ok(4),
			"error" => Ok(5),
			_ => bail!("Invalid tracing level"),
		}
	}
	if let Ok(value) = from_word_tracing(s) {
		return Ok(value);
	}

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
	send_with_tg_cli(args.text, args.importance, args.urgency);
}

fn send_with_tg_cli(text: String, importance: Option<Importance>, urgency: Option<Urgency>) {
	eprintln!(
		"Ignoring importance and urgency for now.\nImportance: {}\nUrgency: {}",
		importance.unwrap_or_default().0,
		urgency.unwrap_or_default().0
	);
	std::process::Command::new("tg")
		.arg("send")
		.arg("-d")
		.arg("-1001800341082")
		.arg(text)
		.spawn()
		.expect("Failed to send message");
}

#[allow(dead_code)]
fn send_with_notifysend(text: String, importance: Option<Importance>, urgency: Option<Urgency>) {
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

#[cfg(test)]
mod tests {
	use eyre::Result;

	#[test]
	fn cli() -> Result<()> {
		let cli: super::Cli = super::Parser::try_parse_from(["v_notify", "send", "Hello, World!"])?;
		insta::assert_debug_snapshot!(cli.command, @r###"
  Send(
      SendArgs {
          text: "Hello, World!",
          importance: None,
          urgency: None,
      },
  )
  "###);
		Ok(())
	}
}
