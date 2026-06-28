use v_utils::macros as v_macros;

use crate::Importance;

#[derive(Clone, Debug, Default, v_macros::LiveSettings, v_macros::MyConfigPrimitives, v_macros::Settings)]
pub struct AppConfig {
	#[settings(flatten)]
	#[serde(default)]
	pub telegram: Option<Telegram>,
	#[settings(flatten)]
	#[serde(default)]
	pub discord: Option<Discord>,
}

/// Telegram bot, reporting each importance level into its own destination.
/// A group is the bot-API `chat_id` (`@name` or numeric `-100…`), optionally suffixed
/// `:<thread_id>` to target a forum topic.
#[derive(Clone, Debug, Default, v_macros::MyConfigPrimitives, v_macros::SettingsNested)]
pub struct Telegram {
	#[private_value]
	pub bot_token: String,
	#[private_value]
	pub trace: String,
	#[private_value]
	pub info: String,
	#[private_value]
	pub debug: String,
	#[private_value]
	pub warn: String,
	#[private_value]
	pub error: String,
}
impl Telegram {
	pub fn group(&self, level: Importance) -> &String {
		[&self.trace, &self.info, &self.debug, &self.warn, &self.error][level as usize]
	}
}

/// Discord, reporting each importance level into its own channel webhook URL.
#[derive(Clone, Debug, Default, v_macros::MyConfigPrimitives, v_macros::SettingsNested)]
pub struct Discord {
	#[private_value]
	pub trace: String,
	#[private_value]
	pub info: String,
	#[private_value]
	pub debug: String,
	#[private_value]
	pub warn: String,
	#[private_value]
	pub error: String,
}

impl Discord {
	pub fn group(&self, level: Importance) -> &String {
		[&self.trace, &self.info, &self.debug, &self.warn, &self.error][level as usize]
	}
}
