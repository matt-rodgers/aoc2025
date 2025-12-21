#! /bin/bash

DAY=$1
perf record --call-graph dwarf ./target/release/aoc2025 $DAY
perf script | inferno-collapse-perf > stacks.folded
cat stacks.folded | inferno-flamegraph > flamegraph.svg
