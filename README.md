# Project Suggestions

This repo contains the backend API for a web application where users can login via Google OAuth create project suggestions
with a list of tags for discoverability. These project suggestions will then be displayed via a web UI.

## UI

You can find the UI code under the `ui` directory.

## Usage

### Without Docker

#### Backend

```bash
# Fill in .env
cp .env.example.local .env
cargo run
```

#### Frontend

```bash
cd ui
# Fill in .env
cp .env.example .env
npm start
```

### Docker

### Local React Server

```bash
# Fill in .env
cp .env.example.local .env
docker build -t project-suggestions .
docker compose -f compose.local.yml up
```

### Production

```bash
# Fill in .env
cp .env.example.production .env.production
docker build -t project-suggestions .
docker compose -f compose.production.yml up
```
