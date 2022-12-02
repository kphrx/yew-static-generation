use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </>
    }
}
