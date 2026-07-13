# API Documentation

Base path: `/`

## GET /health
Returns `ok` for health checks.

## GET /player/{tag}
Looks up a player by tag. Production implementations call the official Clash of Clans API with `CLASH_API_TOKEN`.

## GET /clan/{tag}
Planned clan summary endpoint for members, donations, war status, average town hall, and progress metrics.

## GET /war/{tag}
Planned official clan war endpoint wrapper. It must respect Clash API availability and clan privacy settings.

## GET /coach/{tag}
Returns upgrade recommendations from normalized profile data.

## AI Provider Contract

Providers implement the same application interface:

- `OPENROUTER_API_KEY` for hosted models
- `OLLAMA_BASE_URL` for local models
- prompt templates must include legal-use constraints
