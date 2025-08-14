use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct TypographyProps {
    pub size: String,
    pub family: String,
    pub weight: String,
    pub color_direction: Option<&'static str>,
    pub colors: Option<Vec<&'static str>>
}

#[component]
pub fn Typography(props: TypographyProps) -> Element {
    let mut background: String = String::new();
    background.push_str("linear-gradient");
    background.push('(');
    background.push_str(props.color_direction.unwrap_or("to right bottom"));
    background.push(',');
    for color in props.colors.unwrap_or(vec![color::SILVER]).into_iter() {
        background.push_str(color);
        background.push(','); // LETS HOPE CSS LIKES LEADING COMMAS??
    }
    background.push(')');

    rsx!(
        div {
            style: format!(
                r#"
                    font-size: 2em;
                    font-family: br cobane;
                    font-weight: bold;
                    background: {};
                    -webkit-background-clip: text;
                    -webkit-text-fill-color: transparent;
                    background-clip: text;
                    color: transparent;
                "#,
                background
            )
        }
    )
}