#!/usr/bin/env sh

APP_NAME="day$1"
INPUT_URL="https://adventofcode.com/2020/day/$1/input"

SESSION_ID=`cat .session_id`
TEMPLATE_FILE="main_template.rs"

cargo new $APP_NAME
cp $TEMPLATE_FILE "$APP_NAME/src/main.rs"
echo 'itertools = "0.9"\nlog = "0.4"\nenv_logger = "0.8"\n[dependencies.advent]\npath = "../advent"' >> "$APP_NAME/Cargo.toml"

curl "$INPUT_URL" -H "Cookie: session=$SESSION_ID" > "$APP_NAME/input"

