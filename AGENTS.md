# AGENTS.md

Context and instructions for AI coding agents

## Project Overview

A minimal starter template for building client-side web applications with Rust and Yew.

## General Guidelines

- Do not add dependencies unless explicitly requested.
- After any code change, run `cargo test --quiet` and fix any failures before considering the task complete.
- Keep this file lean and high-level: describe behaviors and constraints; avoid implementation details and code identifiers.

## Project Structure

- `src/`: application code, pages, components, and routing.
- `style/`: application stylesheets.
- `static/`: static assets.
- `docs/`: project documentation and study notes.
- `docker/`: production container and server configuration.
- `index.html`: Trunk entrypoint.

## Development Commands

- run: `trunk serve --open`
- test: `cargo test --quiet`
- lint: `cargo clippy --fix`
- build: `trunk build --release`
