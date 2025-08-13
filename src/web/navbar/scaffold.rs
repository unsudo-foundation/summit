use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct ScaffoldProps {
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
                    min-width: 100%;
                    max-width: 100%;
                    width: 100%;
                    {}
                "#,
                props.style.unwrap_or_default()
            ),
            { props.children }
        }
    )
}