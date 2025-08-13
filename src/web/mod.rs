use super::*;

::modwire::expose!(
    navbar
);

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {}
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