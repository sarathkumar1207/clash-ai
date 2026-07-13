# Architecture

```mermaid
flowchart LR
  UI[React dashboard] --> API[Rust Axum API]
  API --> Clash[Official Clash of Clans API]
  API --> Turso[(Turso SQLite)]
  API --> AI{AI provider}
  AI --> OpenRouter[OpenRouter]
  AI --> Ollama[Local Ollama]
  API --> Vision[OpenCV + ONNX Runtime]
  Vision --> Grid[Internal village grid]
```

ClashAI uses clean architecture:

- Presentation: React views and Axum routes.
- Application: upgrade ranking, clan aggregation, progress workflows, AI orchestration.
- Domain: player, clan, upgrade, screenshot, and recommendation models.
- Infrastructure: Clash API, Turso, GitHub OAuth, OpenRouter, Ollama, OpenCV, ONNX Runtime.

Legal boundary: ClashAI never controls, scrapes, or instruments the Clash of Clans client.
