use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="footer">
            <div class="footer__inner">
                <p class="footer__copy">"stronglytyped.systems"</p>
                <p class="footer__built">"Built with Rust, Leptos, and zero JavaScript frameworks."</p>
            </div>
        </footer>
    }
}
