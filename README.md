# Squarify-rs

[![Build Status](https://img.shields.io/travis/com/dweb0/squarify-rs?style=flat-square)](https://travis-ci.com/dweb0/squarify-rs)
[![License](https://img.shields.io/badge/License-MIT/Apache--2.0-blue.svg?style=flat-square)](https://github.com/dweb0/squarify-rs/blob/master/LICENSE-APACHE)

Rust implementation of the squarify algorithm.

This is a direct translation of the python implementation by [here](https://github.com/laserson/squarify). All credit goes to these developers.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
squarify = { git = "https://github.com/dweb0/squarify-rs" }
```

> Will eventually publish to crates.io when ready.

## Example

```rust
use squarify::squarify;

fn main() {
    let values = vec![500.0, 433.0, 78.0, 25.0, 25.0, 7.0];
    let rects = squarify(&values, 0.0, 0.0, 1000.0, 1000.0, None);

    for rect in rects {
        println!("{:?}", rect);
    }
    
    // Output
    // Rect { x: 0.0, y: 0.0, dx: 1000.0000000000001, dy: 468.16479400749057 }
    // Rect { x: 0.0, y: 468.16479400749057, dx: 1000.0000000000001, dy: 405.43071161048687 }
    // Rect { x: 0.0, y: 873.5955056179774, dx: 1000.0000000000001, dy: 73.03370786516854 }
    // Rect { x: 0.0, y: 946.629213483146, dx: 1000.0000000000001, dy: 23.40823970037453 }
    // Rect { x: 0.0, y: 970.0374531835206, dx: 1000.0000000000001, dy: 23.40823970037453 }
    // Rect { x: 0.0, y: 993.4456928838952, dx: 1000.0000000000001, dy: 6.5543071161048685 }
}
```