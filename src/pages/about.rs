use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <Title text="About — stronglytyped.systems" />
        <div class="about" data-page="about">
            <h1 class="page-title">"About"</h1>

            <section class="about__bio">
                <p>
                    "I'm Richard — a software engineer in NYC with 11+ years of experience shipping "
                    "production systems across adtech, genomics, cloud infrastructure, and AI. "
                    "I think in type systems and composition laws. I see structure where others see features."
                </p>
                <p>
                    "My through-line is the interface between human intention and physical output. "
                    "It manifests in three domains I keep returning to: keyboards and input systems, "
                    "music and composition, and programming languages. Three domains, one obsession."
                </p>
                <p>
                    "I play bassoon at an orchestral level, speak six languages at varying degrees of fluency, "
                    "and solve Rubik's cubes competitively. I believe intelligence is built upon language "
                    "through all layers — it's languages all the way down."
                </p>
            </section>

            <section class="about__skills">
                <h2>"Technical Skills"</h2>
                <div class="about__skill-grid">
                    <div class="about__skill-group">
                        <h3>"Languages"</h3>
                        <ul>
                            <li>"Rust"</li>
                            <li>"Python"</li>
                            <li>"TypeScript"</li>
                            <li>"Go"</li>
                            <li>"OCaml"</li>
                            <li>"Swift"</li>
                        </ul>
                    </div>
                    <div class="about__skill-group">
                        <h3>"AI / LLM"</h3>
                        <ul>
                            <li>"Agent architecture"</li>
                            <li>"LLM orchestration"</li>
                            <li>"Prompt engineering"</li>
                            <li>"Claude API"</li>
                        </ul>
                    </div>
                    <div class="about__skill-group">
                        <h3>"Infrastructure"</h3>
                        <ul>
                            <li>"AWS (VPC, EC2, IAM, S3)"</li>
                            <li>"Kubernetes / Docker"</li>
                            <li>"Terraform / Ansible"</li>
                            <li>"CI/CD Pipelines"</li>
                        </ul>
                    </div>
                    <div class="about__skill-group">
                        <h3>"Formal Methods"</h3>
                        <ul>
                            <li>"Lean 4"</li>
                            <li>"Prolog"</li>
                            <li>"MiniZinc"</li>
                            <li>"Futhark"</li>
                        </ul>
                    </div>
                    <div class="about__skill-group">
                        <h3>"Data"</h3>
                        <ul>
                            <li>"Postgres / MySQL"</li>
                            <li>"InfluxDB / Redshift"</li>
                            <li>"Airflow / RabbitMQ"</li>
                            <li>"Snowflake"</li>
                        </ul>
                    </div>
                    <div class="about__skill-group">
                        <h3>"Frontend"</h3>
                        <ul>
                            <li>"Leptos / WASM"</li>
                            <li>"React / ReScript"</li>
                            <li>"SwiftUI"</li>
                            <li>"Tailwind CSS"</li>
                        </ul>
                    </div>
                </div>
            </section>

            <section class="about__human">
                <h2>"Human Skills"</h2>
                <div class="about__skill-grid">
                    <div class="about__skill-group">
                        <h3>"Music"</h3>
                        <ul>
                            <li>"Bassoon (orchestral)"</li>
                            <li>"Composition"</li>
                            <li>"Music theory"</li>
                        </ul>
                    </div>
                    <div class="about__skill-group">
                        <h3>"Languages"</h3>
                        <ul>
                            <li>"English (native)"</li>
                            <li>"Spanish (fluent)"</li>
                            <li>"Italian, Portuguese"</li>
                            <li>"Latin, Mandarin"</li>
                        </ul>
                    </div>
                    <div class="about__skill-group">
                        <h3>"Other"</h3>
                        <ul>
                            <li>"Speedcubing"</li>
                            <li>"Keyboard layout design"</li>
                            <li>"Category theory"</li>
                        </ul>
                    </div>
                </div>
            </section>

            <section class="about__education">
                <h2>"Education"</h2>
                <p>"B.S. in Computer Science — Stetson University, 2011"</p>
            </section>
        </div>
    }
}
