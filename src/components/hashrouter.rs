use crate::pages::home::Home;
use leptos::*;
use leptos_router::use_location;

#[component]
pub fn HashRouter() -> impl IntoView {
    let hash_memo = use_location().hash;
    view! {
        {move || {
            match hash_memo.get().as_str() {
                "" => view! { <Home/> },
                unknown_hash => format!("\"{}\" Route Not found", unknown_hash).into_view(),
            }
        }}
    }
}
