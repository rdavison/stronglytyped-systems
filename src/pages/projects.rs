use leptos::prelude::*;
use leptos_meta::*;

use crate::components::project_card::ProjectCard;

#[derive(Clone)]
pub struct Project {
    pub name: &'static str,
    pub description: &'static str,
    pub tags: &'static [&'static str],
    pub url: Option<&'static str>,
    pub source: Option<&'static str>,
    pub demos: &'static [(&'static str, &'static str)],
    pub screenshot: Option<&'static str>,
}

fn get_projects() -> Vec<Project> {
    vec![
        Project {
            name: "Coself",
            description: "AI agent system that orchestrates LLM reasoning across Discord, WhatsApp, and HTTP through a single brain with persistent memory. Event-driven runtime with typed dispatch routes to scoped programs. Native macOS portal renders live state as a spatial field \u{2014} beliefs, conversation topology, and reasoning visualized on a GPU-rendered canvas with 11 rendering modes.",
            tags: &["Rust", "AI", "Axum", "Tokio", "Lean 4", "egui", "SwiftUI"],
            url: None,
            source: Some("https://github.com/rdavison/coself"),
            demos: &[
                ("Dashboard", "/demos/dashboard.html"),
                ("Conversation", "/demos/conversation.html"),
                ("Trip Planner", "/demos/trip.html"),
            ],
            screenshot: Some("/screenshots/coself.png"),
        },
        Project {
            name: "qwerkey",
            description: "Keyboard typing visualizer and effort analyzer. Watch animated fingers move across the keyboard at 50fps while it tracks travel distance, same-finger bigrams, scissors, rolls, and ergonomic cost. Supports 6 layouts including a custom-designed one optimized for alternation. Dark/light themes, command palette, and real-time stats overlay.",
            tags: &["OCaml", "Bonsai", "js_of_ocaml", "SVG"],
            url: None,
            source: Some("https://github.com/rdavison/qwerkey"),
            demos: &[("Demo", "/demos/qwerkey.html")],
            screenshot: Some("/screenshots/qwerkey.png"),
        },
        Project {
            name: "sferrakl",
            description: "Keyboard layout analyzer that models finger travel, bigram frequency, and ergonomic cost to compare layouts like QWERTY, Colemak, and Dvorak. Full-stack Leptos app with server-side rendering and WASM hydration.",
            tags: &["Rust", "Leptos", "WASM", "SSR"],
            url: None,
            source: None,
            demos: &[("Demo", "/demos/sferrakl.html")],
            screenshot: Some("/screenshots/sferrakl.png"),
        },
        Project {
            name: "Rektangle",
            description: "Fork of Rectangle (macOS window manager) with substantial original contributions: carousel-based app switcher with alpha crossfade rendering, window staging system, and CVDisplayLink-driven animation pipeline.",
            tags: &["Swift", "macOS", "CoreGraphics", "CVDisplayLink"],
            url: None,
            source: Some("https://github.com/rdavison/Rectangle"),
            demos: &[("Demo", "/demos/rektangle.html")],
            screenshot: Some("/screenshots/rektangle.png"),
        },
        Project {
            name: "cubesim",
            description: "Interactive 3D Rubik\u{2019}s cube simulator built with Bevy. Oblique projection rendering, planar reflection mirrors for hidden faces, scramble generation, and smooth rotation animation.",
            tags: &["Rust", "Bevy", "3D", "Game Engine"],
            url: None,
            source: Some("https://github.com/rdavison/cubesim"),
            demos: &[("Demo", "/demos/cubesim.html")],
            screenshot: Some("/screenshots/cubesim.png"),
        },
        Project {
            name: "stronglytyped.systems",
            description: "This site. Leptos full-stack with SSR and WASM hydration. Build-time markdown rendering with syntect code highlighting. Zero JavaScript frameworks.",
            tags: &["Rust", "Leptos", "SSR", "WASM"],
            url: Some("https://stronglytyped.systems"),
            source: None,
            demos: &[],
            screenshot: None,
        },
    ]
}

#[component]
pub fn ProjectsPage() -> impl IntoView {
    let projects = get_projects();

    view! {
        <Title text="Projects — stronglytyped.systems" />
        <div class="projects" data-page="projects">
            <h1 class="page-title">"Projects"</h1>
            <p class="page-subtitle">"Things I've built. All verified — no vaporware."</p>
            <div class="projects__grid">
                {projects
                    .into_iter()
                    .map(|project| {
                        view! { <ProjectCard project=project /> }
                    })
                    .collect::<Vec<_>>()}
            </div>
        </div>
    }
}
