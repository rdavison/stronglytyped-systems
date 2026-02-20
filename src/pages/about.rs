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
                    "I play bassoon at an orchestral level, speak six languages at varying degrees of fluency, "
                    "and solve Rubik\u{2019}s cubes competitively. I believe intelligence is built upon language "
                    "through all layers \u{2014} it\u{2019}s languages all the way down."
                </p>
                <p>
                    "My through-line is the interface between human intention and physical output. "
                    "It manifests in three domains I keep returning to: keyboards and input systems, "
                    "music and composition, and programming languages. Three domains, one obsession."
                </p>
                <p>
                    "11+ years shipping production systems across adtech, genomics, cloud infrastructure, and AI. "
                    "I think in type systems and composition laws. I see structure where others see features."
                </p>
            </section>

            <section class="about__depths">
                <h2>"Where I Go Deep"</h2>

                <div class="depth-story">
                    <h3>"Formal verification in production"</h3>
                    <p>
                        "I use Lean 4 not as a resume line but as a reasoning engine \u{2014} "
                        "Coself proves behavioral invariants about its own agent architecture before execution. "
                        "Alongside Prolog for deductive queries, MiniZinc for constraint optimization, "
                        "and Futhark for GPU-parallel computation, the system makes decisions that are "
                        "formally verified, not just tested."
                    </p>
                </div>

                <div class="depth-story">
                    <h3>"Type-driven systems in Rust and OCaml"</h3>
                    <p>
                        "Seven years of OCaml at Solvuu taught me to lean on the type system as a design tool, "
                        "not just a safety net. Parse-don\u{2019}t-validate, exhaustive matching, newtype wrappers \u{2014} "
                        "if the compiler can\u{2019}t prove a property, I write a test. If it can, I don\u{2019}t. "
                        "I brought this discipline into Rust when I built Coself\u{2019}s event runtime."
                    </p>
                </div>

                <div class="depth-story">
                    <h3>"Human input systems"</h3>
                    <p>
                        "Keyboards fascinate me because they\u{2019}re the tightest loop between thought and output. "
                        "I\u{2019}ve built effort models that score same-finger bigrams, scissors, lateral stretches, "
                        "and hand alternation across six layouts. I designed a custom layout optimized for alternation. "
                        "The same obsession extends to music (bassoon technique is a physical interface problem) "
                        "and IPA phonetics (how do mouths produce language?)."
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
