use super::*;

#[component]
pub fn Navbar() -> Element {
    let device: Signal<_> = ::diogen::win::use_device();

    rsx!(
        div {
            style: r#"
                display: flex;
                flex-direction: row;
                justify-content: space-between;
                align-items: center;
                min-width: 100%;
                padding-left: 16px;
                padding-right: 16px;
                padding-top: 8px;
                padding-bottom: 8px;
            "#,
            div {
                style: r#"
                    display: flex;
                    flex-direction: row;
                    justify-content: center;
                    align-items: center;
                    gap: 32px;
                "#,
                if let Some(Ok(device)) = device() {
                    if device == ::diogen::win::Device::Laptop4K
                    || device == ::diogen::win::Device::LaptopL
                    || device == ::diogen::win::Device::LaptopS {
                        LaptopLogo {}
                        LaptopLinkGroup {}
                    } else {
                        MobileLogo {}
                    }
                }
            }
            div {
                style: r#"
                    display: flex;
                    flex-direction: row;
                    justify-content: center;
                    align-items: center;
                "#,
                if let Some(Ok(device)) = device() {
                    if device == ::diogen::win::Device::Tablet
                    || device == ::diogen::win::Device::MobileL
                    || device == ::diogen::win::Device::MobileM
                    || device == ::diogen::win::Device::MobileS {
                        MobileBurgerBarToggle {}
                    }
                }
            }
        }
    )
}

#[component]
pub fn LaptopLogo(
    class: Option<String>,
    style: Option<String>
) -> Element {
    rsx!(
        Link {
            to: "/",
            style: r#"
                all: unset;
                display: contents;
            "#,
            ::diogen::layout::Row {
                class,
                style: r#"
                    font-family: alien skyline;
                    font-size: 3em;
                    font-weight: normal;
                    color: {color::SILVER};
                    user-select: none;
                    cursor: pointer;
                    {style.to_owned().unwrap_or_default()}
                "#,
                "unSUDO"
            }
        }
    )
}

#[component]
pub fn MobileLogo(
    class: Option<String>,
    style: Option<String>
) -> Element {
    rsx!(
        LaptopLogo {
            class,
            style: r#"
                font-size: 2em;
            "#
        }
    )
}

#[component]
pub fn LaptopLinkButton(
    to: String,
    class: Option<String>,
    style: Option<String>,
    children: Option<Element>
) -> Element {
    rsx!(
        Link {
            to,
            style: r#"
                all: unset;
                display: contents;
            "#,
            button {
                class,
                style: r#"
                    all: unset;
                    display: flex;
                    flex-direction: row;
                    justify-content: center;
                    align-items: center;
                    cursor: pointer;
                    user-select: none;
                    font-size: 0.8em;
                    font-family: br cobane;
                    font-weight: bold;
                    color: {color::SILVER};
                    {style.to_owned().unwrap_or_default()}
                "#,
                { children }
            }
        }
    )
}

#[component]
pub fn LaptopLinkGroup() -> Element {
    rsx!(
        div {
            style: r#"
                display: flex;
                flex-direction: row;
                justify-content: center;
                align-items: center;
                gap: 32px;
            "#,
            LaptopLinkButton { to: "/", "Whitepaper" }
            LaptopLinkButton { to: "/", "About" }
            LaptopLinkButton { to: "/", "Roadmap" }
            LaptopLinkButton { to: "/", "Community" }
        }
    )
}

#[component]
pub fn MobileBurgerBarToggle() -> Element {
    let toggled: Signal<bool> = use_signal(|| false);

    rsx!(

    )
}