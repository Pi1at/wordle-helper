use super::pages::{error_page::ErrorPage, home::Home};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use leptos_theme::ThemeProvider;
/// An app router which renders the homepage and handles 404's
#[component]
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
