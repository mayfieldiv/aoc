#! /bin/bash

day=$1
part=${2:-part2}

if [ -z "$day" ]; then
    echo "Usage: ./run.sh <day> [part1|part2]"
    exit 1
fi

cargo run --features $part --bin $day < $day.txt
