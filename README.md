[![CI](https://github.com/dxshie/kv2/actions/workflows/ci.yml/badge.svg)](https://github.com/dxshie/kv2/actions/workflows/ci.yml) [![Crates.io Version](https://img.shields.io/crates/v/kv2)](https://crates.io/crates/kv2) [![docs.rs](https://img.shields.io/docsrs/kv2)](https://docs.rs/crate/kv2/latest)

# kv2

A Rust crate for parsing Valve's KeyValues2 (KV2) format.

## Overview

`kv2` is a Rust library for parsing and serializing the [KeyValues2 (KV2)](https://developer.valvesoftware.com/wiki/KeyValues2) format used by Valve in their games and tools. It allows you to read KV2 files and access their data in a structured way.


## Features

- **Parsing**: Parsing KV2 Format.
- **Deserialization**: Deserialization Serde Support for the KV2 parsing.
- **Serialization**: TODO.
- **Handles Various Data Types**: Supports booleans, integers, floats, strings, arrays, hex arrays(binary blobs), objects, and null values.
- **Customizable Parsing**: Built using the [`nom`](https://github.com/Geal/nom) parser combinator library for flexibility.

## Installation

Add `kv2` to your `Cargo.toml` dependencies:

```toml
[dependencies]
kv2 = { version = "0.1.2", features = ["serde"] }
```

## Example

```rust
use kv2::parse_kv2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DmElement {
    id: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DmeModel {
    id: String,
    visible: bool,
}

fn main() {
    let input = r#"
"DmElement"
{
"id" "elementid" "df939bf4-8dd6-435c-9eef-a6e25434ecca"
"name" "string" "root"
}

"DmeModel"
{
"id" "elementid" "90e0ae34-0671-478d-95f5-12fa5c905c7a"
"visible" "bool" "1"
}
        "#;

    match parse_kv2(input) {
        Ok(data) => {
            let element = DmElement::deserialize(data.1[0].clone());
            let model = DmeModel::deserialize(data.1[1].clone());
        }
        Err(e) => {
            error!("{:?}", e);
        }
    }
}
```