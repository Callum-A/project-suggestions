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
cp .env.example .env
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

```bash
# Fill in .env
cp .env.example .env
docker build -t project-suggestions .
docker compose up
```
