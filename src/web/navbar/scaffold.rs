use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct ScaffoldProps {
    pub left: Option<Element>,
    pub right: Option<Element>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn Scaffold(props: ScaffoldProps) -> Element {
    rsx!(
        ::diogen::layout::Row {
            class: props.class,
            style: format!(
                r#"
                    justify-content: space-between;
                    min-width: 100%;
                    max-width: 100%;
                    width: 100%;
                    padding-top: {};
                    padding-left: {};
                    padding-right: {};
                    {}
                "#,
                rho::from(3),
                rho::from(3),
                rho::from(3),
                props.style.unwrap_or_default()
            ),
            if let Some(left) = props.left {
                { left }
            } else {
                div {}
            }
            if let Some(children) = props.children {
                { children }
            } else {
                div {}
            }
            if let Some(right) = props.right {
                { right }
            } else {
                div {}
            }
        }
    )
}