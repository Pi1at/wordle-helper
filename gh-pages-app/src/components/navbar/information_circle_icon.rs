use leptos::*;

/// Navigaion Bar
#[component]
pub fn InformationCircleIcon(#[prop(into)] cl: String) -> impl IntoView {
    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            view-box="0 0 24 24"
            stroke-width="2"
            stroke="currentColor"
            aria-hidden="true"
            class=cl
        >
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
            ></path>
        </svg>
    }
}
