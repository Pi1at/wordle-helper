use leptos::*;

/// Footer links
#[component]
pub fn Footer() -> impl IntoView {
    const REPO_LINK: &str = "https://github.com/Pi1at/wordle-helper";
    const LINK_CSS: &str = "hover:underline underline-offset-4 w-fit \
                            decoration-teal-800 dark:decoration-teal-300";

    let footer_links: [(&'static str, String); 2] = [
        ("Contribute", REPO_LINK.to_string()),
        ("Leave an issue", format!("{}/issues/new", REPO_LINK)),
    ];
    view! {
        <div class="flex justify-center space-x-8">

            {footer_links
                .into_iter()
                .map(|(text, link)| {
                    view! {
                        <a class=LINK_CSS href=link target="_blank" rel="noreferrer">
                            {text}
                        </a>
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
