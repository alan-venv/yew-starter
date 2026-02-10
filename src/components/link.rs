use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[derive(Properties, PartialEq)]
pub struct AppLinkProps {
    pub to: Route,
    pub label: AttrValue,
}

#[component]
pub fn AppLink(props: &AppLinkProps) -> Html {
    html! {
        <Link<Route> classes="btn btn-primary" to={props.to.clone()}>
            {props.label.clone()}
        </Link<Route>>
    }
}
