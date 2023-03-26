use leptos::*;

#[component]
pub fn Markdown(cx: Scope, md: Option<String>) -> impl IntoView {
    let md_txt = match md {
        Some(md) => md,
        None => {
            return view! { cx, <p>"couldn't render markdown"</p> }
            .into_any()
        }
    };
    let parser = pulldown_cmark::Parser::new(&md_txt);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    view! { cx,
        <div class="block w-full max-w-none prose prose-invert mt-8" inner_html=html_output></div>
    }
    .into_any()
}
