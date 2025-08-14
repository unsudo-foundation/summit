use super::*;

mod form;
mod typography;

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
    pub static OBSIDIAN: &str = "#121212";
    pub static CARBON: &str = "#202020";
    pub static JET: &str = "#404040";
    pub static AQUA: &str = "#357DED";
    pub static OFFICE_BLUE: &str = "#383F51";
    pub static STEEL: &str = "#9D9D9C";
    pub static SILVER: &str = "#EFE6DD";
    pub static SPRING: &str = "#00A676";
    pub static HONEY: &str = "#FF8000";
    pub static IMPERIAL_RED: &str = "#FF0004";

    
    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(PartialEq)]
    pub enum Color {
        Hex(u32)
    }

    impl From<u32> for Color {
        fn from(value: u32) -> Self {
            if value > 0xffffff {
                panic!("[ABORT] Value out of 24-bit RGB range: 0x{:X}", value);
            }
            Self::Hex(value)
        }
    }

    impl ::std::fmt::Display for Color {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", match self {
                Self::Hex(code) => format!("#{}", code)
            })
        }
    }


    pub fn interpolate(range: (&str, &str), t: f32) -> String {
        let (rx, gx, bx) = hex_to_rgb(range.0);
        let (ry, gy, by) = hex_to_rgb(range.1);
        let r = rx as f32 + (ry as f32 - rx as f32) * t;
        let r = r.round() as u8;
        let g = gx as f32 + (gy as f32 - gx as f32) * t;
        let g = g.round() as u8;
        let b = bx as f32 + (by as f32 - bx as f32) * t;
        let b = b.round() as u8;
        rgb_to_hex(r, g, b)
    }

    fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }

    fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap();
        (r, g, b)
    }
}


mod easing {
    pub fn ease_in(p: f32, s: f32, c: f32, d: f32) -> f32 {
        c *
        (p / d) *
        (p / d) + s
    }

    pub fn ease_in_cubic(p: f32, s: f32, c: f32, d: f32) -> f32 {
        c *
        (p / d) *
        (p / d) *
        (p / d) + s
    }

    pub fn ease_in_quart(p: f32, s: f32, c: f32, d: f32) -> f32 {
        c *
        (p / d) *
        (p / d) *
        (p / d) * 
        (p / d) + s
    }

    pub fn ease_in_quint(p: f32, s: f32, c: f32, d: f32) -> f32 {
        c * 
        (p / d) *
        (p / d) *
        (p / d) *
        (p / d) *
        (p / d) + s
    }


    pub fn ease_out(p: f32, s: f32, c: f32, d: f32) -> f32 {
        -c * (p / d) * ((p / d) - 2.0f32) + s
    }

    pub fn ease_out_expo(p: f32, s: f32, c: f32, d: f32) -> f32 {
        if p == d {
            return s + c
        }
        let k: f32 = -2f32.powf(-10.0f32 * p / d) + 1.0f32;
        c * k + s
    }

    pub fn ease_out_cubic(p: f32, s: f32, c: f32, d: f32) -> f32 {
        c * (
            (p / d - 1.0f32) *
            (p / d - 1.0f32) *
            (p / d - 1.0f32) + 1.0f32
        ) + s
    }

    pub fn ease_out_quart(p: f32, s: f32, c: f32, d: f32) -> f32 {
        -c * (
            (p / d - 1.0f32) *
            (p / d - 1.0f32) *
            (p / d - 1.0f32) *
            (p / d - 1.0f32) - 1.0f32
        ) + s
    }

    pub fn ease_out_quint(p: f32, s: f32, c: f32, d: f32) -> f32 {
        c * (
            (p / d - 1.0f32) *
            (p / d - 1.0f32) *
            (p / d - 1.0f32) *
            (p / d - 1.0f32) *
            (p / d - 1.0f32) + 1.0f32
        ) + s
    }
}


mod rho {
    pub fn from(k: u16) -> f64 {
        let base: f64 = 8.0;
        let ratio: f64 = 1.618f64;
        let value: f64 = base * ratio.powf(k as f64);
        value.round()
    }
}

