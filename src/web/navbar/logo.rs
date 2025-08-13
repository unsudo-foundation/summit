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
                    font-size: {}px;
                    font-weight: normal;
                    color: {};
                    {}
                "#,
                rho::from(3),
                color::SILVER,
                props.style.unwrap_or_default()
            ),
            "unSUDO"
        }
    )
}