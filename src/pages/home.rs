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

            <div class="hero__thesis">
                <p>
                    "I build systems at the intersection of AI agent architecture, formal verification, "
                    "and human-computer interaction. My obsession is the interface between human intention "
                    "and physical output — whether that\u{2019}s keyboards, music, or programming languages."
                </p>
                <p>
                    "Currently on a deliberate sabbatical, building "
                    <a href="/projects" class="hero__link">"Coself"</a>
                    " — an AI system that combines probabilistic reasoning "
                    "with formal verification, maintains persistent memory, "
                    "and serves a single brain across multiple transport layers."
                </p>
            </div>

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

            <div class="hero__domains">
                <h2 class="section-heading">"What I Work On"</h2>
                <div class="domains-grid">
                    <div class="domain-card">
                        <h3 class="domain-card__title">"AI Agent Systems"</h3>
                        <p class="domain-card__body">
                            "Multi-gateway LLM orchestration, event-driven runtimes with typed dispatch, "
                            "persistent memory substrates, scoped program execution."
                        </p>
                    </div>
                    <div class="domain-card">
                        <h3 class="domain-card__title">"Systems Programming"</h3>
                        <p class="domain-card__body">
                            "Rust, OCaml, Go. Type-driven design, parse-don\u{2019}t-validate, exhaustive matching. "
                            "If the compiler can\u{2019}t prove it, write a test."
                        </p>
                    </div>
                    <div class="domain-card">
                        <h3 class="domain-card__title">"Infrastructure"</h3>
                        <p class="domain-card__body">
                            "AWS, Kubernetes, Terraform. CI/CD pipelines, multi-region deployments, "
                            "observability stacks."
                        </p>
                    </div>
                    <div class="domain-card">
                        <h3 class="domain-card__title">"Formal Methods"</h3>
                        <p class="domain-card__body">
                            "Lean 4 theorem proving, Prolog deduction, MiniZinc constraint optimization, "
                            "Futhark GPU-parallel computation. Proofs, not promises."
                        </p>
                    </div>
                </div>
            </div>

            <div class="hero__cta">
                <a href="/projects" class="btn btn--primary">"Projects"</a>
                <a href="/about" class="btn btn--outline">"About"</a>
                <a href="https://github.com/rdavison" class="btn btn--outline" target="_blank" rel="noopener noreferrer">"GitHub"</a>
            </div>
        </div>
    }
}
