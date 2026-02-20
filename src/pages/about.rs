use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <Title text="About — stronglytyped.systems" />
        <div class="about" data-page="about">
            <h1 class="page-title">"About"</h1>

            <section class="about__bio">
                <p class="about__lead">
                    "11+ years shipping production systems across adtech, genomics, cloud infrastructure, and AI. "
                    "I think in type systems and composition laws."
                </p>
                <p>
                    "I play bassoon at an orchestral level, speak six languages, and solve Rubik\u{2019}s cubes competitively. "
                    "Outside of work, I keep returning to three domains: keyboards and input systems, "
                    "music, and programming languages."
                </p>
            </section>

            <section class="about__depths">
                <h2>"Where I Go Deep"</h2>

                <div class="depth-story">
                    <h3>"Formal verification in production"</h3>
                    <p>
                        "Lean 4, Prolog, MiniZinc, and Futhark running as reasoning engines \u{2014} "
                        "proving behavioral invariants, deductive queries, constraint optimization, "
                        "and GPU-parallel computation. Decisions are formally verified, not just tested."
                    </p>
                </div>

                <div class="depth-story">
                    <h3>"Type-driven systems in Rust and OCaml"</h3>
                    <p>
                        "Seven years of OCaml, then Rust. Parse-don\u{2019}t-validate, exhaustive matching, "
                        "newtype wrappers. If the compiler can prove it, I don\u{2019}t test it. If it can\u{2019}t, I do."
                    </p>
                </div>

                <div class="depth-story">
                    <h3>"Human input systems"</h3>
                    <p>
                        "Effort models scoring same-finger bigrams, scissors, lateral stretches, "
                        "and hand alternation across six keyboard layouts. Custom layout design "
                        "optimized for alternation."
                    </p>
                </div>
            </section>

            <section class="about__stack">
                <h2>"Stack"</h2>
                <p class="about__stack-line">
                    "Rust \u{b7} OCaml \u{b7} Go \u{b7} Python \u{b7} Swift \u{b7} TypeScript \u{b7} "
                    "Lean 4 \u{b7} Prolog \u{b7} Futhark \u{b7} MiniZinc"
                </p>
                <p class="about__stack-line about__stack-line--secondary">
                    "Axum \u{b7} Tokio \u{b7} Leptos \u{b7} Bevy \u{b7} egui \u{b7} SwiftUI \u{b7} "
                    "AWS \u{b7} Kubernetes \u{b7} Terraform \u{b7} Docker"
                </p>
            </section>

            <section class="about__education">
                <h2>"Education"</h2>
                <p>"B.S. in Computer Science \u{2014} Stetson University, 2011"</p>
            </section>
        </div>
    }
}
