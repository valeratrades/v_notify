CLI that fires a one-shot notification at an importance level, routed to a per-level channel.

Each importance level (`trace`/`info`/`debug`/`warn`/`error`, 1-5) maps to its own destination on the chosen adapter, so noise and alarms land in separate channels. Telegram and Discord are supported.
