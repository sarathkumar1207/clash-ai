# Deployment Guide

## Frontend

The static frontend can be hosted for free on GitHub Pages. The included workflow builds `dist/` and publishes it.

## Backend

The Rust backend is designed for free-tier services that support long-running Rust HTTP servers:

- Render free web service where available
- Fly.io free allowance where available
- Railway or Shuttle when their free tiers fit the project

Cloudflare Workers are suitable for the static product surface. A native Rust Axum server should use a container host unless it is compiled to a Worker-compatible target in a future phase.

## Runtime values

Never commit secrets. Configure all keys through the host dashboard.
