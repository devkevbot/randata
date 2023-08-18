# Shuffle

A Rust command line interface to shuffle input. 

## Installation

The project executable can either be built and run via `cargo` or installed
locally onto your machine and run via the executable name `shuffle`.

### Build and run

```sh
cargo run <args>
```

### Install the executable locally

```sh
cargo install --path .
shuffle <args>
```

## How to use

### Examples

#### Shuffle a word

```sh
shuffle word foobar
aforboo
```

#### Shuffle a sequence of numbers

```sh
shuffle seq 1 2 3 4
4 1 2 3
```

### Help documentation

Full instructions can be found by running `cargo run help`

Command-specific instructions can be found by running `cargo run shuffle --help`
