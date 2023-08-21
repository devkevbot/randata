# Shuffle

A Rust command line interface which randomizes input and output.

## How to use

### Examples

#### Shuffle the letters in a word given as input

```txt
shuffle word foobar
aforboo
```

#### Shuffle a sequence of numbers given as input

```txt
shuffle seq 1 2 3 4
4 1 2 3
```

#### Produce a shuffled sequence of numbers

```txt
# Produces a shuffled sequence of five numbers, starting at 10.
shuffle numbers --length 5 --start 10
13 12 14 11 10
```

#### Produce a random color value

```txt
shuffle color --format hex
#e5f5c1

shuffle color --format rgb
(112,39,164)

shuffle color --format hsl
(324,98%,20%)
```

### Help documentation

Full instructions can be found by running `cargo run help`

Command-specific instructions can be found by running `cargo run shuffle --help`

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
