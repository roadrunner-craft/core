# Roadrunner Core ![badge](https://github.com/roadrunner-craft/core/workflows/Rust/badge.svg)

The Core library for roadrunner-related code.

## Features

- World generation and updates
- Entities

## Building

    mise run build

## Testing

    mise run test

## Benching

On a nightly toolchain (`rustup default nightly`)

    cargo bench --features=nightly
