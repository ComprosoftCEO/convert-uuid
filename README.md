# Convert-UUID

Rust utility to convert between normal and base-64 UUID representations

<br />

## Compiling and Running

After you [download and install Rust](https://www.rust-lang.org/tools/install), run the following commands to build and run the program:

```bash
cargo build
cargo run
```

The utility also includes an optional `clipboard` feature for copying text to the clipboard.
To compile this, run:

```
cargo build --features clipboard
```

On Ubuntu, you will need to install the following packages for the `clipboard` to compile:

```
sudo apt install xorg-dev libxcb-render-util0-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

<br />

## Usage

```
convert-uuid 0.1.0
Convert between normal and base-64 UUID representations.

UUIDs can be entered using 32-character, 36-character, or base-64 representation.

USAGE:
    convert-uuid [FLAGS] [input]

FLAGS:
    -b, --copy-b64       Copy the base-64 UUID to the system clipboard
    -n, --copy-normal    Copy the normal UUID to the system clipboard
    -h, --help           Prints help information
    -V, --version        Prints version information

ARGS:
    <input>    UUID to convert  (Generates a random UUID if unset)
```

**Note:** The `--copy-b64` and `--copy-normal` flags are only available if the `clipboard` feature is compiled.
