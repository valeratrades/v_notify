# v_notify
![Minimum Supported Rust Version](https://img.shields.io/badge/nightly-2026--06--29+-ab6000.svg)
[<img alt="crates.io" src="https://img.shields.io/crates/v/v_notify.svg?color=fc8d62&logo=rust" height="20" style=flat-square>](https://crates.io/crates/v_notify)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs&style=flat-square" height="20">](https://docs.rs/v_notify)
![Lines Of Code](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/valeratrades/b48e6f02c61942200e7d1e3eeabf9bcb/raw/v_notify-loc.json)
![Build time](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/valeratrades/b48e6f02c61942200e7d1e3eeabf9bcb/raw/v_notify-build.json)
![Test time](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/valeratrades/b48e6f02c61942200e7d1e3eeabf9bcb/raw/v_notify-test.json)
<br>
[![errors](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/valeratrades/b48e6f02c61942200e7d1e3eeabf9bcb/raw/v_notify-errors.json)](https://github.com/valeratrades/v_notify/actions/workflows/errors.yml?query=branch%3Amain)
[![warnings](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/valeratrades/b48e6f02c61942200e7d1e3eeabf9bcb/raw/v_notify-warnings.json)](https://github.com/valeratrades/v_notify/actions/workflows/warnings.yml?query=branch%3Amain)

CLI that fires a one-shot notification at an importance level, routed to a per-level channel.

Each importance level (`trace`/`info`/`debug`/`warn`/`error`, 1-5) maps to its own destination on the chosen adapter, so noise and alarms land in separate channels. Telegram and Discord are supported.
<!-- markdownlint-disable -->
<details>
<summary>
<h2>Installation</h2>
</summary>

```sh
cargo install --git https://github.com/valeratrades/v_notify
```

### Telegram

1. Create a bot via [@BotFather](https://t.me/BotFather) and grab its token → `telegram.bot_token`.
2. Add the bot to each target channel/group as an admin.
3. Set each level's `chat_id` (a `@name`, or the numeric `-100…` id; append `:thread_id` for a forum topic).

### Discord

For each target channel: *Edit Channel → Integrations → Webhooks → New Webhook → Copy URL*, then set it as that level's value.


</details>
<!-- markdownlint-restore -->

## Usage
```sh
v_notify -a=tg -l=error "yak is not shaved"
```

`-a/--adapter` defaults to `tg`; `-l/--level` is required. A level is `1`-`5`, a color (`blue green yellow orange red`), or a tracing name (`trace info debug warn error`) — all three name the same five levels.

### Configuration

Fill in `~/.config/v_notify.toml`. Every level of the **chosen** adapter must have a group, or the call errors out; the other adapter is never touched.

```toml
[telegram]
bot_token = { env = "TELEGRAM_BOT_TOKEN" }
# chat_id (`@name` or numeric `-100…`), optionally `chat_id:thread_id` for a forum topic
trace = "@my_log_channel"
info  = "@my_log_channel"
debug = "@my_log_channel"
warn  = "@my_alerts_channel"
error = "-1001800341082:42"

[discord]
# per-level webhook URLs
trace = { env = "DISCORD_LOG_WEBHOOK" }
info  = { env = "DISCORD_LOG_WEBHOOK" }
debug = { env = "DISCORD_LOG_WEBHOOK" }
warn  = { env = "DISCORD_ALERTS_WEBHOOK" }
error = { env = "DISCORD_ALERTS_WEBHOOK" }
```

Any field also accepts a `--telegram-error`, `--discord-warn`, … flag for one-off overrides.



<br>

#### Dev links
- <sup><a href="./docs/ARCHITECTURE.md">architecture</a></sup>
- <sup><a href="https://github.com/EV-Main/.github/blob/main/code/STYLE.md">style</a></sup>
- <sup><a href="https://github.com/EV-Main/.github/tree/main/code/conventions">conventions</a></sup>
- <sup><a href="https://github.com/EV-Main/.github/blob/main/code/conventions/git">contributing</a></sup>

#### License

<sup>
	Licensed under <a href="LICENSE">Blue Oak 1.0.0</a>
</sup>

<br>

<sub>
	Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be licensed as above, without any additional terms or conditions.
</sub>

