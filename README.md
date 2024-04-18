# Project Suggestions

This repo contains the backend API for a web application where users can login via Google OAuth create project suggestions
with a list of tags for discoverability. These project suggestions will then be displayed via a web UI.

## UI Repo

UI repo not yet public.

## Usage

### Without Docker

```bash
# Fill in .env
cp .env.example .env
cargo run
```

### Docker

```bash
# Fill in .env
cp .env.example .env
docker build -t project-suggestions .
docker compose up
```
