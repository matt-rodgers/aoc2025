# Advent of code 2025

## Profiling steps

To profile a single day and get an SVG flamegraph:

1. Ensure `inferno` tool is installed (`cargo install --locked inferno`)
2. Record data using perf: `perf record --call-graph dwarf ./target/release/aoc2025 <day>`
3. Collapse stacks: `perf script | inferno-collapse-perf > stacks.folded`
4. Make the flamegraph: `cat stacks.folded | inferno-flamegraph > flamegraph.svg`
5. Open in browser (for zoomable graph).

These steps are combined in the script `profile.sh`. Usage is `./profile.sh <DAY>`
