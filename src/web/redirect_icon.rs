use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct RedirectIconProps {
    pub w: String,
    pub url: Asset,
    pub to: String
}

#[component]
pub fn RedirectIcon(props: RedirectIconProps) -> Element {
    rsx!(
        Redirect {
            to: props.to,
            Icon {
                w: props.w, 
                url: props.url
            }
        }
    )
}