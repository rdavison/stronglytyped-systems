use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text="stronglytyped.systems — Richard Davison" />
        <div class="hero" data-page="home">
            <div class="hero__identity">
                <h1 class="hero__name">"Richard Davison"</h1>
                <p class="hero__role">"Software Engineer — AI Systems & Infrastructure"</p>
                <p class="hero__location">"New York, NY"</p>
            </div>

            <p class="hero__tagline">
                "The interface between human intention and physical output — keyboards, music, programming languages."
            </p>

            <div class="hero__separator" aria-hidden="true"></div>

            <div class="hero__timeline">
                <h2 class="section-heading">"Timeline"</h2>
                <div class="timeline">
                    <div class="timeline__entry">
                        <span class="timeline__date">"2025 –"</span>
                        <div class="timeline__content">
                            <span class="timeline__title">"Sabbatical & Independent Research"</span>
                            <span class="timeline__detail">"Coself, keyboard analyzers, game engines, formal methods. Rust, Lean 4, SwiftUI."</span>
                        </div>
                    </div>
                    <div class="timeline__entry">
                        <span class="timeline__date">"2024 – 25"</span>
                        <div class="timeline__content">
                            <span class="timeline__title">"Lambda — Senior Software Engineer"</span>
                            <span class="timeline__detail">"Multi-region API gateway, OAuth2/OIDC modernization. Go, Python, Terraform, K8s."</span>
                        </div>
                    </div>
                    <div class="timeline__entry">
                        <span class="timeline__date">"2017 – 24"</span>
                        <div class="timeline__content">
                            <span class="timeline__title">"Solvuu — Full Stack Engineer"</span>
                            <span class="timeline__detail">"Genomics analysis platforms, OCaml full-stack, AWS infrastructure. Seven years."</span>
                        </div>
                    </div>
                    <div class="timeline__entry">
                        <span class="timeline__date">"2014 – 17"</span>
                        <div class="timeline__content">
                            <span class="timeline__title">"Sonobi — Software Engineer"</span>
                            <span class="timeline__detail">"Real-time bidding, data pipelines, monitoring. Python, Airflow, InfluxDB."</span>
                        </div>
                    </div>
                </div>
            </div>

            <div class="hero__separator" aria-hidden="true"></div>

            <div class="hero__proof-lines">
                <a href="/projects" class="proof-line">
                    <span class="proof-line__domain">"AI Agents"</span>
                    <span class="proof-line__claim">"One brain serving Discord, WhatsApp, and HTTP — with Lean 4 proofs verifying behavioral invariants"</span>
                </a>
                <a href="/projects" class="proof-line">
                    <span class="proof-line__domain">"Systems"</span>
                    <span class="proof-line__claim">"Event-driven runtime with typed dispatch, scoped programs, and hot-reloadable architecture in Rust"</span>
                </a>
                <a href="/projects" class="proof-line">
                    <span class="proof-line__domain">"Input"</span>
                    <span class="proof-line__claim">"Custom effort models scoring SFBs, scissors, and lateral stretches across 6 keyboard layouts at 50fps"</span>
                </a>
                <a href="/projects" class="proof-line">
                    <span class="proof-line__domain">"Formal"</span>
                    <span class="proof-line__claim">"Prolog deduction, MiniZinc constraint optimization, and Futhark GPU computation — not just types, theorems"</span>
                </a>
            </div>

            <div class="hero__cta">
                <a href="/projects" class="btn btn--primary">"Projects"</a>
                <a href="/about" class="btn btn--outline">"About"</a>
                <a href="https://github.com/rdavison" class="btn btn--outline" target="_blank" rel="noopener noreferrer">"GitHub"</a>
            </div>
        </div>
    }
}
