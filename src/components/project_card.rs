use leptos::prelude::*;

use crate::components::tag_badge::TagBadge;
use crate::pages::projects::Project;

#[component]
pub fn ProjectCard(project: Project) -> impl IntoView {
    view! {
        <div class="project-card">
            <h3 class="project-card__name">{project.name}</h3>
            <p class="project-card__description">{project.description}</p>
            <div class="project-card__tags">
                {project
                    .tags
                    .iter()
                    .map(|tag| {
                        view! { <TagBadge tag=tag.to_string() /> }
                    })
                    .collect::<Vec<_>>()}
            </div>
            <div class="project-card__links">
                {project
                    .url
                    .map(|url| {
                        view! {
                            <a href=url class="project-card__link" target="_blank" rel="noopener noreferrer">
                                "Live"
                            </a>
                        }
                    })}
                {project
                    .source
                    .map(|src| {
                        view! {
                            <a href=src class="project-card__link" target="_blank" rel="noopener noreferrer">
                                "Source"
                            </a>
                        }
                    })}
            </div>
        </div>
    }
}
