# ClashAI

ClashAI is an open-source, legal Clash of Clans companion. It does not automate gameplay, inspect the game process, inject input, scrape private surfaces, or reverse engineer the app. It uses the official Clash of Clans API, user-provided screenshots, configurable AI providers, and local-first progress tracking.

## Product

- Player profile lookup through the official Clash API
- Clan analytics with donations, war readiness, average town hall, and progress views
- Upgrade coach for heroes, troops, pets, equipment, and buildings
- Village screenshot analyzer architecture for OpenCV, ONNX Runtime, and future YOLO models
- AI attack and defense coach through OpenRouter or local Ollama
- Progress dashboard for long-running upgrade history
- GitHub OAuth-only authentication plan for hosted installs

## Repository layout

```text
frontend/   Static product UI and future React/Vite app surface
backend/    Rust Axum service with clean architecture boundaries
shared/     Cross-app contracts and schema notes
docs/       Architecture, API, deployment, roadmap, contribution guide
scripts/    Build and repository utilities
models/     Vision model metadata and training notes
.github/    CI and Pages deployment workflows
```

## Local build

```bash
node scripts/build-static.mjs
```

The current deployment artifact is dependency-free and emits a Cloudflare Worker-compatible `dist/server/index.js` plus static assets.

## Configuration

See `docs/environment.md`. API keys are never committed. Clash API, OpenRouter, Ollama, Turso, and GitHub OAuth values are runtime configuration.

## License

MIT
