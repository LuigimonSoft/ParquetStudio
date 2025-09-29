# AGENTS Instructions

## Project Description
**Parquet Studio** is a cross-platform desktop application (macOS and Windows) for **visualizing, editing, and querying Parquet datasets**.  
It is inspired by **SQL Server Management Studio (SSMS)** in terms of layout and workflow, but designed with a **modern UI** similar to Vuexy, built entirely with **Leptos (Rust → WASM) and Tailwind UI**.  

### Key Capabilities
- Open and manage multiple **Parquet files** simultaneously.
- Inspect **schemas** in a sidebar with real-time navigation.
- Run **SQL-like queries** with syntax highlighting for reserved words.
- Display results in a **data grid** with interactive features.
- Save and restore workspace state (open files, active dataset).
- Integrate with the **Model Context Protocol (MCP)**:
  - Manage **prompts** for dataset analysis.
  - Register **datastores** exposed through MCP.
  - Configure **tools** for advanced processing.
- Store user preferences in JSON configs (`open_files.json` and `mcp_settings.json`).

### Architecture
- **Frontend (`src/`)**:  
  Built with Leptos + Tailwind. Contains routes, components, services, state management, and utilities.
- **Backend (`src-tauri/`)**:  
  Rust + Tauri service-repository pattern. Handles Parquet I/O, query execution, MCP server, and exposes commands to the frontend.
- **Config (`config/`)**:  
  JSON files for workspace state and MCP advanced configuration. Ignored by Git.
- **Tests (`tests/`)**:  
  - Unit tests: `tests/`  
  - Integration tests: `tests/integration/`  

### Scope
The initial release focuses on:
- Managing Parquet datasets locally.
- SQL query execution and result visualization.
- MCP integration for prompts, datastores, and tools.
- Cross-platform packaging with Tauri.  

Future extensions could include:
- Cloud storage integration.
- Multi-user collaboration.
- Export/import pipelines.

These instructions apply to the entire repository, including the **backend (Rust + Tauri)**, the **frontend (Leptos + Tailwind)**, and user-specific configuration handling.

## Project Structure
- **Frontend**:
  - Source code in `src/`.
  - `Cargo.toml` lives at the repository root.
  - Organized into:
    - `models/`: DTOs & ViewModels.
    - `services/`: Tauri commands / backend APIs.
    - `routes/`: app pages (home, settings, etc.).
    - `components/`: reusable UI (sidebar, query editor, datagrid, modal, etc.).
    - `state/`: signals & global stores.
    - `utils/`: constants, formatters, helpers.
- **Backend**:
  - Lives in `src-tauri/`.
  - Follows a **service-repository architecture**:
    - `models/`: DTOs and data structures.
    - `repositories/`: low-level data access (Parquet I/O).
    - `services/`: business logic.
    - `application/`: frontend entry points (Tauri commands).
    - `mcp_server/`: Model Context Protocol integration.
- **Tests**:
  - `tests/`: unit tests.
  - `tests/integration/`: integration tests.
- **Config**:
  - User-specific JSON files in `config/` (ignored by Git):
    - `open_files.json`: workspace state (open files, active file, basic MCP host/port).
    - `mcp_settings.json`: advanced MCP settings (prompts, datastores, tools).

## Style Guidelines
- Format Rust code with:
  ```bash
  cargo fmt --all
  ```

- Use **clear, descriptive names**.
- Keep functions **small and focused**.
- Apply **SOLID principles**:
  - **S**: Single Responsibility — one purpose per module.
  - **O**: Open for extension via traits.
  - **L**: Liskov — mocks and traits must be substitutable.
  - **I**: Interface Segregation — prefer smaller traits.
  - **D**: Dependency Inversion — depend on abstractions.
- Follow **Clean Code**:
  - Avoid “god modules”.
  - Prefer modularity and composability.
  - Ensure readability and maintainability.

## Testing
- **Backend**:
  - Run unit + integration tests:
    ```bash
    cargo test
    ```
  - Unit tests use mocks for repositories, services, and application.
  - Integration tests live in `tests/integration/`.
  - Minimum **85% coverage** required.
- **Frontend**:
  - Run unit + behavior tests with Chrome in headless mode:
    ```bash
    CHROMEDRIVER=$(which chromedriver) wasm-pack test --headless --chrome --test frontend
    ```
  - Coverage must be **≥ 85%**.
- **Naming Pattern**:
  - All test functions must follow:
    ```
    given_<condition>_when_<action>_then_<result>_should<Expectation>
    ```
  - Example:
    ```
    given_schema_lowercase_when_get_schema_then_should_return_uppercase
    ```

## Running the Application
- **Development mode** (hot reload + Tauri window):
  ```bash
  cargo tauri dev
  ```

- **Build production binaries** (DMG for macOS, MSI/EXE for Windows):
  ```bash
  cargo tauri build
  ```

## Configuration Files
- `open_files.json` → stores open files, active dataset, and basic MCP settings (enabled, host, port).
- `mcp_settings.json` → stores MCP prompts, datastores, and tools.
- Both must be written in **pretty JSON** and loaded with safe defaults if missing/corrupted.
- They are user-specific and excluded from version control.

## Commit Etiquette
All commits must follow the **Conventional Commits** specification:

- **Types**:
  - `feat`: a new feature.
  - `fix`: a bug fix.
  - `docs`: documentation changes.
  - `style`: code style (formatting, missing semicolons, etc.) with no code meaning change.
  - `refactor`: code change that neither fixes a bug nor adds a feature.
  - `perf`: performance improvement.
  - `test`: adding or correcting tests.
  - `build`: changes to the build system or dependencies.
  - `ci`: changes to CI configuration.
  - `chore`: other changes that don’t modify src or test files.
- **Format**:
  ```
  <type>(<scope>): <short summary>
  ```

- Scope is optional but recommended (`frontend`, `backend`, `config`, etc.).
- **Examples**:
- ✅ `feat(frontend): add query editor with SQL syntax highlighting`
- ✅ `fix(backend): correct parquet reader buffer overflow`
- ✅ `test(config): add unit test for MCP settings loader`

Following Conventional Commits ensures clear history and supports semantic versioning.

## Platform
- Must run on **macOS** and **Windows**.
- Backend packaged with **Tauri bundler**.
- Frontend compiled to **WASM** with Leptos.