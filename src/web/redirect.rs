use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct RedirectProps {
    pub to: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn Redirect(props: RedirectProps) -> Element {
    rsx!(
        Link {
            to: props.to.unwrap_or("/".into()),
            style: format!(
                r#"
                    all: unset;
                    cursor: pointer;
                "#
            ),
            { props.children }
        }
    )
}