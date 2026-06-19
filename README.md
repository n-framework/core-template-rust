# NFramework Core Template (Rust)

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)

Template-rendering building blocks for the NFramework Rust toolchain. This workspace defines engine-agnostic rendering abstractions and provides adapters for popular template engines, following NFramework's zero-dependency-core, replaceable-adapter model.

These crates power generator rendering in the `nfw` CLI and [nfw-generators](../nfw-generators).

---

## Crates

| Crate | Role |
| --- | --- |
| `n-framework-core-template-abstractions` | Core rendering contracts. No third-party engine dependencies. |
| `n-framework-core-template-tera` | Adapter implementing rendering with [Tera](https://crates.io/crates/tera). |
| `n-framework-core-template-mustache` | Adapter implementing rendering with [Mustache](https://crates.io/crates/mustache). |

Abstractions never depend on adapters; swapping the rendering engine does not touch consumer code.

---

## Build

```bash
make build
```

## Test

```bash
make test
```

## Format & Lint

```bash
make format
make lint
```

## Setup

```bash
make setup
```

---

## License

This project is licensed under the **Apache License 2.0** - see the [LICENSE](LICENSE) file for details.
