use sycamore::prelude::*;





#[component(footerWidget<G>)]
#[perseus::template_rx]
pub fn footer_widget(state: Struct) -> View<G> {

    view! {
        body  {footer  {nav (role="navigation") {ul (class="nav-ul") {li  {a (href="#") {Home} 
        } 

                                                                      li  {a (href="#") {About Us} 
                                                                      } 

                                                                      li  {a (href="#") {Technologies} 
                                                                      } 

                                                                      li  {a (href="#") {Projects} 
                                                                      } 

                                                                      li  {a (href="#") {Contacts} 
                                                                      } 

        } 
        } 
                        p (class="copy") {©  2014} 

        } 
        } 

    }


}

/*
#[component(ShowMenu<G>)]
pub fn show_menus(menulist: Vec<Menu>) -> Template<G> {
let list = Template::new_fragment(
menulist
.into_iter()
.map(|item| {
template! {
MenuWidget(item)
                }
            })
            .collect(),
    );
    template! {
        ul(class="nav navbar-nav  navbar-right") {
            (list)
        }
    }
}

#[component(Nav<G>)]
pub fn nav() -> Template<G> {
    let is_loading = Signal::new(true);
    let set_loading = is_loading.clone();

    let menu_list = Signal::new(vec![]);
    let set_menus = menu_list.clone();

    let future_menus = async { get_menu_list().await };

    handle_future(future_menus, move |data: Result<Vec<Menu>, Status>| {
        if let Ok(menus) = data {
            set_menus.set(menus)
        };
        set_loading.set(false);
    });

    template! {
            nav(class="navbar  navbar-expand-md ") {a(class="navbar-brand", href="/") {"Home"}
            span(class="navbar-toggler-icon") {}
          div(class="collapse navbar-collapse", id="navbarsExampleDefault") {
              (if *is_loading.get() {
                template! { h1() {"x"} }
            } else {
                template! {
                     ShowMenu((*menu_list.get()).clone())
    } // Now you don't
            })

          }
        }
            }
}

*/
