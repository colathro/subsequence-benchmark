# Reviewing sub-array parsing performance

Reviewing the following Medium article about various language performance, rust landing so slow was quite a surprise.
https://medium.com/@shockerovip/which-programming-language-is-faster-993ea2b0b19f

The following code was used for the testing.
https://github.com/FredySandoval/benchmark-indexof/blob/main/rust/indexof/src/main.rs

Adjusted a bit to be rust only, generate a file with make_sample_file.sh.

And benchmarking can be done with benchmark.sh (after installing hyperfine) or just `cargo run --release` with the already provided elapsed time stamping for a rough estimate.

The find_subsequence_v2 function in main.rs shows about 40-45% performance improvement over find_subsequence.
