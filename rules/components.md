## Creating Function Components

To create a function component add the `#[component]` attribute to a function. By convention, the function is named in PascalCase, like all components, to contrast its use to normal html elements inside the html! macro.

**Example:**

```rust
#[component]
fn HelloWorld() -> Html {
    html! { "Hello world" }
}
```

Reference: [Yew Function Components](https://yew.rs/docs/concepts/function-components)
