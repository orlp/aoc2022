# AoC 2022

My solutions for the 2022 edition of Advent of Code in Rust. Simply run `cargo
run --release --bin dayXX` to solve.

Some days require `z3`, install it to the system libraries, or on Windows
download binaries from https://github.com/Z3Prover/z3/releases and unzip to a
folder named `z3` in the crate root.

Alternatively you can run those days with `--features static-link-z3`, although
compilation might take quite a while as z3 is quite big.
