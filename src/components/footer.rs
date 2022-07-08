use sycamore::prelude::*;

#[component(FooterWidget<G>)]
pub fn FooterWidget<G: Html>(cx: Scope) -> View<G> {
    view! {cx,

    footer  {nav (role="navigation") {ul (class="nav-ul") {li  {a (href="#") {"Home"}
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
                            p (class="copy") { " 2014"}

            }


        }
}
