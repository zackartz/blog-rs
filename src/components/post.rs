use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use cfg_if::cfg_if;
use serde::{Deserialize, Serialize};

use crate::components::markdown::{Markdown, MarkdownProps};

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{Pool, postgres::{Postgres, PgPoolOptions}};

        pub async fn db() -> Result<Pool<Postgres>, ServerFnError> {
            let uri = std::env::var("DATABASE_URL").unwrap_or("postgres://postgres:postgres@localhost:5432/postgres".to_string());

            PgPoolOptions::new()
                .max_connections(5)
                .connect(&uri)
                .await
            .map_err(|e| ServerFnError::ServerError(e.to_string()))
        }

        pub fn register_server_functions() {
            _ = GetPosts::register();
            _ = GetPost::register();
        }

        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
        pub struct BlogPostRow {
            pub id: i64,
            title: String,
            slug: String,
            post: String,
            summary: String,
        }
    } else {
        #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
        pub struct BlogPostRow {
            pub id: i64,
            title: String,
            slug: String,
            post: String,
            summary: String,
        }
    }
}

#[server(GetPosts, "/api")]
pub async fn get_posts(cx: Scope) -> Result<Vec<BlogPostRow>, ServerFnError> {
    let conn = db().await?;

    let posts = sqlx::query_as!(BlogPostRow, "SELECT * from posts")
        .fetch_all(&conn)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    Ok(posts)
}

#[server(GetPost, "/api")]
pub async fn get_post(cx: Scope, slug: String) -> Result<BlogPostRow, ServerFnError> {
    let conn = db().await?;

    let post = sqlx::query_as!(BlogPostRow, "SELECT * from posts where slug = $1", slug)
        .fetch_one(&conn)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;

    Ok(post)
}

#[component]
pub fn BlogPost(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);

    let post = create_resource(
        cx,
        move || params().get("slug").cloned().unwrap_or_default(),
        move |id| get_post(cx, id),
    );

    view! { cx,
        <div class="mt-8 w-full">
            <Suspense fallback=|| {
                view! { cx, "Loading..." }
            }>
                {match post.read(cx) {
                    Some(Ok(v)) => {
                        view! { cx,
                            <h1 class="text-4xl font-bold text-stone-50">{v.title}</h1>
                            <Markdown md=Some(v.post)/>
                        }
                    }
                    _ => {
                        view! { cx,
                            <>
                                <Markdown md=Some("error".to_string())/>
                            </>
                        }
                    }
                }}
            </Suspense>
        </div>
    }
}

/// Renders the post component.
#[component]
pub fn Post(cx: Scope, p: BlogPostRow) -> impl IntoView {
    view! { cx,
        <a href=format!("/blog/{}", p.slug)>
            <div class="p-8 rounded-md transition hover:scale-105 hover:shadow-2xl duration-250 border-2 border-stone-900 hover:border-stone-800 hover:bg-gradient-to-tl from-stone-800 to-transparent">
                <h4 class="font-bold text-lg text-stone-200">{p.title}</h4>
                <p class="text-stone-500 font-serif">{p.summary}</p>
            </div>
        </a>
    }
}
