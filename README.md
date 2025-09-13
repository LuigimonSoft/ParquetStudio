# Parquet Studio

Parquet Studio is a cross-platform desktop application for exploring and querying Parquet datasets. It combines a Rust backend with a Leptos/Tailwind frontend and includes an optional MCP server for exposing datasets to AI tooling. It also lets you define new Parquet files by specifying a schema and saving the file to disk.

## Structure
- `backend/` – service-repository backend written in Rust.
- `frontend/` – Leptos/Tailwind frontend (compiled to WASM).
- `config/` – user-specific configuration files (ignored by git).
- Native menu bar with File, Create, and MCP sections for common actions.

## Development
Format code with `cargo fmt --all` and run tests with `cargo test`.
