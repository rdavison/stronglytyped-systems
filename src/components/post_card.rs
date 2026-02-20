use leptos::prelude::*;

use crate::components::tag_badge::TagBadge;
use crate::posts::Post;

#[component]
pub fn PostCard(post: Post) -> impl IntoView {
    let href = format!("/blog/{}", post.slug);
    let title = post.title.to_string();
    let date = post.date.to_string();
    let description = post.description.to_string();
    let tags: Vec<String> = post.tags.iter().map(|t| t.to_string()).collect();

    view! {
        <a href=href class="post-card">
            <article>
                <h2 class="post-card__title">{title}</h2>
                <time class="post-card__date">{date}</time>
                <p class="post-card__description">{description}</p>
                <div class="post-card__tags">
                    {tags
                        .into_iter()
                        .map(|tag| {
                            view! { <TagBadge tag=tag /> }
                        })
                        .collect::<Vec<_>>()}
                </div>
            </article>
        </a>
    }
}
