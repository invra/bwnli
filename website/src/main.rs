mod router;
mod ui;
mod pages;

use router::PageRegistration;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Inner)]
fn inner() -> Html {
    let location = use_location().unwrap();
    let path = location.path();

    let matched = inventory::iter::<PageRegistration>
        .into_iter()
        .find(|p| p.path == path);

    match matched {
        Some(page) => (page.render)(),
        None => html! { <h1>{"404"}</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Inner />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
