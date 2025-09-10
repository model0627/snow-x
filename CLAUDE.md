# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Mofumofu is a modern social blogging platform with a microservices architecture, emphasizing authentic content creation without engagement metrics. The platform features native desktop support, rich markdown rendering, and multi-language support.

## Tech Stack

- **Frontend**: SvelteKit 5, TypeScript, TailwindCSS 4, Paraglide i18n
- **Desktop**: Tauri + Svelte
- **Main Backend**: Rust (Axum, SeaORM, Tower)
- **Tasks API**: Python (FastAPI, Celery, SQLAlchemy)
- **Markdown Service**: Bun + Elysia
- **Database**: PostgreSQL 17
- **Cache/Queue**: Redis 8
- **Search**: Meilisearch
- **Storage**: Cloudflare R2

## Essential Commands

### Frontend Development (mofumofu-ui)
```bash
cd mofumofu-ui
pnpm dev        # Start development server (port 5173)
pnpm build      # Production build
pnpm check      # TypeScript + Svelte type checking
pnpm fmt        # Format code with Prettier
pnpm lint       # Lint code
```

### Desktop Development (mofumofu-desktop)
```bash
cd mofumofu-desktop
pnpm dev        # Start Tauri development
pnpm build      # Build desktop application
```

### Backend Development (mofumofu-backend)

**Rust API:**
```bash
cd mofumofu-backend
cargo run       # Run main API server (port 8000)
cargo test      # Run all tests
cargo fmt       # Format code
cargo clippy    # Lint code
```

**Python Tasks API:**
```bash
cd mofumofu-backend/tasks
uv sync         # Install dependencies
uv run uvicorn app:app --port 7000 --reload  # Development server
uv run celery -A app.workers worker --loglevel=info  # Start Celery worker
uv run pytest   # Run tests
uv run ruff check  # Lint code
uv run ruff format  # Format code
```

**Markdown Service:**
```bash
cd mofumofu-backend/markdown-service
bun dev         # Development server (port 6700)
bun start       # Production server
```

### Docker Development (Recommended)
```bash
# Start all services
docker compose up -d

# View logs
docker compose logs -f [service-name]

# Rebuild specific service
docker compose up -d --build [service-name]

# Reset database
docker compose down -v
docker compose up -d
```

## Architecture

### Service Ports
- Frontend: 5173
- Main API: 8000
- Tasks API: 7000
- Markdown Service: 6700
- PostgreSQL: 5432
- Redis: 6379
- Meilisearch: 7700

### Key Directory Structure

```
mofumofu-backend/
├── src/
│   ├── api/v0/routes/   # REST endpoints (auth, posts, users, images, etc.)
│   ├── service/          # Business logic layer
│   ├── entity/           # SeaORM database models
│   ├── middleware/       # Auth, CORS, logging middleware
│   └── utils/            # Shared utilities
├── tasks/                # Python background tasks
│   ├── app/workers/      # Celery task definitions
│   └── app/models/       # SQLAlchemy models
└── markdown-service/     # Bun markdown processor

mofumofu-ui/
├── src/
│   ├── routes/           # SvelteKit pages and API routes
│   ├── lib/
│   │   ├── components/   # Reusable Svelte components
│   │   ├── stores/       # Global state management
│   │   └── utils/        # Frontend utilities
│   └── paraglide/        # i18n translations
```

### Database Schema

Core entities managed by SeaORM:
- **users**: User accounts with OAuth support
- **posts**: Blog posts with markdown content
- **comments**: Hierarchical comment system
- **images**: Image metadata and CDN URLs
- **tags**: Tag system for posts
- **follows**: User following relationships

### Authentication Flow

1. JWT-based authentication with refresh tokens
2. OAuth providers: Google, GitHub
3. Account linking for multiple OAuth providers
4. Session management via Redis

### Background Tasks

Celery workers handle:
- Image processing (resize, WebP conversion)
- Search indexing (Meilisearch)
- Email notifications
- Post view counting
- Cache invalidation

## Development Guidelines

### API Conventions

- REST API version: `/api/v0/`
- Authentication: Bearer token in Authorization header
- Response format: JSON with consistent error structure
- Pagination: `page` and `limit` query parameters

### Testing

- Rust: Write tests in `#[cfg(test)]` modules
- Python: Use pytest fixtures for database setup
- Integration: Test against Docker services

### Environment Variables

Required `.env` file in root:
```
DATABASE_URL=postgresql://user:password@localhost:5432/mofumofu
REDIS_URL=redis://localhost:6379
MEILISEARCH_URL=http://localhost:7700
R2_BUCKET_NAME=your-bucket
JWT_SECRET=your-secret
```

### Common Tasks

**Adding a new API endpoint (Rust):**
1. Create route handler in `src/api/v0/routes/`
2. Add business logic in `src/service/`
3. Update router in `src/api/v0/mod.rs`
4. Add tests in same file

**Adding a new background task (Python):**
1. Create task in `tasks/app/workers/`
2. Register in Celery app
3. Call from Rust API using Redis queue

**Modifying database schema:**
1. Update SeaORM entities
2. Create migration with `sea-orm-cli migrate generate`
3. Run migration with `sea-orm-cli migrate up`

## Important Notes

- Always run `cargo fmt` and `cargo clippy` before committing Rust code
- Use `ruff` for Python formatting and linting
- Frontend uses Prettier with TailwindCSS plugin
- Docker Compose is the recommended development environment
- Production uses separate docker-compose.prod.yml
- Meilisearch requires initialization on first run