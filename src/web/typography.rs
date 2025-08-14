use super::*;

// TODO move to diogen.

#[repr(u8)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Alignment {
    Left,
    Center,
    Justify,
    Right
}

#[repr(u8)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Decor {
    Solid,
    Double,
    Dotted,
    Dashed,
    Wavy
}

#[repr(u8)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Family {
    BrCobane
}

#[repr(u8)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Weight {
    Normal,
    Bold,
    Bolder,
    Lighter
}

#[derive(Clone)]
#[derive(PartialEq)]
pub enum WhiteSpace {
    Normal,
    NoWrap,
    Pre,
    PreWrap,
    PreLine
}

#[derive(Clone)]
#[derive(PartialEq)]
pub enum OverflowWrap {
    Normal,
    BreakWord,
    Anywhere
}

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct TypographyProps {
    pub size: Option<unit::Unit<unit::Relative>>,
    pub family: Option<Family>,
    pub weight: Option<Weight>,
    pub color_direction: Option<direction::Direction>,
    pub colors: Option<Vec<color::Color>>,
    pub letter_spacing: Option<unit::Unit<unit::Relative>>,


    pub word_spacing: Option<&'static str>,


    pub decor: Option<Decor>,
    pub decoration_color: Option<&'static str>,
    pub decoration_style: Option<&'static str>,


    pub white_space: Option<WhiteSpace>,

    pub overflow_wrap: Option<OverflowWrap>,
    pub align: Option<Alignment>,
    pub shadow: Option<&'static str>
}

#[component]
pub fn Typography(props: TypographyProps) -> Element {
    let size: &'static str = props.size.unwrap_or("1rem");
    let family: &'static str = props.family.unwrap_or("br cobane");
    let weight: &'static str = props.weight.unwrap_or("normal");
    let color_direction: &'static str = props.color_direction.unwrap_or("to right bottom");
    let colors: Vec<&'static str> = props.colors.unwrap_or(vec![color::SILVER]);
    let mut background: String = String::new();
    background.push_str("linear-gradient");
    background.push('(');
    background.push_str(color_direction);
    background.push(',');
    for color in colors {
        background.push_str(color);
        background.push(','); // LETS HOPE CSS LIKES LEADING COMMAS??
    }
    background.push(')');

    rsx!(
        div {
            style: format!(
                r#"
                    font-size: {};
                    font-family: {};
                    font-weight: {};
                    background: {};
                    -webkit-background-clip: text;
                    -webkit-text-fill-color: transparent;
                    background-clip: text;
                    color: transparent;
                "#,
                size,
                family,
                weight,
                background
            )
        }
    )
}
