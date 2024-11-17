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
kv2 = { version = "0.1.0", features = ["serde"] }