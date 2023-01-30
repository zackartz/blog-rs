use leptos::*;

/// Renders the navbar.
#[component]
pub fn Post(cx: Scope) -> impl IntoView {
    view! { cx,
        <a href="/blog/1">
            <div class="p-8 rounded-md transition hover:scale-105 hover:shadow-2xl duration-250 border-2 border-stone-900 hover:border-stone-800 hover:bg-gradient-to-tl from-stone-800 to-transparent">
                <h4 class="font-bold text-lg text-stone-200">"Post Title Goes Here"</h4>
                <p class="text-stone-500 font-serif">"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Suscipit tellus mauris a diam. "</p>
            </div>
        </a>
    }
}
