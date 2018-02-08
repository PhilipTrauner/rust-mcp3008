# rust-mcp3008

[![](https://img.shields.io/crates/v/mcp3008.svg)](https://crates.io/crates/mcp3008)
[![](https://docs.rs/mcp3008/badge.svg)](https://docs.rs/mcp3008)
[![](https://travis-ci.org/PhilipTrauner/rust-mcp3008.svg?branch=master)](https://travis-ci.org/PhilipTrauner/rust-mcp3008)

<p align="center">
	<img src="https://user-images.githubusercontent.com/9287847/35982700-b8731460-0cf0-11e8-836a-42537d76396e.png" height="300"/>
</p>

<p align="center">
	<strong>MCP3008 A/D converter</strong>
</p>


`rust-mcp3008` is a rewrite of the excellent [Adafruit_Python_MCP3008](https://github.com/adafruit/Adafruit_Python_MCP3008) Python library in Rust. 

## Usage
<details>
<summary>
Cargo.toml
</summary>

```toml
[dependencies]
mcp3008 = "1.0.0"
```

</details>

<p></p>

```rust
extern crate mcp3008;

use mcp3008::Mcp3008;

fn main() {
    if let Ok(mut mcp3008) = Mcp3008::new("/dev/spidev0.0") {
        println!("{}", mcp3008.read_adc(0).unwrap());
    }
}
```
