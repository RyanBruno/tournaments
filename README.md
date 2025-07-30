# Tournaments Workspace

This repository is a Cargo workspace containing three Rust crates.

## Workspace layout

```
backend/   - backend server and persistence layer
frontend/  - web UI built with Dioxus
models/    - shared data models
log4rs.yml - logging configuration
```

See `backend/README.md` for backend usage and authentication details.

The backend stores its data under the `data/` directory (ignored in Git). It
creates the following sub directories at runtime:

```
 data/
 ├── dashboard/
 │   ├── transactions/
 │   ├── snapshots/
 │   └── events/
 └── platform/
     ├── transactions/
     ├── snapshots/
     └── events/
```

Logs are written to the `log/` directory according to `log4rs.yml`.
Both `data/` and `log/` are listed in `.gitignore` and are created when the
applications run.

## Building

Both crates are configured as binaries inside the workspace. Use Cargo to build
and run them:

```bash
# Run the backend API (requires the nightly toolchain)
cargo run -p backend --bin backend

# Run the frontend application
cargo run -p frontend --bin frontend
```

The backend uses unstable Rust features, so ensure you have the nightly toolchain
installed (e.g. `rustup default nightly`).

## Running the frontend

The web UI uses Dioxus and needs a few prerequisites for development:

```bash
cargo install dioxus-cli
```

Start the application with:

```bash
dx serve --package frontend
```

The frontend communicates with the backend over HTTP at `http://localhost:8080`.

## Authentication

The backend uses JSON Web Tokens (JWT) for all API requests. Set the `JWT_SECRET` environment variable to configure the signing key; it defaults to `secret` if unset.

`POST /platform/login` and `POST /dashboard/login` now return a JSON object containing a `token` field. Use this token in subsequent requests by including an `Authorization: Bearer <token>` header.