mod direction {
    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(PartialEq)]
    pub enum Direction {

    }
}

mod unit {
    // TODO move to diogen.

    pub trait Measurable 
    where
        Self: ::std::fmt::Display {}
    impl Measurable for Absolute {}
    impl Measurable for Relative {}
    impl Measurable for Viewport {}
    impl Measurable for Angle {}
    impl Measurable for Time {}
    impl Measurable for Frequency {}
    impl Measurable for Resolution {}


    #[derive(Clone)]
    #[derive(PartialEq)]
    pub struct Unit<T> 
    where
        T: Measurable {
        pub measurement: T,
        pub n: f64
    }

    impl<T> From<(T, f64)> for Unit<T> 
    where
        T: Measurable {
        fn from(value: (T, f64)) -> Self {
            Self {
                measurement: value.0,
                n: value.1
            }
        }
    }


    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(PartialEq)]
    #[derive(::strum_macros::EnumString)]
    #[derive(::strum_macros::Display)]
    #[strum(serialize_all = "lowercase")]
    pub enum Absolute {
        Px,
        Cm,
        Mm,
        Q,
        In,
        Pc,
        Pt
    }

    impl Absolute {
        pub fn into_unit(self, n: f64) -> Unit<Self> {
            Unit {
                measurement: self,
                n
            }
        }
    }


    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(PartialEq)]
    #[derive(::strum_macros::EnumString)]
    #[derive(::strum_macros::Display)]
    #[strum(serialize_all = "lowercase")]
    pub enum Relative {
        Em,
        Ex,
        Ch,
        Rem,
        Lh,
        Rlh
    }

    impl Relative {
        pub fn into_unit(self, n: f64) -> Unit<Self> {
            Unit {
                measurement: self,
                n
            }
        }
    }


    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(PartialEq)]
    #[derive(::strum_macros::EnumString)]
    #[derive(::strum_macros::Display)]
    #[strum(serialize_all = "lowercase")]
    pub enum Viewport {
        Vw,
        Vh,
        Vmin,
        Vmax,
        Vb,
        Vi,
        Svw,
        Svh,
        Lvw,
        Lvh,
        Dvw,
        Dvh
    }

    impl Viewport {
        pub fn into_unit(self, n: f64) -> Unit<Self> {
            Unit {
                measurement: self,
                n
            }
        }
    }


    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(PartialEq)]
    #[derive(::strum_macros::EnumString)]
    #[derive(::strum_macros::Display)]
    #[strum(serialize_all = "lowercase")]
    pub enum Angle {
        Grad,
        Turn,
        Deg,
        Rad
    }

    impl Angle {
        pub fn into_unit(self, n: f64) -> Unit<Self> {
            Unit {
                measurement: self,
                n
            }
        }
    }


    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(PartialEq)]
    #[derive(::strum_macros::EnumString)]
    #[derive(::strum_macros::Display)]
    #[strum(serialize_all = "lowercase")]
    pub enum Time {
        S,
        Ms
    }

    impl Time {
        pub fn into_unit(self, n: f64) -> Unit<Self> {
            Unit {
                measurement: self,
                n
            }
        }
    }


    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(PartialEq)]
    #[derive(::strum_macros::EnumString)]
    #[derive(::strum_macros::Display)]
    #[strum(serialize_all = "lowercase")]
    pub enum Frequency {
        Hz,
        Khz
    }

    impl Frequency {
        pub fn into_unit(self, n: f64) -> Unit<Self> {
            Unit {
                measurement: self,
                n
            }
        }
    }


    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(PartialEq)]
    #[derive(::strum_macros::EnumString)]
    #[derive(::strum_macros::Display)]
    #[strum(serialize_all = "lowercase")]
    pub enum Resolution {
        Dpi,
        Dpcm,
        Dppx
    }

    impl Resolution {
        pub fn into_unit(self, n: f64) -> Unit<Self> {
            Unit {
                measurement: self,
                n
            }
        }
    }
}