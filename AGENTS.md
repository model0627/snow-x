# Repository Guidelines

## Architecture at a Glance
- `frontend (SvelteKit)` ⇄ `backend API (Rust/Axum)` ⇄ `PostgreSQL/Redis`, with `tasks (FastAPI + Celery)` handling async jobs and `markdown-service (Bun)` rendering markdown for both web and desktop clients.

## Project Structure & Module Organization
- Root services split by domain: `snow-x-backend` (Rust API plus Python tasks and Bun markdown service), `snow-x-ui` (SvelteKit frontend), and `snow-x-desktop` (Tauri shell). Shared assets live in `assets/`.
- Backend Rust code sits under `snow-x-backend/src`, with migrations in `migration/` and background jobs under `tasks/`. Python task APIs expose FastAPI routes in `tasks/app/api/routes`.
- UI views and components are inside `snow-x-ui/src` (`routes/`, `lib/components/`, `lib/stores/`), while the desktop shell uses `snow-x-desktop/src` alongside Tauri configuration in `src-tauri/`.

## Build, Test, and Development Commands
- Backend API: `cargo run` to serve on port 8000, `cargo test` for Rust unit/integration tests, `cargo fmt && cargo clippy` before submitting changes.
- Python tasks: from `snow-x-backend/tasks`, run `uv sync`, `uv run uvicorn app:app --reload --port 7000`, `uv run celery -A app.workers worker --loglevel=info`, and `uv run pytest`.
- Markdown service: `cd snow-x-backend/markdown-service && bun dev` for local development.
- Frontend: `cd snow-x-ui && pnpm dev`, with `pnpm build`, `pnpm check`, `pnpm fmt`, and `pnpm lint` as required.
- Desktop app: `cd snow-x-desktop && pnpm dev` for Tauri live reload or `pnpm build` for distributables.
- Full stack via Docker: from `snow-x-backend`, use `docker compose up -d` and `docker compose logs -f api` for service inspection.

## Coding Style & Naming Conventions
- Rust modules follow snake_case filenames and mirror directory hierarchy; run `cargo fmt` (rustfmt defaults) and keep functions small with explicit error types.
- Python code adheres to Ruff rules; prefer snake_case for modules/functions, PascalCase for classes, and define async tasks in `tasks/app/tasks/*`.
- Frontend and desktop TypeScript use 2-space indentation, Svelte component names in PascalCase, files in `lib/components` named `ComponentName.svelte`. Run `pnpm fmt` and `pnpm lint` (Prettier + Tailwind plugin) before committing. Keep i18n keys in `project.inlang` consistent with route names.

## Testing Guidelines
- Rust tests live beside implementation in `#[cfg(test)]` modules; integration tests should seed PostgreSQL via Docker containers.
- Python task tests rely on pytest—place new suites under `tasks/tests` (create per feature) and reuse existing fixtures for Redis/Postgres stubs.
- Frontend regressions are caught through `pnpm check`; add Vitest suites under `snow-x-ui/src` when introducing complex logic. Snapshot updates require reviewer approval.

## Commit & Pull Request Guidelines
- Follow the repository pattern of descriptive, sentence-style commit subjects that explain the feature or fix (e.g., “Improve policy execution flow by batching Celery jobs”). Use the imperative mood when possible and keep body text wrapped at 88 columns.
- Each PR should include: scope summary, linked issues (GH `Fixes #id`), testing notes (`cargo test`, `pnpm check`, etc.), and screenshots or terminal output when UI or CLI behavior changes.
- Ensure linting/formatting commands succeed, all services compile, and new migrations are documented in the PR description before requesting review.
