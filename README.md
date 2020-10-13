# Infer DateTime format

[![Build Status](https://travis-ci.org/klangner/dtinfer.rs.svg?branch=master)](https://travis-ci.org/klangner/dtinfer.rs)
[![Crates.io](https://img.shields.io/crates/v/dtinfer.svg)](https://crates.io/crates/dtinfer) [![Crates.io](https://img.shields.io/crates/l/dtinfer.svg)](https://github.com/klangner/dtinfer.rs/blob/master/LICENSE-MIT) [![docs.rs](https://docs.rs/dtinfer/badge.svg)](https://docs.rs/dtinfer/)

Library for infering date time format from the given string. 


# Usage

Add dependency to your project
```
dfinfer = "0.1"
```

And then:

```rust
use dtinfer;

let pattern = infer_best("2020-10-09 15:24:50+00:00").unwrap();
assert_eq!(pattern, "%Y-%m-%d %H:%M:%S%z")
```

# Features

Only ISO 8601 like dates are supported as for now. Check tests for examples.

  
# License

Licensed under either of

 * [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
 * [MIT license](http://opensource.org/licenses/MIT)

at your option.


**Contributions**

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
