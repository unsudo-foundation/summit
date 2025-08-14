use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct IconProps {
    pub w: String,
    pub url: Asset,
    pub class: Option<String>,
    pub style: Option<String>
}

#[component]
pub fn Icon(props: IconProps) -> Element {
    rsx!(
        ::diogen::layout::Col {
            style: format!(
                r#"
                    min-width: {};
                    aspect-ratio: 1 / 1;
                    background-image: url({});
                    background-position: center;
                    background-size: contain;
                    background-repeat: no-repeat;
                    color: {};
                    {}
                "#,
                props.w,
                props.url,
                color::SILVER,
                props.style.to_owned().unwrap_or_default()
            )
        }
    )
}