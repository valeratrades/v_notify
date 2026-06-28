#![feature(default_field_values)]
use std::str::FromStr;

use color_eyre::eyre::{Result, bail};

pub mod config;
use config::AppConfig;

/// Importance of a message: 5 levels in rising order. The mapping to colors and to
/// tracing levels is forever backwards-compatible.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Importance {
	Trace,
	Info,
	Debug,
	Warn,
	Error,
}

impl Importance {
	pub const ALL: [Importance; 5] = [Importance::Trace, Importance::Info, Importance::Debug, Importance::Warn, Importance::Error];
}

impl FromStr for Importance {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			"blue" | "trace" | "1" => Importance::Trace,
			"green" | "info" | "2" => Importance::Info,
			"yellow" | "debug" | "3" => Importance::Debug,
			"orange" | "warn" | "4" => Importance::Warn,
			"red" | "error" | "5" => Importance::Error,
			_ => return Err(format!("`{s}` is not a level (expected 1-5, a color, or a tracing level)")),
		})
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Color {
	Blue,
	Green,
	Yellow,
	Orange,
	Red,
}
impl Color {
	pub fn emoji(self) -> char {
		match self {
			Color::Blue => '🔵',
			Color::Green => '🟢',
			Color::Yellow => '🟡',
			Color::Orange => '🟠',
			Color::Red => '🔴',
		}
	}
}

impl From<Importance> for Color {
	fn from(i: Importance) -> Self {
		match i {
			Importance::Trace => Color::Blue,
			Importance::Info => Color::Green,
			Importance::Debug => Color::Yellow,
			Importance::Warn => Color::Orange,
			Importance::Error => Color::Red,
		}
	}
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, clap::ValueEnum)]
pub enum Adapter {
	#[default]
	#[value(name = "tg", alias = "telegram")]
	Tg,
	#[value(name = "discord", alias = "dc")]
	Discord,
}

/// A fully-resolved destination to post to, for the chosen adapter.
#[derive(Debug)]
pub enum Target {
	/// `chat` is the bot-API `chat_id`, optionally `chat_id:thread_id` for a forum topic.
	Telegram {
		bot_token: String,
		chat: String,
	},
	DiscordWebhook(String),
}

impl AppConfig {
	/// Resolve the send target for `adapter` at `level`. Only the chosen adapter is inspected
	/// (we never require config for an adapter the user didn't pick). Every importance level of
	/// the chosen adapter must have a group configured — otherwise we error out.
	pub fn resolve(&self, adapter: Adapter, level: Importance) -> Result<Target> {
		match adapter {
			Adapter::Tg => {
				if self.telegram.bot_token.is_empty() {
					bail!("telegram: `bot_token` is not configured");
				}
				let missing: Vec<&str> = Importance::ALL.iter().filter(|&&l| self.telegram.group(l).is_none()).map(level_name).collect();
				if !missing.is_empty() {
					bail!("telegram: no group configured for level(s): {}", missing.join(", "));
				}
				Ok(Target::Telegram {
					bot_token: self.telegram.bot_token.clone(),
					chat: self.telegram.group(level).expect("validated above").clone(),
				})
			}
			Adapter::Discord => {
				let missing: Vec<&str> = Importance::ALL.iter().filter(|&&l| self.discord.group(l).is_none()).map(level_name).collect();
				if !missing.is_empty() {
					bail!("discord: no webhook configured for level(s): {}", missing.join(", "));
				}
				Ok(Target::DiscordWebhook(self.discord.group(level).expect("validated above").clone()))
			}
		}
	}
}

/// Fire the notification. Blocking single-shot POST — this is a one-and-done CLI, no runtime.
pub fn send(target: &Target, color: Color, text: &str) -> Result<()> {
	let body = format!("{} {text}", color.emoji());
	let client = reqwest::blocking::Client::new();
	let response = match target {
		Target::Telegram { bot_token, chat } => {
			let url = format!("https://api.telegram.org/bot{bot_token}/sendMessage");
			// ponytail: forum topics encoded as `chat_id:thread_id`; split here rather than a destination type.
			let (chat_id, thread) = chat.split_once(':').map_or((chat.as_str(), None), |(c, t)| (c, Some(t)));
			let mut payload = serde_json::json!({ "chat_id": chat_id, "text": body });
			if let Some(thread) = thread {
				let thread: i64 = thread.parse().map_err(|_| color_eyre::eyre::eyre!("telegram topic id `{thread}` is not an integer"))?;
				payload["message_thread_id"] = thread.into();
			}
			client.post(url).json(&payload).send()?
		}
		Target::DiscordWebhook(url) => client.post(url).json(&serde_json::json!({ "content": body })).send()?,
	};
	if !response.status().is_success() {
		let status = response.status();
		bail!("send failed ({status}): {}", response.text().unwrap_or_default());
	}
	Ok(())
}
fn level_name(l: &Importance) -> &'static str {
	match l {
		Importance::Trace => "trace",
		Importance::Info => "info",
		Importance::Debug => "debug",
		Importance::Warn => "warn",
		Importance::Error => "error",
	}
}
