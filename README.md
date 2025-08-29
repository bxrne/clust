# clust

A TUI for checking out your kubernetes cluster.

## Usage

```bash
cargo run
```

- Press `q` to quit
- Type `:pods` or `:help` in the command entry box to switch views

## Development

- Requires Rust and Cargo
- UI built with [ratatui](https://github.com/ratatui-org/ratatui) and [crossterm](https://github.com/crossterm-rs/crossterm)

## Testing

To run tests:

```bash
cargo test
```

## CI/CD

- CI: Build and test on every push
- CD: Release build and optional deploy (see `.github/workflows`)

