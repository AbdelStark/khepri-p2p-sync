# khepri-p2p-sync

Khepri P2P Synchronization

> ## ⚠️ WARNING! ⚠️
>
> This repo contains highly experimental code.
> Expect rapid iteration.

## Prerequisites

- [Rust](https://www.rust-lang.org/)
- [Protostar](https://github.com/software-mansion/protostar)
- [Starknet](https://www.cairo-lang.org/docs/quickstart.html#installation)

## 📦 Installation

## 🧱 From crates.io

```bash
cargo install khepri-p2p-sync
```

## 🔧 From source

```bash
cargo install --path .
```

## 🔬 Usage

## 📗 Logging 

Change log level using `RUST_LOG` environment variable.

Example:

```bash
RUST_LOG=debug cargo run
```

## 🌡️ Testing

```bash
cargo test --verbose
```

## 🏄‍♂️ Test coverage

```bash
sh scripts/test_coverage.sh
```

## 🛠️ Development

### Install git hooks

```bash
sh scripts/install_git_hooks.sh
```

## 📄 License

**khepri-p2p-sync** is released under the [MIT](LICENSE).