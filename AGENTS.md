# AGENTS Instructions

## Scope
These instructions apply to the entire repository, covering both the **backend (Rust + Tauri)** and **frontend (Leptos + Tailwind)**, as well as user configuration handling.

## Project Structure
- **Backend**:  
  - Lives under `backend/` (mapped to `src-tauri/`).  
  - Uses a **service-repository architecture** with a clear separation:  
    - `models/`: DTOs and data structures.  
    - `repositories/`: low-level data access (Parquet I/O).  
    - `services/`: business logic.  
    - `application/`: frontend entry points (Tauri commands).  
    - `mcp_server/`: Model Context Protocol integration.  
- **Frontend**:  
  - Lives under `frontend/` using **Leptos** (Rust → WASM) with **Tailwind UI**.  
  - Organized into:  
    - `models/`: DTOs & ViewModels.  
    - `services/`: backend API/Tauri calls.  
    - `routes/`: app pages (home, settings, etc.).  
    - `components`: reusable UI (sidebar, query editor, datagrid, modal, etc.).  
    - `state/`: global app signals/stores.  
    - `utils/`: constants, helpers.  
    - `tests/`: unit tests (`wasm-bindgen-test`) + E2E (Playwright/Cypress).  
- **Config**:  
  - User-specific configs live in `config/` and are **ignored by Git**:  
    - `open_files.json`: workspace state (open files, active file, basic MCP host/port).  
    - `mcp_settings.json`: advanced MCP configuration (prompts, datastores, tools).  

## Style Guidelines
- **Rust (backend)**:  
  - Format with `cargo fmt --all`.  
  - Keep functions small and focused.  
  - Follow **SOLID principles**:  
    - Single Responsibility → each module has one purpose.  
    - Open/Closed → extend via traits.  
    - Liskov → traits & mocks must be substitutable.  
    - Interface Segregation → small, precise traits.  
    - Dependency Inversion → depend on abstractions.  
  - Use **Clean Code**: descriptive names, modular files, no "god modules".  
- **Frontend (Leptos + Tailwind)**:  
  - Tailwind classes for all styling.  
  - Components must be **stateless** whenever possible.  
  - Organize by responsibility (layout, editor, grid, modal).  
  - Reuse constants and helpers from `utils/`.  
- **Configuration**:  
  - Store user configs as **pretty-printed JSON** (`serde_json::to_string_pretty`).  
  - Provide safe defaults if files are missing or corrupted.  
  - Keep workspace (`open_files.json`) separate from advanced MCP settings (`mcp_settings.json`).  

## Testing
- **Backend**:  
  - Unit tests with mocks for repositories, services, and application layers.  
  - Integration tests with real Parquet files under `tests/`.  
  - Coverage must be **≥ 85%**.  
- **Frontend**:  
  - Unit tests with `wasm-bindgen-test`.  
  - E2E/UI behavior tests with Playwright or Cypress.  
  - Coverage must be **≥ 85%**.  
- **Naming Pattern**:  
  - Tests must use the format:  
    ```
    given_<condition>_when_<action>_then_<result>_should<Expectation>
    ```  
  - Example:  
    `given_schema_lowercase_when_get_schema_then_should_return_uppercase`  

Always run the full test suite before committing:
```bash
cargo test
```

## Commit Etiquette
Use concise commit messages written in the imperative mood (e.g., "Add feature" rather than "Added feature").

