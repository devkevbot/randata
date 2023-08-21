# Randata

A command line interface which produces randomized output in different formats.

## Examples

### Shuffle the letters in a word given as input

```txt
randata shuffle foobar
aforboo
```

### Shuffle a sequence of numbers given as input

```txt
randata shuffle 1 2 3 4
4 1 2 3
```

### Produce a shuffled sequence of 5 numbers, starting at 10

```txt
randata numbers --length 5 --start 10
13 12 14 11 10
```

### Produce a random color value

```txt
randata color --format hex
#e5f5c1

randata color --format rgb
(112,39,164)

randata color --format hsl
(324,98%,20%)
```

### Produce a random IP address

```txt
randata ip --format ipv4
229.222.240.211

randata ip --format ipv6
4ade:8899:39c3:fc9e:e4fe:e20e:8f4a:2dda
```

### Produce a coin flip

```txt
randata coin-flip
heads
```

### Produce a dice roll

```txt
randata dice-roll --sides 10
7
```

## Help documentation

Full instructions can be found by running `cargo run help`

Command-specific instructions can be found by running `cargo run randata --help`

## Installation

The project executable can either be built and run via `cargo` or installed
locally onto your machine and run via the executable name `randata`.

### Build and run

```sh
cargo run <args>
```

### Install the executable locally

```sh
cargo install --path .
randata <args>
```
