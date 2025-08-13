use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct IconProps {
    pub url: Asset,
    pub size: String,
    pub class: Option<String>,
    pub style: Option<String>
}

#[component]
pub fn Icon(props: IconProps) -> Element {
    rsx!(
        ::diogen::layout::Col {
            style: r#"
                width: {props.size};
                background-image: url({props.url});
                background-position: center;
                background-size: contain;
                background-repeat: no-repeat;
                color: {color::SILVER};
                {props.style.to_owned().unwrap_or_default()}
            "#
        }
    )
}