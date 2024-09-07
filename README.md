# `az` - Aztec encoder CLI

A small CLI tool to create Aztec codes and share data with your phone.

## Usage

To create an Aztec code from a string use:

```sh
$ az "Hello World! 🗺️"
```

`az` also listens on stdin:

```sh
echo "Good morning" | az -
```

For scanning use a barcode reader like [Binary Eye](https://github.com/markusfisch/BinaryEye).


## Build

Run `cargo build --release`
