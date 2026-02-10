use yew::prelude::*;

use crate::components::link::AppLink;
use crate::Route;

#[component]
pub fn SecondPage() -> Html {
    html! {
        <main class="home page-transition">
            <section class="hero">
                <p class="tag">{"Routing example"}</p>
                <div class="actions">
                    <AppLink to={Route::Home} label="Back to home" />
                </div>
            </section>
        </main>
    }
}
