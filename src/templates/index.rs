use perseus::{make_rx, Html, RenderFnResultWithCause, SsrNode, Template};
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use sycamore::prelude::{view, View};

use crate::components::menu::MenuWidget;

#[perseus::template_rx]
pub fn index_page<G: Html>(cx: Scope) -> View<G> {
    view! { cx,

                div (class="line") {
    MenuWidget()
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
