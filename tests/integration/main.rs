//! Single integration-test binary, per https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html

use v_notify::{Adapter, Color, Importance, config::AppConfig};

#[test]
fn importance_parses_from_number_color_and_tracing_level() {
	// each level is reachable by its 1-5 rank, its color, and its tracing name
	for (n, color, tracing, level) in [
		("1", "blue", "trace", Importance::Trace),
		("2", "green", "info", Importance::Info),
		("3", "yellow", "debug", Importance::Debug),
		("4", "orange", "warn", Importance::Warn),
		("5", "red", "error", Importance::Error),
	] {
		assert_eq!(n.parse::<Importance>().unwrap(), level);
		assert_eq!(color.parse::<Importance>().unwrap(), level);
		assert_eq!(tracing.parse::<Importance>().unwrap(), level);
		assert_eq!(Color::from(level).emoji(), color.parse::<Importance>().map(|i| Color::from(i).emoji()).unwrap());
	}
	assert!("0".parse::<Importance>().is_err());
	assert!("purple".parse::<Importance>().is_err());
}

#[test]
fn resolve_errors_when_chosen_adapter_unconfigured() {
	// Empty config: neither adapter configured → resolve names the missing adapter.
	let cfg = AppConfig::default();

	let err = cfg.resolve(Adapter::Tg, Importance::Error).unwrap_err().to_string();
	assert!(err.contains("telegram"), "{err}");

	let err = cfg.resolve(Adapter::Discord, Importance::Error).unwrap_err().to_string();
	assert!(err.contains("discord"), "{err}");
}
