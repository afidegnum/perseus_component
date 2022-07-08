use crate::components::footer::FooterWidget;
use perseus::{make_rx, Html, RenderFnResultWithCause, SsrNode, Template};
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use sycamore::prelude::{view, View};

// #[perseus::template_rx]
#[perseus::template(IndexPage)]
#[component(IndexPage<G>)]
pub fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,

                div (class="line") {
    FooterWidget()
        }
                div (class="line") {}
            }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index").template(index_page).head(head)
}

#[perseus::head]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Index Page | Perseus Example – Basic" }
    }
}
