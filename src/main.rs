mod components;
mod pages;

use components::link::AppLink;
use pages::home::HomePage;
use pages::second::SecondPage;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/second")]
    SecondPage,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <HomePage /> },
        Route::SecondPage => html! { <SecondPage /> },
        Route::NotFound => html! {
            <main class="home">
                <section class="hero">
                    <p class="tag">{"Not Found"}</p>
                    <h1>{"Page not found"}</h1>
                    <div class="actions">
                        <AppLink to={Route::Home} label="Back to home" />
                    </div>
                </section>
            </main>
        },
    }
}

#[component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
