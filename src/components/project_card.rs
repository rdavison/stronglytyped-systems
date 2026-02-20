use leptos::prelude::*;

use crate::components::tag_badge::TagBadge;
use crate::pages::projects::Project;

#[component]
pub fn ProjectCard(project: Project) -> impl IntoView {
    let has_demos = !project.demos.is_empty();
    let demos: Vec<(&'static str, &'static str)> = project.demos.to_vec();
    let screenshot = project.screenshot;

    let (demo_open, set_demo_open) = signal(false);
    let (active_demo, set_active_demo) = signal(0_usize);

    view! {
        <div class=move || {
            if demo_open.get() {
                "project-card project-card--demo-open"
            } else {
                "project-card"
            }
        }>
            {screenshot.map(|src| {
                let has_demos_for_badge = has_demos;
                let set_open = set_demo_open;
                view! {
                    <div
                        class="project-card__preview"
                        style=move || if demo_open.get() { "display: none;" } else { "" }
                    >
                        <img
                            src=src
                            alt="Project screenshot"
                            class="project-card__screenshot"
                        />
                        {if has_demos_for_badge {
                            Some(view! {
                                <button
                                    class="project-card__demo-badge"
                                    on:click=move |_| set_open.set(true)
                                >
                                    "Try Demo"
                                </button>
                            })
                        } else {
                            None
                        }}
                    </div>
                }
            })}

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
                let multi_tab = demo_tabs.len() > 1;
                Some(view! {
                    <div
                        class="project-card__demo"
                        style=move || if demo_open.get() { "" } else { "display: none;" }
                    >
                        <div class="project-card__demo-header">
                            <span class="project-card__demo-label-inline">"Live Demo"</span>
                            {if multi_tab {
                                let tabs = demo_tabs.clone();
                                Some(view! {
                                    <div class="project-card__demo-tabs">
                                        {tabs
                                            .into_iter()
                                            .enumerate()
                                            .map(|(idx, (label, _url))| {
                                                let set_active = set_active_demo;
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
                                })
                            } else {
                                None
                            }}
                            <button
                                class="project-card__demo-close"
                                on:click=move |_| set_demo_open.set(false)
                            >
                                "\u{2715} Close"
                            </button>
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
