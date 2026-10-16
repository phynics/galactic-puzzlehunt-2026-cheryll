# Cheryll

A Rust solver for an Albert/Bernard-style knowledge puzzle (like Cheryl's Birthday). Albert knows the row of a hidden cell, Bernard knows the column, and they make public statements about what they know. The solver searches statement sequences that narrow the candidates down to a single cell.

## Usage

```sh
cargo run
```

This prints the shortest statement sequence that isolates each candidate cell. Other visualizations (step-by-step board renders, horizontal boards, pruned solution tree) are available in `src/main.rs`.
