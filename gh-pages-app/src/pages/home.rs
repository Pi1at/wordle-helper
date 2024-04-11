use std::iter;

use leptos::{ev::keydown, *};
use leptos_lucide::{icons::*, LucideAttributes, LucideAttributesCtx};
use leptos_use::{use_event_listener, use_window};
use web_sys::js_sys::Iterator;

use crate::components::{
    footer::Footer, settings::Settings, Letter, LetterBox, Point2D, WordRow,
    WordleSolver,
};

/// Main page for app
#[component]
pub fn Home() -> impl IntoView {
    let attributes = LucideAttributes::new()
        //.set_height("24")
        //.set_width("96")
        //.set_stroke_width("1.625")
        .set_classes("h-6 w-6 cursor-pointer");
    let attributes_ctx = LucideAttributesCtx(RwSignal::new(attributes));
    provide_context(attributes_ctx);
    let (active_cell, set_active_cell) = create_signal(Point2D::default());

    let create_letter_signal = || create_rw_signal(Letter::default());
    let create_row = || {
        vec![
            create_letter_signal(),
            create_letter_signal(),
            create_letter_signal(),
            create_letter_signal(),
            create_letter_signal(),
        ]
    };
    let (word_grid, wg) = create_signal(vec![
        create_row(),
        create_row(),
        create_row(),
        create_row(),
        create_row(),
        create_row(),
    ]);
    //let wg = word_grid.clone();
    let _ = use_event_listener(use_window(), keydown, move |evt| {
        match evt.key().as_str() {
            "ArrowUp" => {
                set_active_cell.update(|v| v.y = v.y.saturating_sub(1));
                log::debug!("y:{} x:{}", active_cell().y, active_cell().x);
            }
            "ArrowDown" => {
                set_active_cell.update(|v| v.y = (v.y + 1).min(5));
                log::debug!("y:{} x:{}", active_cell().y, active_cell().x);
            }
            "ArrowLeft" => {
                set_active_cell.update(|v| v.x = v.x.saturating_sub(1));
                log::debug!("y:{} x:{}", active_cell().y, active_cell().x);
            }
            "ArrowRight" => {
                set_active_cell.update(|v| v.x = (v.x + 1).min(4));
                log::debug!("y:{} x:{}", active_cell().y, active_cell().x);
            }
            "Backspace" => {
                (wg.update(|w| {
                    (w[active_cell().y][active_cell().x])
                        .update(|l| l.grapheme = String::new());
                }));
                set_active_cell.update(|v| v.x = v.x.saturating_sub(1));
                log::debug!("y:{} x:{}", active_cell().y, active_cell().x);
            }
            " " => {
                (wg.update(|w| {
                    (w[active_cell().y][active_cell().x])
                        .update(|l| l.state = (l.state + 1) % 3);
                }));
            }
            k if "йцукенгшщзхъфывапролджэячсмитьбю".contains(k) =>
            {
                (wg.update(|w| {
                    (w[active_cell().y][active_cell().x])
                        .update(|l| l.grapheme = k.to_uppercase());
                }));
                set_active_cell.update(|v| v.x = (v.x + 1).min(4));
                log::debug!("y:{} x:{}", active_cell().y, active_cell().x);
            }
            _ => log::debug!("window keydown: '{}'", evt.key()),
        }
    });

    //let l = word_grid[0][0].clone();
    view! {
        <main class="h-screen flex flex-col bg-[#fdfdfd] dark:bg-[#0a0a0a] dark:text-white items-center">
            <h1>"Wordle Helper (alpha)"</h1>
            <div>use arrows, backspace and russian alphabet</div>
            <div>click or space to change letter state</div>
            <div class="space-y-24">
                <Settings/>
            </div>
            <div class="flex">
                <ul>

                    {move || {
                        word_grid()
                            .into_iter()
                            .map(|w| {
                                view! {
                                    <li>
                                        <WordRow word=w/>
                                    </li>
                                }
                            })
                            .collect_view()
                    }}

                </ul>
            </div>
            <div>X: {move || active_cell().x + 1} Y: {move || active_cell().y + 1}</div>
            <WordleSolver/>
            <Footer/>
        </main>
    }
}
