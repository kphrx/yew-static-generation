use yew::prelude::*;
use yew_router::prelude::*;

use crate::page::Home;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home />
        },
        Route::NotFound => html! {
            <h1>{ "Page Not Found" }</h1>
        },
    }
}

#[function_component(AppRouter)]
pub fn router() -> Html {
    html! {
        <Switch<Route> render={switch} />
    }
}
