use leptos::*;
use leptos_router::use_location;

use crate::pages::home::Home;

#[component]
pub fn HashRouter() -> impl IntoView {
    let hash_memo = use_location().hash;
    view! {
        {move || {
            match hash_memo.get().as_str() {
                "" => view! { <Home/> },
                unknown_hash => format!("\"{unknown_hash}\" Route Not found").into_view(),
            }
        }}
    }
}
