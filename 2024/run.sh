#! /bin/bash

day=$1

if [ -z "$day" ]; then
    echo "Usage: ./run.sh <day>"
    exit 1
fi

cargo run --bin $day < $day.txt
