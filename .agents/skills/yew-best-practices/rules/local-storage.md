---
title: Storing Data in localStorage
---

## Storing Data in localStorage

Use `gloo_storage::LocalStorage` to read and write data in the browser's `localStorage`. Add `gloo-storage` to `Cargo.toml`. Values are serialized/deserialized with `serde` automatically.

**Example:**

```rust
use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;

#[component]
fn MyComponent() -> Html {
    let name = use_state(String::new);

    let on_save = {
        let name = name.clone();
        Callback::from(move |_: MouseEvent| {
            LocalStorage::set("name", &*name).unwrap();
        })
    };

    let on_load = {
        let name = name.clone();
        Callback::from(move |_: MouseEvent| {
            let result: Result<String, _> = LocalStorage::get("name");
            match result {
                Ok(value) => name.set(value),
                Err(_) => name.set("not found".to_string()),
            }
        })
    };

    let on_clear = Callback::from(move |_: MouseEvent| {
        LocalStorage::delete("name");
    });

    html! {
        <div>
            <button onclick={on_save}>{"Save"}</button>
            <button onclick={on_load}>{"Load"}</button>
            <button onclick={on_clear}>{"Clear"}</button>
            <p>{(*name).clone()}</p>
        </div>
    }
}
```

Reference: [gloo-storage docs](https://docs.rs/gloo-storage)
