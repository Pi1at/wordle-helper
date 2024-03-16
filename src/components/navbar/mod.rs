mod chart_bar_icon;
mod cog_icon;
mod information_circle_icon;

use crate::components::navbar::{
    chart_bar_icon::ChartBarIcon, cog_icon::CogIcon,
    information_circle_icon::InformationCircleIcon,
};
use leptos::*;
/// Navigaion Bar
#[component]
pub fn NavBar() -> impl IntoView {
    const GAME_TITLE: &str = "Wordle Helper";
    view! {
        <div class="navbar">
            <div class="flex items-center justify-between h-12 px-5">
                <div class="flex w-24 justify-start">
                    <InformationCircleIcon cl="h-6 w-6 mr-2 cursor-pointer dark:stroke-white"/>
                </div>
                <p class="text-xl font-bold dark:text-white">{GAME_TITLE}</p>
                <div class="flex w-24 justify-end">
                    <ChartBarIcon cl="h-6 w-6 mr-3 cursor-pointer dark:stroke-white"/>
                    <CogIcon cl="h-6 w-6 cursor-pointer dark:stroke-white"/>
                </div>
            </div>
            <hr/>
        </div>
    }
}
