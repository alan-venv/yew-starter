---
title: Async Actions
---

## Async Actions

Yew component functions are synchronous. To run async code, use `spawn_local` from `wasm_bindgen_futures` — it spawns a `Future` on the browser's microtask queue without blocking the UI.

Reference: [wasm-bindgen-futures docs](https://docs.rs/wasm-bindgen-futures)

### On component mount (use_effect_with)

To run async code when a component mounts, combine `use_effect_with` with `spawn_local`. Use a `{}` block with shadowed clones before the hook call — this is the Yew community convention to avoid naming conflicts.

**Example** — show a warning message 10 seconds after the component mounts:

```rust
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[component]
fn MyPage() -> Html {
    let warning = use_state(|| Option::<String>::None);

    {
        let warning = warning.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                TimeoutFuture::new(10_000).await;
                warning.set(Some("Session is about to expire.".to_string()));
            });
            || ()
        });
    }

    html! {
        <div>
            if let Some(msg) = (*warning).clone() {
                <p class="warning">{msg}</p>
            }
        </div>
    }
}
```

### Inside event callbacks

To run async code in response to a user action, use `spawn_local` inside a `Callback`. The outer closure captures state handles by move, so the async block needs its own clones:

**Example** — delay a confirmation message by 2 seconds after a button click:

```rust
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[component]
fn MyComponent() -> Html {
    let message = use_state(|| Option::<String>::None);

    let on_click = {
        let message = message.clone();
        Callback::from(move |_: MouseEvent| {
            let message = message.clone();
            spawn_local(async move {
                TimeoutFuture::new(2_000).await;
                message.set(Some("Action confirmed!".to_string()));
            });
        })
    };

    html! {
        <div>
            <button onclick={on_click}>{"Confirm"}</button>
            if let Some(msg) = (*message).clone() {
                <p>{msg}</p>
            }
        </div>
    }
}
```
