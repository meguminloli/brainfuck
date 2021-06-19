# Brainfuck

Another interpreter of Brainfuck. This time in Rust.

Some tests don't work on debug builds because of overflow, so try it with
```
cargo build --release
```

Usage:
```
brainfuck <input> <output>
where output is an optional argument if you want to transpile to C
```

TODO:
- Optimizations to generated tokens
- Way to change variable type from `std::env::args()`