use leptos::*;
use leptos_router::use_params_map;

/// Error page (404 not found)
#[component]
pub fn ErrorPage() -> impl IntoView {
    let params = use_params_map();
    let p_unknown =
        move || params.with(|p| p.get("page").cloned().unwrap_or_default());

    let unknown = p_unknown();

    view! {
        <main class="bg-white dark:bg-[#0a0a0a] dark:text-white text-black h-screen w-full flex flex-col items-center justify-center">
            <p class="font-robotomono">Unknown command: {unknown}</p>
        </main>
    }
}
