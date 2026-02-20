use leptos::prelude::*;

#[component]
pub fn TagBadge(tag: String) -> impl IntoView {
    view! {
        <span class="tag-badge">{tag}</span>
    }
}
