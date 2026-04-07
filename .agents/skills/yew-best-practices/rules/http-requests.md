---
title: Making HTTP Requests
---

## Making HTTP Requests

Use `gloo_net::http::Request` to make HTTP requests from Yew components. Since all requests are async, they must run inside `spawn_local` — see the `async-actions` rule for details.

Use `.json(&body)` to serialize a request body and `.json::<T>().await` to deserialize a response. Add `gloo-net` with the `http` feature and `serde` with `derive` to `Cargo.toml`.

**Example:**

```rust
use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(Deserialize)]
struct Book {
    title: String,
}

#[component]
fn MyComponent() -> Html {
    let title = use_state(|| "Loading...".to_string());

    {
        let title = title.clone();
        use_effect_with((), move |_| {
            spawn_local(async move {
                let result = Request::get("https://example.com/book/1")
                    .send()
                    .await;

                match result {
                    Ok(response) if response.ok() => {
                        let book = response.json::<Book>().await.unwrap();
                        title.set(book.title);
                    }
                    _ => {
                        title.set("not found".to_string());
                    }
                }
            });
            || ()
        });
    }

    html! {
        <p>{(*title).clone()}</p>
    }
}
```

Reference: [gloo-net docs](https://docs.rs/gloo-net)
