use super::*;

::modwire::expose!(
    pub dropdown
    pub input_field
    pub rack
    pub toggle
);

#[derive(Props, Clone, PartialEq)]
pub struct RootProps {
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn Root(props: RootProps) -> Element {
    rsx!(
        form {
            style: format!(
                r#"
                    {}
                "#,
                props.style.unwrap_or_default()
            ),
            { props.children }
        }
    )
}