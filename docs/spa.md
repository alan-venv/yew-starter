# Single Page Application (SPA)

A SPA is a web application that loads a single HTML page and updates its content dynamically through JavaScript (or WebAssembly), without reloading the entire page on every navigation. The browser becomes the application's runtime, while the server is reduced to acting mainly as a data API.

In the traditional model, a Multi Page Application (MPA), every user interaction triggers a full HTTP cycle: a request to the server, HTML rendering on the backend, and a response containing an entirely new page. In a SPA, the server delivers the application once, and all subsequent navigation happens on the client side.

## What the SPA model makes possible

- **Persistent state across views** -- a music player that keeps playing while the user navigates (Spotify Web), or a chat widget that stays open on any page (Intercom).
- **Real-time updates** -- dashboards using WebSockets to refresh data without a page reload (Grafana, collaborative Figma).
- **Offline interactions** -- with Service Workers, the application can work without a connection and sync later (Google Docs, Notion).
- **Continuous transitions and animations** -- dragging items between lists (Trello), reordering elements with drag-and-drop, or using interactive canvases.
- **Granular lazy loading** -- loading modules on demand as the user navigates, instead of shipping everything upfront.

## SPA vs MPA

| Aspect                | SPA                        | Traditional MPA           |
|-----------------------|----------------------------|---------------------------|
| Navigation            | Client-side (History API)  | Server-side (full reload) |
| State                 | Persists across routes     | Lost on every request     |
| Server                | Data API                   | Renders HTML              |
| Initial load          | Heavier                    | Lighter                   |
| Subsequent navigation | Instant                    | New HTTP cycle            |
| SEO                   | Requires SSR/prerendering  | Native                    |
| Offline               | Possible (Service Workers) | Not practical             |

## WASM as a differentiator

WebAssembly adds another layer to the SPA model: near-native performance in the browser. Applications that once required desktop environments can now run in the browser with practical performance, including image editors, CAD tools, and heavy data-processing workflows. Frameworks like Yew make it possible to build SPAs in Rust compiled to WASM, combining memory safety with strong performance.
