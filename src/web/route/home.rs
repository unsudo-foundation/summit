use super::*;

#[component]
pub fn Home() -> Element {
    let device: Signal<_> = ::diogen::win::use_device();

    rsx!(
        ::diogen::layout::Page {
            scroll_snap: diogen::layout::PageScrollSnap::Proximity,
            style: r#"
                background: {color::OBSIDIAN};
            "#,
            ::diogen::layout::PageItem {
                top: rsx!(
                    Navbar {}
                ),
                if let Some(Ok(device)) = device() {
                    if device == ::diogen::win::Device::Laptop4K
                    || device == ::diogen::win::Device::LaptopL
                    || device == ::diogen::win::Device::LaptopS {
                        
                    } else {
                        ::diogen::layout::Col {
                            style: r#"
                                justify-content: start;
                                flex: 1;
                            "#,
                            ::diogen::layout::Col {
                                style: r#"
                                    align-items: start;
                                "#,
                                ::diogen::layout::Row {
                                    style: r#"
                                        font-size: 2em;
                                        font-family: br cobane;
                                        font-weight: bold;
                                        color: {color::SILVER};
                                    "#,
                                    "Trustless by Design"
                                }
                                ::diogen::layout::Row {
                                    style: r#"
                                        font-size: 1em;
                                        font-family: br cobane;
                                        font-weight: normal;
                                        color: {color::SILVER};
                                    "#,
                                    "Next gen DAOs"
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}

#[component]
fn Hero(
    children: Option<Element>
) -> Element {
    let device: Signal<_> = ::diogen::win::use_device();

    rsx!(
        ::diogen::layout::Row {
            style: r#"
                padding: 8px;
            "#,
            ::diogen::layout::Col {
                style: r#"
                    align-items: start;
                    gap: 16px;
                "#,
                ::diogen::layout::Col {
                    style: r#"
                        align-items: start;
                    "#,
                    ::diogen::layout::Row {
                        style: r#"
                            font-size: 1em;
                            font-family: br cobane;
                            font-weight: bold;

                        "#,
                        "Trustless by Design"
                    }
                    ::diogen::layout::Row {
                        style: r#"
                            font-size: 2em;
                            font-family: br cobane;
                            font-weight: normal;
                        "#,
                        "Next gen DAOs"
                    }
                }
                ::diogen::layout::Row {
                    style: r#"
                        gap: 8px;
                    "#,
                    HeroCtaButton { "Get Started" }
                    HeroCtaButton { "Learn More" }
                }
            }
            ::diogen::layout::Col {
                "IMAGE GOES HERE"
            }
        }
    )
}

#[component]
fn HeroCtaButton(
    children: Option<Element>
) -> Element {
    let border_color: Signal<&color::Color> = use_signal(|| &color::CARBON);
    
    rsx!(
        button {
            onmouseenter: {
                let mut border_color: Signal<_> = border_color.to_owned();
                move |_| {
                    border_color.set(&color::OFFICE_BLUE);
                }
            },
            onmouseleave: {
                let mut border_color: Signal<_> = border_color.to_owned();
                move |_| {
                    border_color.set(&color::CARBON);
                }
            },
            style: r#"
                all: unset;
                display: flex;
                flex-direction: row;
                justify-content: center;
                align-items: center;
                font-size: 2em;
                font-family: br cobane;
                font-weight: normal;
                border-width: 1px;
                border-style: solid;
                border-color: {border_color};
                border-radius: 2px;
                color: white;
                cursor: pointer;
                user-select: none;
                padding: 8px;
            "#,
            { children }
        }
    )
}


#[component]
fn Typography(
    font_size: Option<f64>,
    font_family: Option<&'static str>,
    font_weight: Option<&'static str>
) -> Element {
    let device: Signal<_> = ::diogen::win::use_device();

    rsx!(
        div {

        }
    )
}