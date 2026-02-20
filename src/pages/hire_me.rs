use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn HireMePage() -> impl IntoView {
    view! {
        <Title text="Hire Me — stronglytyped.systems" />
        <div class="hire-me" data-page="hire-me">
            <h1 class="page-title">"Hire Me"</h1>
            <p class="page-subtitle">"Open to full-time roles and consulting engagements."</p>

            <section class="hire-me__services">
                <h2>"What I Bring"</h2>
                <div class="hire-me__service-grid">
                    <div class="hire-me__service">
                        <h3>"AI Agent Architecture"</h3>
                        <p>"LLM orchestration, multi-channel agent systems, event-driven runtimes. I've built a production AI system from scratch — the hard parts, not just API wrappers."</p>
                    </div>
                    <div class="hire-me__service">
                        <h3>"Systems & Infrastructure"</h3>
                        <p>"Rust, Go, Python backends. AWS, Kubernetes, Terraform. 11+ years shipping production systems across adtech, genomics, and cloud infrastructure."</p>
                    </div>
                    <div class="hire-me__service">
                        <h3>"Full-Stack Development"</h3>
                        <p>"OCaml, TypeScript, React, Leptos. I've built full-stack applications in functional and imperative paradigms. Strong opinions on type safety, loosely held."</p>
                    </div>
                </div>
            </section>

            <section class="hire-me__availability">
                <h2>"Availability"</h2>
                <p>
                    "Currently on sabbatical and open to the right opportunity. "
                    "Reach out via "<a href="/contact">"the contact page"</a>" or connect on "
                    <a href="https://linkedin.com/in/richardndavison" target="_blank" rel="noopener noreferrer">"LinkedIn"</a>"."
                </p>
            </section>
        </div>
    }
}
