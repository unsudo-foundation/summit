use super::*;
use route::home::Home as HomeRoute;

mod form;
mod route;

::modwire::expose!(
    navbar
    icon
    redirect_icon
    redirect
);

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    HomeRoute {},
    #[route("/control-center")]
    ControlCenter {}
}

#[component]
fn Sale() -> Element {
    rsx!(

    )
}


#[component]
fn ControlCenter() -> Element {
    let selected: Signal<Option<&'static str>> = use_signal(|| None);
    let toggled: Signal<_> = use_signal(|| false);

    rsx!(
        ::diogen::layout::Page {
            scroll_snap: diogen::layout::PageScrollSnap::Proximity,
            style: format!(
                r#"
                    background: {};
                    color: {};
                "#,
                color::OBSIDIAN,
                color::SILVER
            ),
            ::diogen::layout::PageItem {
                top: rsx!(
                    Navbar {}
                ),
                bottom: rsx!(
                    div {}
                ),
                form::Root {
                    
                }
            }
        }
    )
}

mod color {
    pub use ::diogen::color::Color;

    pub static OBSIDIAN: Color = Color::from_hex(0x121212);
    pub static CARBON: Color = Color::from_hex(0x202020);
    pub static JET: Color = Color::from_hex(0x404040);
    pub static AQUA: Color = Color::from_hex(0x357ded);
    pub static OFFICE_BLUE: Color = Color::from_hex(0x383f51);
    pub static STEEL: Color = Color::from_hex(0x9d9d9c);
    pub static SILVER: Color = Color::from_hex(0xefe6dd);
    pub static SPRING: Color = Color::from_hex(0x00a676);
    pub static HONEY: Color = Color::from_hex(0xff8000);
    pub static IMPERIAL_RED: Color = Color::from_hex(0xff0004);
}

mod marker {
    //! Marks incomplete or problematic sections without breaking the interface.
    //! Lets users and developers continue using the app safely.
    
    use super::*;

    #[component]
    pub fn NotImplemented(
        class: Option<&'static str>,
        style: Option<&'static str>,
        children: Option<Element>
    ) -> Element {
        rsx!(
            div {
                class,
                style: r#"
                    font-size: 1em;
                    font-family: br cobane;
                    font-weight: normal;
                    color: {color::HONEY};
                    {style.to_owned().unwrap_or_default()}
                "#,
                if let Some(children) = children {
                    span {
                        span { "NotImplemented: " }
                        span { { children } }
                    }
                } else {
                    "NotImplemented"
                }
            }
        )
    }
}