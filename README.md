# rust-api

[![Tests](https://github.com/hope-inc/rust-api/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/hope-inc/rust-api/actions/workflows/test.yml)

RESTful API template built with Rust lang. It uses [MongoDB](https://docs.mongodb.com/)
database and [Axum](https://github.com/tokio-rs/axum) HTTP framework.

### Requirements
- [Rust](https://www.rust-lang.org/tools/install)
- [MongoDB](https://docs.mongodb.com/manual/installation/)

### Feature highlights
* Authentication. Based on [jsonwebtoken](https://github.com/Keats/jsonwebtoken)
* Layered configuration. Based on [config-rs](https://github.com/mehcode/config-rs)
* Logs. Based on [tracing](https://github.com/tokio-rs/tracing)
* Error handling
* Pagination
* E2E Tests
* OpenAPI Specification
* CI based on Github actions
* Dependabot configuration

### Project structure

```bash
├── Cargo.lock
├── Cargo.toml
├── README.md
├── config
│   ├── default.json    # Default configuration
│   ├── production.json # Production configuration (Overwrites the default)
│   └── test.json       # Test configuration (Overwrites the default)
├── rustfmt.toml
├── src
│   ├── database.rs
│   ├── errors.rs
│   ├── lib             # Helpers not related to the business model
│   │   ├── authenticate_request.rs
│   │   ├── date.rs
│   │   ├── mod.rs
│   │   ├── models.rs   # Base Database Model trait
│   │   ├── to_object_id.rs
│   │   └── token.rs
│   ├── logger.rs
│   ├── main.rs
│   ├── models
│   │   ├── cat.rs
│   │   ├── mod.rs
│   │   └── user.rs
│   ├── routes
│   │   ├── cat.rs
│   │   ├── mod.rs
│   │   ├── status.rs
│   │   └── user.rs
│   ├── settings.rs
│   └── tests           # E2E Tests
└── test.sh
```

### Test
To run tests make sure MongoDB is up and running.
```
$ cargo make test
```
