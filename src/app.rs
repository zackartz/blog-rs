use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::markdown::{Markdown, MarkdownProps};
use crate::components::navbar::{Navbar, NavbarProps};
use crate::components::post::{Post, PostProps};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        <Stylesheet href="https://rsms.me/inter/inter.css" />

        <Stylesheet href="https://fonts.googleapis.com/css2?family=Fira+Code&display=swap"/>
        // sets the document title
        <Title text="Zachary Myers"/>

        // content for this welcome page
        <Router>
            <main>
            <div class="w-full h-screen bg-stone-900 flex justify-center">
            <div class="w-3/5">
                <Navbar />
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                    <Route path="/about" view=|cx| view! { cx, <About/> } />
                    <Route path="/blog" view=|cx| view! { cx, <Blog/> } />
                    <Route path="/blog/:id" view=|cx| view! { cx, <BlogPost/> }/>
                    <Route path="/test" view=|cx| view! { cx, <Test /> }/>
                </Routes>
            </div>
        </div>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
                <div class="w-full">
                    <h3 class="text-2xl text-stone-400 font-bold mt-10">"Recent Posts"</h3>
                    <div class="w-full grid grid-cols-2 gap-4 mt-8">
                        <Post />
                        <Post />
                        <Post />
                        <Post />
                    </div>
                </div>
                <div class="w-full border border-stone-800 mt-8" />
                <p class="text-center mt-4 text-stone-700">"&copy Zachary Myers"</p>
    }
}

/// Renders the about page of your application.
#[component]
fn About(cx: Scope) -> impl IntoView {
    view! { cx,
                <div class="w-full">
                    <h3 class="text-2xl text-stone-400 font-bold mt-10">"About"</h3>
                </div>
    }
}

/// Renders the blog page of your application.
#[component]
fn Blog(cx: Scope) -> impl IntoView {
    view! { cx,
                <div class="w-full">
                    <h3 class="text-2xl text-stone-400 font-bold mt-10">"Recent Posts"</h3>
                    <div class="w-full grid grid-cols-2 gap-4 mt-8">
                        <Post />
                        <Post />
                        <Post />
                        <Post />
                    </div>
                </div>
    }
}

#[component]
pub fn BlogPost(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);

    let post = create_resource(
        cx,
        move || params().get("id").cloned().unwrap_or_default(),
        move |id| async move {
            if id.is_empty() {
                None
            } else {
                Some("# markdown header".to_string())
            }
        },
    );

    view! { cx,
        <div>
            <Suspense fallback=|| view! { cx, "Loading..." }>
                {move || match post.read() {
                    None => view! {cx, <div>"Error loading post!"</div>},
                    Some(v) => view! {cx, <div>{v}</div>}
                }}
            </Suspense>
        </div>
    }
}

#[component]
pub fn Test(cx: Scope) -> impl IntoView {
    view! { cx,
        <Markdown />
    }
}
