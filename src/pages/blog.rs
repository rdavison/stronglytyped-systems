use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::hooks::use_params_map;

use crate::components::post_card::PostCard;
use crate::components::tag_badge::TagBadge;
use crate::posts;

#[component]
pub fn BlogListPage() -> impl IntoView {
    let posts = posts::get_posts();

    view! {
        <Title text="Blog — stronglytyped.systems" />
        <div class="blog-list" data-page="blog">
            <h1 class="page-title">"Blog"</h1>
            <p class="page-subtitle">"Thoughts on systems programming, type theory, and building things that last."</p>
            {if posts.is_empty() {
                view! {
                    <div class="blog-list__empty">
                        <p>"No posts yet. The infrastructure is here — build-time markdown rendering, syntax highlighting, the works. The words will follow."</p>
                    </div>
                }
                    .into_any()
            } else {
                view! {
                    <div class="blog-list__posts">
                        {posts
                            .into_iter()
                            .map(|post| {
                                view! { <PostCard post=post /> }
                            })
                            .collect::<Vec<_>>()}
                    </div>
                }
                    .into_any()
            }}
        </div>
    }
}

#[component]
pub fn BlogPostPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.read().get("slug");

    let post = move || {
        slug().and_then(|s| posts::get_post_by_slug(&s))
    };

    view! {
        <div class="blog-post" data-page="blog">
            {move || match post() {
                Some(post) => {
                    let title = post.title.to_string();
                    let date = post.date.to_string();
                    let html = post.html.to_string();
                    let page_title = format!("{} — stronglytyped.systems", title);
                    let tags: Vec<String> = post.tags.iter().map(|t| t.to_string()).collect();

                    view! {
                        <Title text=page_title />
                        <article class="blog-post__article">
                            <header class="blog-post__header">
                                <h1 class="blog-post__title">{title}</h1>
                                <div class="blog-post__meta">
                                    <time class="blog-post__date">{date}</time>
                                    <div class="blog-post__tags">
                                        {tags
                                            .into_iter()
                                            .map(|tag| {
                                                view! { <TagBadge tag=tag /> }
                                            })
                                            .collect::<Vec<_>>()}
                                    </div>
                                </div>
                            </header>
                            <div class="blog-post__content" inner_html=html />
                        </article>
                    }
                        .into_any()
                }
                None => {
                    view! {
                        <Title text="Post Not Found — stronglytyped.systems" />
                        <div class="blog-post__not-found">
                            <h1>"Post not found"</h1>
                            <p>"The post you're looking for doesn't exist."</p>
                            <a href="/blog">"Back to blog"</a>
                        </div>
                    }
                        .into_any()
                }
            }}
        </div>
    }
}
