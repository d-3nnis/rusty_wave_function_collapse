# Rusty Wave Function Collapse

This project is a very simple implementation of the Wave Function Collapse algorithm in Rust. It currently only supports ASCII tiles and direct adjacancy rules.

## Installation

```sh
git clone https://github.com/d-3nnis/rusty_wave_function_collapse.git
cd rusty_wave_function_collapse
cargo build --release
```

## Running sample code

To run the example `generate` binary

```sh
cargo run --release --bin generate
```


This will generate a pattern using the Wave Function Collapse algorithm and print it to the console.

![Image](https://github.com/user-attachments/assets/f0f2b0bd-acad-4f4a-97c8-0ca7da1b6be5)

## Usage

To include the project as a library in other projects, you can follow these steps:

* Add the following to your `Cargo.toml` file:
  ```toml
  [dependencies]
  rusty_wave_function_collapse = { path = "path/to/rusty_wave_function_collapse" }
  ```
  Replace `"path/to/rusty_wave_function_collapse"` with the actual path to the `rusty_wave_function_collapse` project.

For a simple example, reference `bin/generate.rs`.

## Contributing

Contributions are welcome! If you would like to contribute to this project, please follow these steps:

