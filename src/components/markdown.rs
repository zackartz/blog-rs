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
pub fn Markdown(cx: Scope) -> impl IntoView {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let el = leptos::HtmlElement::from_html(cx, leptos::A::default(), {
            let res = format!(
                "<div class=\"prose prose-invert\">{}</div>",
                markdown_to_html("# hello world\n```\ncodeblock\n```")
            );
            res
        });

        el
    }
    // view! { cx,
    //     <h1>"Hello World"</h1>
    // }
}
