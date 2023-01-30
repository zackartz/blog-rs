use leptos::*;
use pulldown_cmark::{html, Parser};

#[allow(dead_code)]
fn markdown_to_html(markdown: &str) -> String {
    let mut html_output = String::new();
    let parser = Parser::new(markdown);
    html::push_html(&mut html_output, parser);
    html_output
}

#[component]
pub fn Markdown(_cx: Scope, _md: String) -> impl IntoView {
    #[cfg(not(target_arch = "wasm32"))]
    {
        leptos::HtmlElement::from_html(_cx, leptos::A::default(), {
            let res = format!(
                "<article class=\"block w-full max-w-none prose prose-invert\">{}</article>",
                markdown_to_html(&_md)
            );
            res
        })
    }
}
