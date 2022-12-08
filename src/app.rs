use std::collections::HashMap;

use stylist::manager::StyleManager;
use stylist::yew::{styled_component, Global, ManagerProvider};
use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

use crate::router::AppRouter;

#[styled_component(AppTemplate)]
fn app_template() -> Html {
    html! {
        <>
            <Global css={css!(r#"html, body {
                        font-family: sans-serif;
                        padding: 0;
                        margin: 0;
                    }"#)} />
            <main>
                <AppRouter />
            </main>
        </>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let manager = (*use_memo(
        |_| StyleManager::new().expect("failed to create style manager."),
        (),
    ))
    .to_owned();

    html! {
        <Suspense fallback={html!{<div>{"Loading..."}</div>}}>
            <ManagerProvider {manager}>
                <BrowserRouter>
                    <AppTemplate />
                </BrowserRouter>
            </ManagerProvider>
        </Suspense>
    }
}

#[derive(Properties, PartialEq, Debug)]
pub struct ServerAppProps {
    pub url: yew::AttrValue,
    pub queries: HashMap<String, String>,
    pub style_manager: StyleManager,
}

#[function_component(ServerApp)]
pub fn server_app(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    html! {
        <Suspense fallback={html!{<div>{"Loading..."}</div>}}>
            <ManagerProvider manager={props.style_manager.clone()}>
                <Router {history}>
                    <AppTemplate />
                </Router>
            </ManagerProvider>
        </Suspense>
    }
}
