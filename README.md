# Reviewing sub-array parsing performance

Reviewing the following Medium article about various language performance, rust landing so slow was quite a surprise.
https://medium.com/@shockerovip/which-programming-language-is-faster-993ea2b0b19f

The following code was used for the testing.
https://github.com/FredySandoval/benchmark-indexof/blob/main/rust/indexof/src/main.rs

Adjusted a bit to be rust only, generate a file with make_sample_file.sh.

And benchmarking can be done with benchmark.sh (after installing hyperfine) or just `cargo run --release` with the already provided elapsed time stamping for a rough estimate.

The find_subsequence_v2 function shows about 45% performance increase, putting it roughly inline with the python bench on my system.

The find_subsequence_v3 function in main.rs shows about 75% performance improvement over find_subsequence now, and uses memchr crate, which essentially calls into the same memmem that occurs in C. So I'd almost identical performance between them now.

On my M1 mac, the python test is about 3.9 seconds vs the improved rust test being 2.6 seconds.

I think an important take away here is the runtime overhead, which is non-existance with rust, similar to C. A fair implementation for all languages should all try to depend on the same libc functionality to properly measure the runtime impact.
