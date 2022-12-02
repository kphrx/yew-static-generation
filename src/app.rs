use std::collections::HashMap;

use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

use crate::router::AppRouter;

#[function_component(AppTemplate)]
fn app_template() -> Html {
    html! {
        <>
            <main>
                <AppRouter />
            </main>
        </>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <AppTemplate />
        </BrowserRouter>
    }
}

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub url: yew::AttrValue,
    pub queries: HashMap<String, String>,
}

#[function_component(ServerApp)]
pub fn server_app(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    html! {
        <Router history={history}>
            <AppTemplate />
        </Router>
    }
}
