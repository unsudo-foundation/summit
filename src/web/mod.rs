use super::*;

mod form;

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
    Home {},
    #[route("/control-center")]
    ControlCenter {}
}

#[component]
fn Home() -> Element {
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
                ::diogen::layout::Col {
                    style: format!(
                        r#"
                            justify-content: start;
                            flex: 1;
                        "#
                    ),
                    form::Root {
                        style: format!(
                            r#"
                                min-width: 200px;
                                
                            "#
                        ),
                        form::Dropdown {
                            form::DropdownLabel { "Hello World" }
                            form::DropdownSelect {
                                form::DropdownOption { "Option" }
                                form::DropdownOption { "HI" }
                            }
                        }
                    }
                    ::diogen::layout::Col {
                        style: format!(
                            r#"
                                min-width: 100%;
                                min-height: 10px;
                                border-width: 1px;
                                border-style: solid;
                                border-color: {};
                                border-radius: 2px;
                            "#,
                            color::CARBON
                        ),
                        // ICON GROUP
                        ::diogen::layout::Row {
                            style: format!(
                                r#"
                                    justify-content: start;
                                    gap: 8px;
                                "#
                            ),
                            for (w, url, to) in vec![
                                ("15px", asset!("asset/icon/social/discord.svg"), "/"),
                                ("15px", asset!("asset/icon/social/github.svg"), "/"),
                                ("15px", asset!("asset/icon/social/medium.svg"), "/"),
                                ("15px", asset!("asset/icon/social/telegram.svg"), "/"),
                                ("15px", asset!("asset/icon/social/x.svg"), "/")
                            ] {
                                RedirectIcon { w, url, to }
                            }                            
                        }
                        ::diogen::layout::Row {
                            style: format!(
                                r#"
                                    font-size: 2em;
                                    font-family: br cobane;
                                    font-weight: bold;
                                    background: linear-gradient(
                                        to right bottom,
                                        {},
                                        {}
                                    );
                                    -webkit-background-clip: text;
                                    -webkit-text-fill-color: transparent;
                                    background-clip: text;
                                    color: transparent;
                                "#,
                                color::SILVER,
                                color::STEEL
                            ),
                            "Trustless by Design. Ruthless in Reliability."
                        }
                    }
                }
            }
        }
    )
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