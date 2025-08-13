use super::*;

::modwire::expose!(
    pub dropdown
    pub input_field
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
        ::diogen::layout::Col {
            style: r#"
                border-width: 1px;
                border-style: solid;
                border-color: {color::JET};
                border-radius: 2px;
                padding: 10px;
            "#,
            { props.children }
        }
    )
}