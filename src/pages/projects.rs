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
}

fn get_projects() -> Vec<Project> {
    vec![
        Project {
            name: "Coself",
            description: "AI agent system that orchestrates LLM reasoning across Discord, WhatsApp, and HTTP through a single brain with persistent memory. Event-driven runtime with typed dispatch routes to scoped programs. Combines probabilistic reasoning with formal verification via Prolog, Lean 4, Futhark, and MiniZinc.",
            tags: &["Rust", "AI", "Axum", "Tokio", "Lean 4"],
            url: None,
            source: Some("https://github.com/rdavison/coself"),
        },
        Project {
            name: "Coself Portal",
            description: "Native macOS application that renders live agent state as a spatial field — beliefs, conversation topology, and internal reasoning visualized on a GPU-rendered canvas. The visual cortex of the Coself system.",
            tags: &["Swift", "SwiftUI", "Metal", "macOS"],
            url: None,
            source: None,
        },
        Project {
            name: "sferrakl",
            description: "Keyboard layout analyzer that models finger travel, bigram frequency, and ergonomic cost to compare layouts like QWERTY, Colemak, and Dvorak. Full-stack Leptos app with server-side rendering and WASM hydration.",
            tags: &["Rust", "Leptos", "WASM", "SSR"],
            url: None,
            source: None,
        },
        Project {
            name: "Rektangle",
            description: "Fork of Rectangle (macOS window manager) with substantial original contributions: carousel-based app switcher with alpha crossfade rendering, window staging system, and CVDisplayLink-driven animation pipeline.",
            tags: &["Swift", "macOS", "CoreGraphics", "CVDisplayLink"],
            url: None,
            source: Some("https://github.com/rdavison/Rectangle"),
        },
        Project {
            name: "cubesim",
            description: "Interactive 3D Rubik's cube simulator built with Bevy. Oblique projection rendering, planar reflection mirrors for hidden faces, scramble generation, and smooth rotation animation.",
            tags: &["Rust", "Bevy", "3D", "Game Engine"],
            url: None,
            source: Some("https://github.com/rdavison/cubesim"),
        },
        Project {
            name: "stronglytyped.systems",
            description: "This site. Leptos full-stack with SSR and WASM hydration. Build-time markdown rendering with syntect code highlighting. Zero JavaScript frameworks.",
            tags: &["Rust", "Leptos", "SSR", "WASM"],
            url: Some("https://stronglytyped.systems"),
            source: None,
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
