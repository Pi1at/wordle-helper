#![allow(clippy::module_name_repetitions)]
pub mod footer;
pub mod hashrouter;
pub mod keyboard;
pub mod navbar;
pub mod settings;

use leptos::*;
use leptos_use::{use_cycle_list, UseCycleListReturn};

#[component]
pub fn WordleSolver() -> impl IntoView {
    //let words = ["hello", "world", "leptos"];
    let (word, set_word) = create_signal(String::new());

    view! {
        <div>
            // <input
            // placeholder="Enter a word"
            // value=move || word
            // on:input=move |ev| set_word(event_target_value(&ev))
            // />
            // {move || {
            // words
            // .iter()
            // .filter(|w| w.contains(word))
            // .map(|word| {
            // view! { <li>{word}</li> }
            // })
            // .collect::<Vec<_>>()
            // }}
            <ul></ul>
        </div>
    }
}

#[derive(Default, Clone, Copy)]
pub struct Point2D {
    pub x: usize,
    pub y: usize,
}

#[derive(Default, Clone)]
pub struct Letter {
    pub grapheme: String,
    pub state: u8,
}

#[component]
pub fn LetterBox(letter: RwSignal<Letter>) -> impl IntoView {
    let styles = ["bg-gray-500", "bg-yellow-500", "bg-green-500"];
    //let UseCycleListReturn { state, next, .. } = use_cycle_list(vec![0, 1,
    // 2]);
    let grapheme = move || letter().grapheme;
    let style = move || styles[letter().state as usize];
    view! {
        <div
            class=move || {
                format!(
                    "w-14 h-14 border-solid border-2 flex items-center justify-center mx-0.5 text-4xl font-bold rounded dark:text-white select-none cursor-pointer {}",
                    style(),
                )
            }

            on:click=move |_| {
                letter.update(|l| l.state = (l.state + 1) % 3);
                log::debug!("{}", style());
            }

            on:keydown=move |e| { log::debug!("{}", e.key()) }
        >

            <div class="letter-container">{grapheme()}</div>
        </div>
    }
}

#[component]
pub fn WordRow(word: Vec<RwSignal<Letter>>) -> impl IntoView {
    view! {
        <div class="flex">
            {word.into_iter().map(|l| view! { <LetterBox letter=l/> }).collect::<Vec<_>>()}
        </div>
    }
}
