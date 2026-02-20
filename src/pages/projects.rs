use leptos::prelude::*;
use leptos_meta::*;

use crate::components::project_card::ProjectCard;

#[derive(Clone)]
pub struct Project {
    pub name: &'static str,
    pub description: &'static str,
    pub highlight: Option<&'static str>,
    pub stack: &'static str,
    pub url: Option<&'static str>,
    pub demos: &'static [(&'static str, &'static str)],
    pub screenshot: Option<&'static str>,
}

fn get_flagship() -> Project {
    Project {
        name: "Coself",
        description: "AI agent system with a single brain serving Discord, WhatsApp, and HTTP. Persistent memory, event-driven runtime with typed dispatch, and a native macOS portal rendering live state on a GPU canvas.",
        highlight: Some("Formally verified behavioral invariants. Four reasoning engines running in parallel per invocation."),
        stack: "Rust \u{b7} Axum \u{b7} Tokio \u{b7} Lean 4 \u{b7} egui \u{b7} SwiftUI",
        url: None,
        demos: &[
            ("Assistant", "/demos/assistant.html"),
            ("Sprint Lifecycle", "/demos/sprint-lifecycle.html"),
            ("Sprint Kanban", "/demos/sprint-kanban.html"),
            ("Trip Planner", "/demos/trip.html"),
        ],
        screenshot: Some("/screenshots/coself.png"),
    }
}

fn get_explorations() -> Vec<Project> {
    vec![
        Project {
            name: "qwerkey",
            description: "Keyboard typing visualizer and effort analyzer. Spring-physics finger animation at 50fps with travel distance, same-finger bigrams, and ergonomic cost tracking. Layout analytics tab comparing four keyboards across five metrics.",
            highlight: Some("Custom effort model with real-time finger animation and comparative layout analysis."),
            stack: "OCaml \u{b7} Bonsai \u{b7} js_of_ocaml",
            url: None,
            demos: &[("Demo", "/demos/qwerkey.html")],
            screenshot: Some("/screenshots/qwerkey.png"),
        },
        Project {
            name: "macOS window manager",
            description: "macOS window manager with carousel-based app switcher, alpha crossfade transitions, and window staging. CVDisplayLink-driven animation at display refresh rate.",
            highlight: Some("Per-frame alpha compositing pipeline synced to the display refresh rate."),
            stack: "Swift \u{b7} macOS \u{b7} CoreGraphics \u{b7} CVDisplayLink",
            url: None,
            demos: &[("Demo", "/demos/rektangle.html")],
            screenshot: Some("/screenshots/rektangle.png"),
        },
        Project {
            name: "cubesim",
            description: "Interactive 3D Rubik\u{2019}s cube simulator with perspective projection, per-face lighting, animated rotations, and drag-to-rotate camera.",
            highlight: Some("3D perspective rendering with painter\u{2019}s algorithm and eased face rotation animation."),
            stack: "Rust \u{b7} Bevy \u{b7} 3D",
            url: None,
            demos: &[("Demo", "/demos/cubesim.html")],
            screenshot: Some("/screenshots/cubesim.png"),
        },
    ]
}

#[component]
pub fn ProjectsPage() -> impl IntoView {
    let flagship = get_flagship();
    let explorations = get_explorations();

    view! {
        <Title text="Projects — stronglytyped.systems" />
        <div class="projects" data-page="projects">
            <h1 class="page-title">"Projects"</h1>
            <p class="page-subtitle">"Things I\u{2019}ve built. All verified \u{2014} no vaporware."</p>

            <div class="projects__flagship">
                <ProjectCard project=flagship />
            </div>

            <h2 class="projects__section-heading">"Explorations"</h2>

            <div class="projects__grid">
                {explorations
                    .into_iter()
                    .map(|project| {
                        view! { <ProjectCard project=project /> }
                    })
                    .collect::<Vec<_>>()}
            </div>
        </div>
    }
}
