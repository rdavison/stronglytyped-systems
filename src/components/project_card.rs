use leptos::prelude::*;

use crate::components::tag_badge::TagBadge;
use crate::pages::projects::Project;

#[component]
pub fn ProjectCard(project: Project) -> impl IntoView {
    let has_demos = !project.demos.is_empty();
    let demos: Vec<(&'static str, &'static str)> = project.demos.to_vec();

    // For demo tabs: track which tab is active (default to first)
    let (active_demo, set_active_demo) = signal(0_usize);

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

            {if has_demos {
                let demo_tabs = demos.clone();
                let demo_frames = demos.clone();
                Some(view! {
                    <div class="project-card__demo">
                        <span class="project-card__demo-label">"Live Demo"</span>
                        <div class="project-card__demo-tabs">
                            {demo_tabs
                                .into_iter()
                                .enumerate()
                                .map(|(idx, (label, _url))| {
                                    let set_active = set_active_demo.clone();
                                    view! {
                                        <button
                                            class=move || {
                                                if active_demo.get() == idx {
                                                    "project-card__demo-tab project-card__demo-tab--active"
                                                } else {
                                                    "project-card__demo-tab"
                                                }
                                            }
                                            on:click=move |_| set_active.set(idx)
                                        >
                                            {label}
                                        </button>
                                    }
                                })
                                .collect::<Vec<_>>()}
                        </div>
                        {demo_frames
                            .into_iter()
                            .enumerate()
                            .map(|(idx, (_label, url))| {
                                view! {
                                    <iframe
                                        src=url
                                        style=move || {
                                            if active_demo.get() == idx {
                                                "display: block;"
                                            } else {
                                                "display: none;"
                                            }
                                        }
                                    />
                                }
                            })
                            .collect::<Vec<_>>()}
                    </div>
                })
            } else {
                None
            }}
        </div>
    }
}
