use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct LogoProps {
    pub class: Option<String>,
    pub style: Option<String>
}

#[component]
pub fn Logo(props: LogoProps) -> Element {
    rsx!(
        ::diogen::layout::Row {
            style: format!(
                r#"
                    font-family: alien skyline;
                    font-size: {}em;
                    font-weight: normal;
                    color: {};
                    user-select: none;
                    cursor: pointer;
                    {}
                "#,
                1.5f64,
                color::SILVER,
                props.style.unwrap_or_default()
            ),
            "unSUDO"
        }
    )
}