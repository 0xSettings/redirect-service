# SHORTEN URL + REDIRECTION SERVICE 

A URL shortening service built with Rust using Axum, SQLx, and PostgreSQL. Generates unique 6-character alphanumeric short keys, persists mappings to a relational database, and issues HTTP 301 redirects. Idempotent by design — submitting the same long URL always returns the same short key.

---

## Table of Contents

- [Architecture](#architecture)
- [Project Structure](#project-structure)
- [Prerequisites](#prerequisites)
- [Getting Started](#getting-started)
- [Environment Variables](#environment-variables)
- [Database](#database)
- [API Reference](#api-reference)
- [Design Decisions](#design-decisions)
- [Error Handling](#error-handling)
- [Development](#development)
- [Future Improvements](#future-improvements)

---

## Architecture

The codebase follows a strict layered architecture. Each layer only knows about the layer directly beneath it — never above it, and never skipping layers.

```
┌─────────────────────────────────────┐
│           HTTP Client               │
└────────────────┬────────────────────┘
                 │ HTTP Request
┌────────────────▼────────────────────┐
│         api/handler.rs              │  Axum extractors, HTTP status codes,
│    (Presentation Layer)             │  JSON serialization, redirects
└────────────────┬────────────────────┘
                 │ Domain types
┌────────────────▼────────────────────┐
│       service/url_service.rs        │  Business logic, key generation,
│        (Service Layer)              │  idempotency, collision handling
└────────────────┬────────────────────┘
                 │ Trait calls
┌────────────────▼────────────────────┐
│   repository/url_repository.rs      │  SQL queries, database mapping,
│      (Repository Layer)             │  PostgreSQL implementation
└────────────────┬────────────────────┘
                 │ SQL
┌────────────────▼────────────────────┐
│            PostgreSQL               │
└─────────────────────────────────────┘
```

### Key Design Principles

**Newtype Pattern** — `ShortKey` wraps a `String` but validation always runs through `ShortKey::new()`. It is impossible to construct an invalid `ShortKey` at compile time — the type system enforces the invariant.

**Trait-based Repository** — `UrlRepository` is a trait, not a concrete type. `UrlService` depends on the trait, not on `PostgresUrlRepository` directly. This means the entire database layer can be swapped for an in-memory fake in tests without touching service code.

**Idempotency** — `POST /shorten` with the same URL always returns the same short key. The service checks for an existing mapping before generating a new one, leveraging the `UNIQUE` constraint on `original_url` in the database.

**Collision-safe key generation** — Short keys are randomly generated and checked for existence before use. In the astronomically unlikely event of a collision (62⁶ = ~56 billion combinations), the loop retries automatically.

---

## Project Structure

```
redirect-service/
├── src/
│   ├── main.rs                      # Entry point — wires layers, starts server
│   ├── api/
│   │   ├── mod.rs
│   │   └── handler.rs               # HTTP handlers (POST /shorten, GET /{key})
│   ├── model/
│   │   ├── mod.rs                   # Re-exports all model types
│   │   ├── errors.rs                # DomainError enum
│   │   ├── short_key.rs             # ShortKey newtype with validation
│   │   └── url.rs                   # Url (internal) and UrlResponse (external)
│   ├── repository/
│   │   ├── mod.rs
│   │   └── url_repository.rs        # UrlRepository trait + PostgresUrlRepository
│   └── service/
│       ├── mod.rs
│       └── url_service.rs           # Business logic, key generation
├── migrations/
│   └── 0001_create_urls.sql         # Creates the urls table
├── .env                             # Local environment variables (not committed)
├── docker-compose.yml               # Local PostgreSQL via Docker
└── Cargo.toml                       # Dependencies
```

---

## Prerequisites

| Tool | Version | Purpose |
|------|---------|---------|
| Rust | 1.75+ | Language toolchain |
| Cargo | latest | Build system |
| PostgreSQL | 15+ | Database |
| sqlx-cli | latest | Migrations |
| Docker (optional) | latest | Local PostgreSQL |

Install sqlx-cli:
```bash
cargo install sqlx-cli --no-default-features --features postgres
```

---

## Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/0xSettings/redirect-service
cd redirect-service
```

### 2. Configure environment

Create a `.env` file in the project root:
```env
DATABASE_URL=postgres://postgres:your_password@localhost/redirect
```

### 3. Start PostgreSQL

**Via Docker:**
```bash
docker-compose up -d
```

**Via local install:** ensure PostgreSQL is running and accessible at the `DATABASE_URL` above.

### 4. Run database migrations

```bash
cargo sqlx database create
cargo sqlx migrate run
```

This creates the `redirect` database and the `urls` table.

### 5. Build and run

```bash
cargo run
```

The server starts on `http://localhost:8080`.

---

## Environment Variables

| Variable | Required | Example | Description |
|----------|----------|---------|-------------|
| `DATABASE_URL` | Yes | `postgres://postgres:password@localhost/redirect` | PostgreSQL connection string |

> **Note:** Never commit your `.env` file. It is listed in `.gitignore`.

---

## Database

### Schema

```sql
CREATE TABLE IF NOT EXISTS urls (
    short_key    VARCHAR(20)  PRIMARY KEY,
    original_url TEXT         NOT NULL UNIQUE,
    created_at   TIMESTAMP    NOT NULL DEFAULT NOW()
);
```

- `short_key` — primary key, the 6-character alphanumeric code
- `original_url` — the full original URL, unique-constrained to enforce idempotency
- `created_at` — UTC timestamp of creation

### Migrations

Migrations live in `./migrations/` and are run automatically on server startup via `sqlx::migrate!()`. To run them manually:

```bash
cargo sqlx migrate run
```

To revert:
```bash
cargo sqlx migrate revert
```

---

## API Reference

### `POST /shorten`

Shortens a URL. If the URL has been shortened before, returns the existing short key.

**Request**

```http
POST /shorten
Content-Type: application/json

{
  "url": "https://www.example.com/very/long/path?with=query&params=true"
}
```

**Response — 201 Created**

```json
{
  "shortUrl": "http://localhost:8080/hP6iBd",
  "originalUrl": "https://www.example.com/very/long/path?with=query&params=true",
  "createdAt": "2026-02-15T00:37:28.120922Z"
}
```

**Response — 500 Internal Server Error**

```json
"Database error: ..."
```

---

### `GET /{short_key}`

Resolves a short key and redirects to the original URL.

**Request**

```http
GET /hP6iBd
```

**Response — 301 Moved Permanently**

```http
HTTP/1.1 301 Moved Permanently
Location: https://www.example.com/very/long/path?with=query&params=true
```

**Response — 400 Bad Request**

Returned when the key is fewer than 6 characters or contains non-alphanumeric characters.

**Response — 404 Not Found**

Returned when the key does not exist in the database.

---

### Example — PowerShell

```powershell
# Shorten a URL
$response = Invoke-RestMethod `
  -Uri "http://localhost:8080/shorten" `
  -Method POST `
  -ContentType "application/json" `
  -Body '{"url": "https://doc.rust-lang.org/book/ch21-02-multithreaded.html"}' `
  -UseBasicParsing

$response

# Output:
# shortKey                     originalUrl                              createdAt
# --------                     -----------                              ---------
# http://localhost:8080/hP6iBd https://doc.rust-lang.org/book/...      2026-02-15T00:37:28Z
```

### Example — curl (Linux/macOS)

```bash
# Shorten a URL
curl -X POST http://localhost:8080/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://doc.rust-lang.org/book/ch21-02-multithreaded.html"}'

# Follow the redirect
curl -L http://localhost:8080/hP6iBd
```

---

## Design Decisions

### Why `Box<dyn UrlRepository>` instead of generics?

Using a generic type parameter on `UrlService` (`UrlService<R: UrlRepository>`) would work but propagates the type parameter everywhere — through `AppState`, into every handler, and throughout `main.rs`. `Box<dyn UrlRepository>` pays a small runtime cost for dynamic dispatch but keeps the rest of the codebase free of generic noise. For a service with I/O-bound database calls, the dynamic dispatch overhead is completely negligible.

### Why `ShortKey` as a newtype instead of `String`?

Passing raw `String` values for short keys means any string can be passed anywhere — no validation guarantee. The newtype pattern makes invalid states unrepresentable. Once a `ShortKey` exists, you know it passed validation. Functions that accept `&ShortKey` cannot accidentally receive an unvalidated string.

### Why check for duplicates before insert instead of `ON CONFLICT`?

The `UNIQUE` constraint on `original_url` would catch duplicates at the database level, but the error would need to be caught and handled as a special case. The explicit `find_by_original_url` check before insert makes the idempotency logic explicit, readable, and testable.

### Why 301 instead of 302?

`301 Moved Permanently` is semantically correct — the short URL always resolves to the same original URL. It also allows browsers and CDNs to cache the redirect, reducing load on the service for frequently used links. Use `302 Found` if you need the flexibility to change where a short URL points later.

---

## Error Handling

All errors flow through `DomainError`:

```rust
pub enum DomainError {
    UrlNotFound,          // Short key does not exist in database
    InvalidShortKey,      // Key failed validation (< 6 chars or non-alphanumeric)
    DatabaseError(sqlx::Error),  // Any database-level failure
}
```

`DatabaseError` uses `#[from] sqlx::Error` so the `?` operator automatically converts any `sqlx::Error` into a `DomainError::DatabaseError` without manual `.map_err()` calls throughout the codebase.

The handler layer maps domain errors to HTTP status codes:

| DomainError | HTTP Status |
|-------------|-------------|
| `UrlNotFound` | 404 Not Found |
| `InvalidShortKey` | 400 Bad Request |
| `DatabaseError` | 500 Internal Server Error |

---

## Development

### Check for compile errors

```bash
cargo check
```

### Run with live reloading

```bash
cargo install cargo-watch
cargo watch -x run
```

### View database contents

```bash
psql -U postgres -d redirect -c "SELECT * FROM urls;"
```

### Reset the database

```bash
cargo sqlx database drop
cargo sqlx database create
cargo sqlx migrate run
```

---

## Future Improvements

- **Custom short keys** — allow users to specify their own alias instead of a random key
- **Expiry** — add a `expires_at` column and reject expired keys at resolve time
- **Click tracking** — record each redirect hit with timestamp and user agent
- **Rate limiting** — prevent abuse of the `POST /shorten` endpoint
- **Authentication** — API key or JWT to scope short URLs to users
- **Admin API** — endpoints to list, update, and delete short URLs
- **Metrics** — expose a `/metrics` endpoint for Prometheus scraping
- **Integration tests** — use `sqlx::test` with a real database per test
- **Configuration** — replace hardcoded `http://localhost:8080` with a config struct loaded from environment

---

## License

MIT