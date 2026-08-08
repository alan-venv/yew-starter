---
title: Handling Events
---

## Handling Events

To read values from DOM input elements inside event callbacks, use the `TargetCast` trait, which is built into Yew and re-exported via `yew::prelude::*`.

Add `web-sys` to `Cargo.toml` with the `HtmlInputElement` feature. This is required because Rust does not allow using transitive crates directly in code, even if they are already pulled in by Yew.

Import `HtmlInputElement` from `web_sys` and call `target_unchecked_into` on the event. This method comes from `TargetCast`, which is already in scope via `yew::prelude::*`. For form submission, use `SubmitEvent` and call `prevent_default` to avoid page reload.

**Example:**

```rust
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[component]
fn MyComponent() -> Html {
    let value = use_state(String::new);

    let handle_input = {
        let value = value.clone();
        Callback::from(move |e: InputEvent| {
            let input = e.target_unchecked_into::<HtmlInputElement>();
            value.set(input.value());
        })
    };

    let handle_submit = {
        let value = value.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            // use *value
        })
    };

    html! {
        <form onsubmit={handle_submit}>
            <input type="text" oninput={handle_input} value={(*value).clone()} />
        </form>
    }
}
```

Reference: [Yew Events](https://yew.rs/docs/concepts/html/events#using-targetcast)
