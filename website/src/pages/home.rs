use yew::prelude::*;
use crate::register_page;
use crate::ui::CounterElem;

register_page!("/", Home);

#[function_component(Home)]
pub fn home() -> Html {
    html! { <CounterElem/> }
}
