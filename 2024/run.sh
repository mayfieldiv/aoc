#! /bin/bash

day=$1
part=${2:-part2}
rest=${@:3}

if [ -z "$day" ]; then
    echo "Usage: ./run.sh <day> [part1|part2] [--release]"
    exit 1
fi

cargo run --features $part --bin $day $rest < $day.txt
