use leptos::*;
use leptos_lucide::icons::*;

/// Navigaion Bar
#[component]
pub fn NavBar() -> impl IntoView {
    const GAME_TITLE: &str = "Wordle Helper";
    view! {
        <div class="navbar">
            <div class="flex items-center justify-between h-12 px-5">
                <div class="flex w-24 justify-start">
                    // cl="h-6 w-6 mr-2 cursor-pointer dark:stroke-white"
                    <Info/>
                </div>
                <p class="text-xl font-bold dark:text-white">{GAME_TITLE}</p>
                <div class="flex w-24 justify-end">
                    // cl="h-6 w-6 mr-3 cursor-pointer dark:stroke-white"
                    <BarChart/>
                    // cl="h-6 w-6 cursor-pointer dark:stroke-white"
                    <Cog/>
                </div>
            </div>
            <hr/>
        </div>
    }
}
