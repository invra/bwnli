use yew::prelude::*;

pub struct PageRegistration {
    pub path: &'static str,
    pub render: fn() -> Html,
}

inventory::collect!(PageRegistration);

#[macro_export]
macro_rules! register_page {
    ($path:literal, $component:ty) => {
        inventory::submit! {
            $crate::router::PageRegistration {
                path: $path,
                render: || yew::html! { <$component /> },
            }
        }
    };
}
