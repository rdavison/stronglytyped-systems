use leptos::prelude::*;
use leptos_meta::*;

#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        <Title text="Contact — stronglytyped.systems" />
        <div class="contact" data-page="contact">
            <h1 class="page-title">"Contact"</h1>
            <p class="page-subtitle">"Want to chat? Here's where to find me."</p>

            <div class="contact__links">
                <a href="https://github.com/rdavison" class="contact__link" target="_blank" rel="noopener noreferrer">
                    <span class="contact__link-label">"GitHub"</span>
                    <span class="contact__link-value">"github.com/rdavison"</span>
                </a>
                <a href="mailto:richardneildavison@gmail.com" class="contact__link">
                    <span class="contact__link-label">"Email"</span>
                    <span class="contact__link-value">"richardneildavison@gmail.com"</span>
                </a>
                <a href="https://linkedin.com/in/richardndavison" class="contact__link" target="_blank" rel="noopener noreferrer">
                    <span class="contact__link-label">"LinkedIn"</span>
                    <span class="contact__link-value">"linkedin.com/in/richardndavison"</span>
                </a>
            </div>
        </div>
    }
}
