# AGENTS

## TL;DR
- This is a basic Yew starter template for quickly bootstrapping new ideas.

## Guidelines
- Do not add new dependencies unless explicitly requested.
- After any code change, run `cargo test` and fix any build or test errors before considering the work done.
- Keep `AGENTS.md` lean and summarized: describe behavior/actions at a high level, and avoid references to code identifiers.

## Project Structure
- `src/main.rs`: routing setup with yew-router and app entrypoint.
- `src/pages/`: page modules.
- `src/components/`: shared reusable components.
- `style/reset.css`: global css reset.
- `style/shared.css`: shared page styles and animations.
- `static/`: static assets.
- `docs/`: project documentation and study notes.
- `index.html`: trunk entrypoint.
