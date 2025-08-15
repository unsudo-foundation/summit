use super::*;

#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct DropdownProps {
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn Dropdown(props: DropdownProps) -> Element {
    rsx!(
        div {
            class: props.class,
            style: format!(
                r#"
                    display: flex;
                    flex-direction: column;
                    justify-content: start;
                    align-items: start;
                    min-width: 100%;
                    flex: 1;
                    {}
                "#,
                props.style.unwrap_or_default()
            ),
            { props.children }
        }
    )
}


#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct DropdownLabelProps {
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn DropdownLabel(props: DropdownLabelProps) -> Element {
    rsx!(
        label {
            class: props.class,
            style: format!(
                r#"
                    font-size: 1em;
                    font-family: br cobane;
                    font-weight: normal;
                    {}
                "#,
                props.style.unwrap_or_default()
            ),
            { props.children }
        }
    )
}


#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct DropdownSelectProps {
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn DropdownSelect(props: DropdownSelectProps) -> Element {
    rsx!(
        select {
            class: props.class,
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


#[derive(Props)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct DropdownOptionProps {
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn DropdownOption(props: DropdownOptionProps) -> Element {
    rsx!(
        option {
            class: props.class,
            style: format!(
                r#"
                    font-size: 1em;
                    font-family: br cobane;
                    font-weight: normal;
                    color: black;
                    cursor: pointer;
                    user-select: none;
                    background: transparent;
                    padding: 2px;
                    {}
                "#,
                props.style.unwrap_or_default()
            ),
            { props.children }
        }
    )
}