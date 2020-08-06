# creek

Creek is a small, simple library for doing flow analysis.

## Usage



## Dependencies

By default, Creek uses [fnv](https://doc.servo.org/fnv/) instead of the standard
[`SipHasher`](https://doc.rust-lang.org/std/hash/struct.SipHasher.html) as it is
more performant on small keys, such as integers.
