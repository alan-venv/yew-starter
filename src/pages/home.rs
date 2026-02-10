use yew::prelude::*;

use crate::components::link::AppLink;
use crate::Route;

#[component]
pub fn HomePage() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| counter.set(*counter + 1)
    };

    html! {
        <main class="home page-transition">
            <section class="hero">
                <p class="tag">{"Yew Starter Template"}</p>
                <div class="counter">
                    <p class="counter-label">{"Counter"}</p>
                    <p class="counter-value">{*counter}</p>
                    <button class="btn btn-ghost" onclick={onclick}>{"+1"}</button>
                </div>
                <div class="actions">
                    <AppLink to={Route::SecondPage} label="Open second page" />
                </div>
            </section>
        </main>
    }
}
