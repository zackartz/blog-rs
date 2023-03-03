use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::navbar::{Navbar, NavbarProps};
use crate::components::post::{get_posts, BlogPost, BlogPostProps, Post, PostProps};

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
                            <Route path="/blog/:slug" view=|cx| view! { cx, <BlogPost/> }/>
                            // <Route path="/test" view=|cx| view! { cx, <Test /> }/>
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
    let posts = create_resource(cx, move || (), move |_| get_posts(cx));

    view! { cx,
        <div class="w-full">
            <h3 class="text-2xl text-stone-400 font-bold mt-10">"Recent Posts"</h3>
                <Suspense fallback=|| view! { cx, "Loading..." }>
                    {
                        match posts.read(cx) {
                            Some(Ok(p)) => {
                                view! { cx,
                                    <div class="w-full grid grid-cols-2 gap-4 mt-8">
                                    // <For
                                    //     each=move || p.clone()
                                    //     key=|post| post.id
                                    //     view=move |post: crate::components::post::BlogPostRow | {
                                    //         view! {cx,
                                    //             <Post p=post />
                                    //         }
                                    //     }
                                    // />
                                        {p.into_iter().map(move |post| {
                                            view! { cx,
                                                <Post p=post />
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                }.into_any()
                            },
                            _ => view! {cx,
                                <h1>"Error loading posts :("</h1>
                            }.into_any(),
                        }
                    }
                </Suspense>
        </div>
        <div class="w-full border border-stone-800 mt-8" />
        <p class="text-center mt-4 text-stone-700">"© Zachary Myers"</p>
    }
}

// posts.iter().map(|p| {
// view! { cx,
//     <Post p=p.clone() />
// }

/// Renders the about page of your application.
#[component]
fn About(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="w-full">
            <h3 class="text-2xl text-stone-400 font-bold mt-10">"About"</h3>
            <p class="text-stone-500 pt-2">"Hey there, I'm Zachary Myers, a self-taught software engineer originally from the sunny shores of Honolulu, HI. I've been working with Golang for a while now and currently diving into Rust. I love tinkering with both front and backend development and always looking for new ways to expand my programming knowledge. If you have any questions or just want to chat, shoot me an email at "<a class="text-stone-400 underline" href="mailto:me@zackmyers.io">"me@zackmyers.io"</a>"."</p>
        </div>
    }
}

/// Renders the blog page of your application.
#[component]
fn Blog(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="w-full ">
            <h3 class="text-2xl text-stone-400 font-bold mt-10">"Recent Posts"</h3>
            <div class="w-full grid grid-cols-2 gap-4 mt-8">
            </div>
        </div>
    }
}

// #[component]
// pub fn Test(cx: Scope) -> impl IntoView {
//     view! { cx,
//         <Markdown />
//     }
// }
