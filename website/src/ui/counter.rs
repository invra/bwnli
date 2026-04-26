use leptos::prelude::*;
use stylist::{Style, css};

#[component]
pub fn Counter() -> impl IntoView {
    let (value, set_value) = signal(0);
    let btn_wide = Style::new(css!("min-width: 200px;")).unwrap();
    let button_class = format!("btn btn-primary {}", btn_wide.get_class_name());

    view! {
        <div class="d-flex flex-column gap-2 justify-content-center align-items-center vh-100">
            <p class="display-1 text-center">{value}</p>
            <button class={button_class.clone()} on:click=move |_| set_value.update(|v| *v -= 1)>{ "-" }</button>
            <button class={button_class.clone()} on:click=move |_| set_value.update(|v| *v += 1)>{ "+" }</button>
        </div>
    }
}
