# Garde-actix-web &emsp; [![Documentation]][docs.rs] [![Latest Version]][crates.io] [![Build Status]][build]

[docs.rs]: https://docs.rs/garde-actix-web/

[crates.io]: https://crates.io/crates/garde-actix-web

[build]: https://github.com/netwo-io/garde-actix-web/actions/workflows/build.yaml?branch=main

[Documentation]: https://img.shields.io/docsrs/garde-actix-web

[Latest Version]: https://img.shields.io/crates/v/garde-actix-web.svg

[Build Status]: https://github.com/netwo-io/garde-actix-web/actions/workflows/build.yaml/badge.svg?branch=main

Actix-web wrapper for [garde](https://github.com/jprochazk/garde), a Rust validation library.

- [Installation](#installation)
- [Usage example](#usage-example)
- [Feature flags](#feature-flags)
- [Compatibility matrix](#compatibility-matrix)
- [About us](#about-us)

### Installation

```toml
[dependencies]
garde = "0.22"
garde-actix-web = "0.12"
```

### Usage example

Simply use `garde-actix-web` exposed types as a drop in for actix types.

Your types must implement `Validate` from `garde`. Validation happens during actix's `FromRequest` invocation.

If the payload is invalid, a 400 error is returned (404 for Path).

Custom error handling can be implemented with an extractor config (`garde_actix_web::web::QueryConfig` in place
of `actix_web::web::QueryConfig` for example).

```rust
use actix_web::HttpResponse;
// instead of actix_web::web::Path
use garde_actix_web::web::Path;
use garde::Validate;

#[derive(Validate)]
struct MyStruct<'a> {
  #[garde(ascii, length(min = 3, max = 25))]
  username: &'a str,
}

fn test(id: Path<MyStruct>) -> HttpResponse {
  todo!()
}
```
Context needs to be provided through actix's `data` or `app_data`, if not found default will be used instead.

### Feature flags

| name       | description                                                    | extra dependencies                              |
|------------|----------------------------------------------------------------|-------------------------------------------------|
| `serde_qs` | Enables the usage of `garde` for `serde_qs::actix::QsQuery<T>` | [`serde_qs`](https://crates.io/crates/serde_qs) |

### Compatibility matrix

| garde version | serde_qs version | garde-actix-web-version |
|---------------|------------------|-------------------------|
| `0.14`        | `0.12`           | `0.1.x`                 |
| `0.15`        | `0.12`           | `0.2.x`                 |
| `0.16`        | `0.12`           | `0.3.x`                 |
| `0.17`        | `0.12`           | `0.4.x`                 |
| `0.18`        | `0.12`           | `0.5.x`, `0.6.x`        |
| `0.18`        | `0.13`           | `0.7.x`                 |
| `0.19`        | `0.13`           | `0.8.x`                 |
| `0.20`        | `0.13`           | `0.9.x`                 |
| `0.20`        | `0.13`           | `0.10.x`                |
| `0.22`        | `0.13`           | `0.11.x`                |
| `0.22`        | `0.15`           | `0.12.x`                |

### About us

Garde-actix-web is provided by [Netwo](https://www.netwo.io).

We use this crate for our internal needs and therefore are committed to its maintenance, however we cannot provide any
additional guaranty. Use it at your own risks.

While we won't invest in any feature we don't need, we are open to accept any pull request you might propose.

We are a France based full-remote company operating in the telecom sector. If you are interested in learning more, feel
free to visit [our career page](https://www.netwo.io/carriere).
