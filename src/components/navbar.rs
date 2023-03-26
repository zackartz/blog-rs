use leptos::*;

/// Renders the navbar.
#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {
    let links = vec!["Home", "Blog", "About", "CV"];

    let items = links
        .iter()
        .map(|l| {
            view! { cx, <NavbarItem _l=Box::new(l)/> }
        })
        .collect::<Vec<_>>();

    view! { cx,
        <div class="w-full bg-stone-900 flex block justify-center align-center">
            <div class="w-full pt-14">
                <div class="float-left">
                    <h1 class="text-stone-50 font-bold text-4xl">"Zachary Myers"</h1>
                    <p class="text-stone-500 font-semibold text-xl pt-2">
                        "Software Engineer based out of Honolulu, HI"
                    </p>
                </div>
                <div class="flex pt-6 float-right gap-x-4 text-stone-700">{items}</div>
            </div>
        </div>
        <div class="w-full bg-stone-900 flex block justify-center align-center">
            <div class="w-full border border-stone-800 mt-8"></div>
        </div>
    }
}

#[component]
fn NavbarItem(cx: Scope, _l: Box<&'static str>) -> impl IntoView {
    view! { cx,
        <a
            href={match *_l {
                "Home" => "/".to_string(),
                "CV" => "/cv.pdf".to_string(),
                _ => format!("/{}", _l.to_lowercase()),
            }}
            target={match *_l {
                "CV" => Some("_blank"),
                _ => None,
            }}
            class="py-1 px-2 hover:bg-stone-800 hover:text-stone-500 border border-stone-900 rounded-md hover:border-stone-700 transition-all duration-150"
        >
            <p>{*_l}</p>
        </a>
    }
}
