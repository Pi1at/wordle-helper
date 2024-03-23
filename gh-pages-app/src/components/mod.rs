#![allow(clippy::module_name_repetitions)]
pub mod footer;
pub mod hashrouter;
pub mod keyboard;
pub mod navbar;
pub mod settings;

use leptos::*;

#[component]
pub fn WordleSolver() -> impl IntoView {
    //let words = ["hello", "world", "leptos"];
    let (word, set_word) = create_signal(String::new());

    view! {
        <div>
            <input
                placeholder="Enter a word"
                value=move || word
                on:input=move |ev| set_word(event_target_value(&ev))
            />
            // {move || {
            // words
            // .iter()
            // .filter(|w| w.contains(word))
            // .map(|word| {
            // view! { <li>{word}</li> }
            // })
            <ul>// .collect::<Vec<_>>()
            // }}

            </ul>
        </div>
    }
}
