use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct RackProps {
    pub children: Option<Element>
}

#[component]
pub fn Rack(props: RackProps) -> Element {
    rsx!(
        div {
            style: format!(
                r#"
                    display: flex;
                    flex-direction: row;
                    justify-content: space-between;
                    align-items: center;
                    min-width: 100%;
                "#
            ),
            { props.children }
        }
    )
}