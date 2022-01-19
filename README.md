Library to generate [Snowflake](https://blog.twitter.com/engineering/en_us/a/2010/announcing-snowflake) ids. Uses the [discord](https://discord.com/developers/docs/reference#snowflakes) format for the generated ids.

## ID format

The number of bits are the same as discord's. The timestamp is based off a configurable custom epoch.

| Timestamp   | Datacenter ID  | Machine ID | Sequence  |
| ----------- | -------------- | ---------- | --------- |
| 42 bits     | 5 bits         | 5 bits     | 12 bits   |

Timestamp holds `42` bits, which allows for `2^42 = 4 398 046 511 104 millis ≈ 139 years`.

The sequence supports up to a maximum of `2^12 = 4096` ids per millisecond. If that limit it hit it generates ids for the next millisecond, this allows for a short burst above `4096` ids per millisecond, but it should not be sustained.

## Benchmark

Run `cargo bench` for results, on my machine it results in approximately `92 ns` for each id, so more than 10 million ids per second. When more than `2^12 = 4096` ids are generated per second, it takes ids off the next millisecond.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
