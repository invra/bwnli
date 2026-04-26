mod pages;
mod ui;
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Suspense>
                <Routes fallback=|| view! { <p>"FALLBACK"</p> }>
                    <Route path=path!("/") view=pages::home::Home />
                </Routes>
            </Suspense>
        </Router>
    }
}
