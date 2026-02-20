use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::path;

use crate::components::footer::Footer;
use crate::components::nav::Nav;
use crate::pages::about::AboutPage;
use crate::pages::blog::{BlogListPage, BlogPostPage};
use crate::pages::contact::ContactPage;
use crate::pages::hire_me::HireMePage;
use crate::pages::home::HomePage;
use crate::pages::projects::ProjectsPage;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/stronglytyped-systems.css" />
        <Title text="stronglytyped.systems" />
        <Meta name="description" content="Software engineering portfolio and blog" />
        <Meta name="theme-color" content="#1E2326" />

        <Router>
            <div class="bg-blur" aria-hidden="true"></div>
            <div class="bg-overlay" aria-hidden="true"></div>
            <Nav />
            <main>
                <Routes fallback=|| "404 — not found.">
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/blog") view=BlogListPage />
                    <Route path=path!("/blog/:slug") view=BlogPostPage />
                    <Route path=path!("/projects") view=ProjectsPage />
                    <Route path=path!("/about") view=AboutPage />
                    <Route path=path!("/contact") view=ContactPage />
                    <Route path=path!("/hire-me") view=HireMePage />
                </Routes>
            </main>
            <Footer />
        </Router>
    }
}
