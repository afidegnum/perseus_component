use perseus::Template;
use sycamore::prelude::{view, Html, Scope, SsrNode, View};

#[perseus::template_rx]
pub fn about_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
            p { "About111." }
            p { "About.222" }
            p { "Abou4444t." }

                 div (class="line") {
            p { "Aboutdddd." }

    }

        }
}

#[perseus::head]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "About Page | Perseus Example – Basic" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("about").template(about_page).head(head)
}
