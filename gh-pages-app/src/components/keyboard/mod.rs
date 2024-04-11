use leptos::*;

/// Keyboard component for wordle game board
#[component]
pub fn Keyboard() -> impl IntoView {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();

    let kb_btns = alphabet
        .into_iter()
        .map(|letter| {
            view! {
                <button class="flex items-center justify-center rounded mx-0.5 text-sm font-bold cursor-pointer select-none dark:text-white">
                    {letter}
                </button>
            }
        })
        .collect_view();

    view! { <div class="keyboard">{kb_btns}</div> }
}
