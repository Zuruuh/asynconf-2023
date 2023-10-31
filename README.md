# Asynconf 2023

My submission for the asynconf 2023 coding contest. Built entirely in rust with wasm and leptos

# Update configuration data

You can update the `data.json` file and recompile the application to test the app's behaviour with new values.

## Requirements

```
cargo
rustup
trunk
```

### Warning

Rust takes a reaaaaaaally long time to compile, if you want to speed up the process you can follow the next steps in a virtual machine.

## Installation

```
rustup default nightly
rustup target add wasm32-unknown-unknown
cargo install trunk # can be installed with nix on replit
```

## Run prod build

```
trunk build --release
```

## Preview prod build

```
trunk serve --release
```
