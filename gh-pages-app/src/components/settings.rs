use leptos::*;
use leptos_theme::{use_theme, Theme};

/// Settings
#[component]
pub fn Settings() -> impl IntoView {
    let theme_buttons: [Theme; 3] = [Theme::Light, Theme::Dark, Theme::System];

    // 2. retrieve the theme_signal global state
    let theme_signal = use_theme();
    view! {
        <div class="space-y-20">
            <div class="space-y-2">
                // <p class="">Current theme: {move || theme_signal().to_string()}</p>

                <div class="flex space-x-4">
                    <p>"Select theme: "</p>
                    {theme_buttons
                        .into_iter()
                        .map(|theme| {
                            view! {
                                <button on:click=move |_| {
                                    theme_signal.set(theme);
                                }>
                                    <p>{theme.to_string()}</p>
                                </button>
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}
