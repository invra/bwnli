// use crate::register_page;
use crate::ui::CounterElem;
use leptos::prelude::*;

// register_page!("/", Home);

#[component]
pub fn Home() -> impl IntoView {
    view! { <CounterElem/> }
}
