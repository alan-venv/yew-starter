# Yew Starter Template
A base template to quickly start new web projects with **Rust + Yew**.

## Requirements
- Rust
- Trunk

Install Trunk:
```bash
cargo install trunk
```

## Run locally
```bash
trunk serve --open
```

## Production build
```bash
trunk build --release
```

## Current structure
- `src/main.rs`: routing setup with yew-router and app entrypoint.
- `src/pages/`: page modules.
- `src/components/`: shared reusable components.
- `style/reset.css`: global css reset.
- `style/shared.css`: shared page styles and animations.
- `static/`: static assets.
- `docs/`: project documentation and study notes.
- `index.html`: trunk entrypoint.
- `rules/*.md`: project rules and conventions.

## Suggested initial prompt
- `Give me a quick summary of the codebase`
