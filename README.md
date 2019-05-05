This is just a little benchmarking exercise to determine how efficient
[bincode](https://github.com/TyOverby/bincode) is when serializing and
deserializing structures that contains large byte slices
(e.g. uncompressed video frames).  Turns out it's pretty darn
efficient as long as you use
[serde_bytes](https://github.com/serde-rs/bytes) and zero-copy
deserialization (e.g. deserialize from a byte slice, which is your
only option anyway if your struct contains slices).

Here's what I saw on my machine when serializing and deserializing
uncompressed 4K YUV420 frames:

```
 $ cargo +nightly bench
   Compiling bincode-benchmark v0.1.0 (/home/dicej/p/bincode-benchmark)
    Finished release [optimized + debuginfo] target(s) in 1.36s
     Running target/release/deps/bincode_benchmark-7d98c47ce3ad9fac

running 3 tests
test tests::bench_copy_message   ... bench:     954,577 ns/iter (+/- 33,729)
test tests::bench_deserialize    ... bench:          66 ns/iter (+/- 0)
test tests::bench_serialize_into ... bench:     432,790 ns/iter (+/- 5,887)

test result: ok. 0 passed; 0 failed; 0 ignored; 3 measured; 0 filtered out

```
