```sh
v_notify -a=tg -l=error "yak is not shaved"
```

`-a/--adapter` defaults to `tg`; `-l/--level` is required. A level is `1`-`5`, a color (`blue green yellow orange red`), or a tracing name (`trace info debug warn error`) — all three name the same five levels.

## Configuration

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
