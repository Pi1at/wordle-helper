#![allow(clippy::wildcard_imports)]

mod components;
mod pages;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::ThemeProvider;
use pages::{error_page::ErrorPage, home::Home};

#[component]
#[must_use]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    view! {
        <ThemeProvider>
            <Router base="/wordle-helper/" trailing_slash=TrailingSlash::Redirect>
                <Routes>
                    <Route path="/" view=Home>
                        <Route path="/:page" view=ErrorPage/>
                    </Route>
                    <Route path="/:page" view=ErrorPage/>
                </Routes>
                <Redirect path="/"/>
            </Router>
        </ThemeProvider>
    }
}

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
