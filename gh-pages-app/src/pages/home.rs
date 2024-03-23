use crate::components::{
    footer::Footer, navbar::NavBar, settings::Settings, WordleSolver,
};
use leptos::*;

/// Main page for app
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <main class="h-screen flex flex-col bg-[#fdfdfd] dark:bg-[#0a0a0a] dark:text-white">
            <NavBar/>
            <div class="space-y-24">
                <Settings/>
            // <Keyboard/>
            </div>
            <div class="flex">
                <div class="bg-red-500 w-14 h-14 flex items-center justify-center">
                    <span class="text-lg">"Я"</span>
                </div>
                <div class="bg-yellow-500 w-14 h-14 flex items-center justify-center">
                    <span class="letter-container">"Ч"</span>
                </div>
                <div class="w-14 h-14 bg-yellow-500 border-solid border-2 flex items-center justify-center mx-0.5 text-4xl font-bold rounded dark:text-white">
                    <div class="letter-container">"В"</div>
                </div>
                <div class="flex border-solid border-2 h-4 w-8">
                    <div class="bg-yellow-500 flex w-4 items-center justify-center text-pretty dark:text-white">
                        "В"
                    </div>
                    <div class="bg-red-500 flex w-4 items-center justify-center text-pretty dark:text-white">
                        "Ф"
                    </div>
                </div>
            </div>
            <WordleSolver/>
            <Footer/>
        </main>
    }
}
