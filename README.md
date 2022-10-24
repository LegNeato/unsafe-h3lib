## About

A pure rust implementation of [h3](https://github.com/uber/h3). `h3`'s C code was translated to Rust via [c2rust](https://github.com/immunant/c2rust) and then manually edited to get it to work.

## Status

- The `h3lib` library builds via cargo.
- The helper apps, tests, and benchmarks all build and run via cargo.
- _A couple of the tests fail due to lack of `f128` support in Rust._

## Licence

The majority of this code is auto-translated from `h3`, so the same license applies (Apache 2.0).
