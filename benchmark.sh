TIMES=100
# Get the python version
VERSION=rustc_1.61.0
hyperfine --runs $TIMES --time-unit second "./target/release/rust-bench" --warmup 3 --export-json ./bench_$VERSION.json -n $VERSION 