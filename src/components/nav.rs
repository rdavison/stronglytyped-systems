use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;

#[component]
pub fn Nav() -> impl IntoView {
    let location = use_location();
    let pathname = move || location.pathname.get();

    let nav_items: Vec<(&str, &str, &str)> = vec![
        ("/", "Home", "home"),
        ("/blog", "Blog", "blog"),
        ("/projects", "Projects", "projects"),
        ("/about", "About", "about"),
        ("/contact", "Contact", "contact"),
        ("/hire-me", "Hire Me", "hire-me"),
    ];

    view! {
        <nav class="nav">
            <div class="nav__inner">
                <ul class="nav__tabs">
                    {nav_items
                        .into_iter()
                        .map(|(href, label, page)| {
                            let href_owned = href.to_string();
                            let is_active = move || {
                                let path = pathname();
                                if href_owned == "/" {
                                    path == "/"
                                } else {
                                    path.starts_with(&href_owned)
                                }
                            };
                            view! {
                                <li class="nav__tab" data-page=page>
                                    <A
                                        href=href
                                        attr:class=move || {
                                            if is_active() { "nav__link nav__link--active" } else { "nav__link" }
                                        }
                                    >
                                        {label}
                                    </A>
                                </li>
                            }
                        })
                        .collect::<Vec<_>>()}
                </ul>
            </div>
        </nav>
    }
}
