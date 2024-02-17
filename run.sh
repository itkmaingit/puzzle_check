#!/bin/bash

# 引数からnameを取得
name="$1"

# 引数が提供されているか確認
if [ -z "$name" ]; then
    echo "Usage: $0 <name>"
    exit 1
fi

# Cargoコマンドを実行
cargo run --release --bin "$name" > "data/${name}.txt"