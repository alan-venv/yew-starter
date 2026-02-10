## Creating Properties

Properties ("Props") are component arguments that Yew watches for changes during re-rendering. To define props for function components, create a struct with `#[derive(Properties, PartialEq)]` and receive it by reference in the component function.

**Example:**

```rust
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct GreetingProps {
    name: AttrValue,
    #[prop_or_default]
    is_loading: bool,
}

#[component]
fn Greeting(props: &GreetingProps) -> Html {
    if props.is_loading {
        html! { <p>{"Loading..."}</p> }
    } else {
        html! { <p>{format!("Hello {}", props.name)}</p> }
    }
}

#[component]
fn App() -> Html {
    html! { <Greeting name="Sam" /> }
}
```

Reference: [Yew Properties](https://yew.rs/docs/concepts/function-components/properties)
