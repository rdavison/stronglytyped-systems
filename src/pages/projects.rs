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
        description: "AI agent system that orchestrates LLM reasoning across Discord, WhatsApp, and HTTP through a single brain with persistent memory. Event-driven runtime with typed dispatch routes to scoped programs. Native macOS portal renders live state as a spatial field \u{2014} beliefs, conversation topology, and reasoning visualized on a GPU-rendered canvas.",
        highlight: Some("Single brain architecture serving three transports, with Lean 4 proofs verifying behavioral invariants before execution."),
        stack: "Rust \u{b7} Axum \u{b7} Tokio \u{b7} Lean 4 \u{b7} egui \u{b7} SwiftUI",
        url: None,
        demos: &[
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
            description: "Keyboard typing visualizer and effort analyzer. Animated fingers move across the keyboard at 50fps tracking travel distance, same-finger bigrams, scissors, rolls, and ergonomic cost. Includes a layout analytics tab comparing QWERTY, Colemak, Dvorak, and Graphite across five metrics.",
            highlight: Some("Custom effort model with spring-physics finger animation and comparative layout analysis across four keyboards."),
            stack: "OCaml \u{b7} Jane Street Bonsai \u{b7} js_of_ocaml",
            url: None,
            demos: &[("Demo", "/demos/qwerkey.html")],
            screenshot: Some("/screenshots/qwerkey.png"),
        },
        Project {
            name: "Frobisher",
            description: "Fork of Rectangle (macOS window manager) with original contributions: carousel-based app switcher with alpha crossfade, window staging system, and CVDisplayLink-driven animation. Named after Robert Frobisher from Cloud Atlas \u{2014} whose Cloud Atlas Sextet is a tiling algorithm rendered as music.",
            highlight: Some("CVDisplayLink animation pipeline with per-frame alpha compositing at display refresh rate."),
            stack: "Swift \u{b7} macOS \u{b7} CoreGraphics \u{b7} CVDisplayLink",
            url: None,
            demos: &[("Demo", "/demos/rektangle.html")],
            screenshot: Some("/screenshots/rektangle.png"),
        },
        Project {
            name: "cubesim",
            description: "Interactive 3D Rubik\u{2019}s cube simulator with perspective projection, per-face lighting, animated face rotations, and drag-to-rotate camera.",
            highlight: Some("Animated face rotations with eased interpolation and painter\u{2019}s algorithm depth sorting."),
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
