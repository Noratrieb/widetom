#!/usr/bin/env bash

docker run -d -v "$(pwd)/config.toml:/app/config.toml" -v "$(pwd)/bot_token:/app/bot_token" \
    -e CONFIG_PATH=/app/config.toml -e BOT_TOKEN_PATH=/app/bot_token --name widertom widertom:1.0
