use sycamore::prelude::*;

#[component(MenuWidget<G>)]
pub fn MenuWidget<G: Html>(cx: Scope) -> View<G> {
    view! {cx,
    div (class="wrapper") {
                header (role="banner") {nav (role="navigation") {h1  {a (href="#") {"Asperion"}
                }

                                                                 ul (class="nav-ul") {li  {a (href="#") {"Home"}
                                                                 }

                                                                                      li  {a (href="#") {"About Us"}
                                                                                      }

                                                                                      li  {a (href="#") {"Technologies"}
                                                                                      }

                                                                                      li  {a (href="#") {"Projects"}
                                                                                      }

                                                                                      li  {a (href="#") {"Contacts"}
                                                                                      }

                                                                 }
                }
                }
                main (role="main") {}
            }
            }
}
