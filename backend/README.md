# Backend Service

This crate exposes an HTTP API for managing tournament data. It requires a signing key for issuing JSON Web Tokens.

## Environment

Set `JWT_SECRET` to your desired secret key before running the service:

```bash
export JWT_SECRET=mysecret
cargo run -p backend --bin backend
```

If unset, the server defaults to `secret`.

## Authentication flow

Call `POST /platform/login` or `POST /dashboard/login` with valid credentials. The response body contains a `token` field:

```json
{ "token": "<jwt>" }
```

Include this token in the `Authorization` header when calling other endpoints:

```
Authorization: Bearer <jwt>
```
