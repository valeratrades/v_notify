```sh
cargo install --git https://github.com/valeratrades/v_notify
```

## Telegram

1. Create a bot via [@BotFather](https://t.me/BotFather) and grab its token → `telegram.bot_token`.
2. Add the bot to each target channel/group as an admin.
3. Set each level's `chat_id` (a `@name`, or the numeric `-100…` id; append `:thread_id` for a forum topic).

## Discord

For each target channel: *Edit Channel → Integrations → Webhooks → New Webhook → Copy URL*, then set it as that level's value.
