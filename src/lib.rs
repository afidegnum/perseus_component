mod components;
mod error_pages;
mod templates;
use crate::components::menu::MenuWidget;
use std::collections::HashMap;

use std::path::Path;
use walkdir::WalkDir;

use perseus::{Html, PerseusApp, PerseusRoot};
#[perseus::main(perseus_actix_web::dflt_server)]

pub fn main<G: Html>() -> PerseusApp<G> {
    let mut static_paths: HashMap<String, String> = HashMap::new();

    let pathstr = env!("CARGO_MANIFEST_DIR");
    let p = Path::new(pathstr);
    let target = p.join("static");
    let rp = target.file_name().unwrap();

    for x in WalkDir::new(&target)
        .into_iter()
        .flatten()
        .filter(|e| e.file_type().is_file())
    {
        let a = x
            .path()
            .strip_prefix(&target)
            .unwrap()
            .to_str()
            .expect("invalid utf8 in path")
            .to_owned();
        let k = Path::new("/").join(a.clone());

        let bf = Path::new(rp).join(a.clone());

        static_paths.insert(k.display().to_string(), bf.display().to_string());
    }

    PerseusApp::new()
        .template(crate::templates::index::get_template)
  .template(crate::templates::about::get_template)
        .error_pages(crate::error_pages::get_error_pages)
        .static_aliases(static_paths)
        //.static_alias("style.css", "/style.css")
            .index_view(|cx| {
                sycamore::view! { cx,

                    head {

                        link (rel="stylesheet", href="https://cdnjs.cloudflare.com/ajax/libs/meyer-reset/2.0/reset.min.css")
    link (rel="stylesheet", href="https://fonts.googleapis.com/css?family=Roboto+Condensed:400,700,300")
    link (rel="stylesheet", href="https://fonts.googleapis.com/css?family=Open+Sans+Condensed:300,700")
    link (rel="stylesheet", href="https://fonts.googleapis.com/css?family=Source+Sans+Pro:300,400,700")
    link (rel="stylesheet", href="style.css") {}
                    }
                    body {

                        // This creates an element into which our app will be interpolated
                        // This uses a few tricks internally beyond the classic `<div id="root">`, so we use this wrapper for convenience
    MenuWidget()
                         PerseusRoot()

                        // Because this is in the index view, this will be below every single one of our pages
                        // Note that elements in here can't be selectively removed from one page, it's all-or-nothing in the index view (it wraps your whole app)
                        // Note also that this won't be reloaded, even when the user switches pages
                         footer { "This is a footer!" }
                    }
                }
            })
}
