use super::*;


#[derive(Props, Clone, PartialEq)]
pub struct DropdownProps {
    pub selected: Signal<Option<&'static str>>,
    pub selectable: Vec<&'static str>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub children: Option<Element>
}

#[component]
pub fn Dropdown(props: DropdownProps) -> Element {
    let selected: Signal<_> = props.selected.to_owned();
    let toggled: Signal<bool> = use_signal(|| false);

    rsx!(
        div {
            style: r#"display: contents"#,
            ::diogen::layout::Col {
                style: r#"
                    align-items: start;
                    {props.style.to_owned().unwrap_or_default()}
                "#,
                div {
                    onclick: {
                        let mut toggled: Signal<_> = toggled.to_owned();
                        move |_| {
                            toggled.set(!toggled());
                        }
                    },
                    style: format!(
                        r#"
                            display: contents;
                        "#
                    ),
                    ::diogen::layout::Row {
                        style: r#"
                            font-size: 32px;
                            font-family: br cobane;
                            font-weight: normal;
                            color: {color::SILVER};
                            user-select: none;
                            cursor: pointer;
                        "#,
                        if let Some(selected) = selected() {
                            { selected }
                        } else {
                            { props.children }
                        }
                    }
                }
                if toggled() {
                    ::diogen::layout::Col {
                        style: r#"
                            justify-content: start;
                            align-items: start;
                            position: absolute;
                            transform: translate(0, 100%);
                            border-width: 1px;
                            border-style: solid;
                            border-color: {color::JET};
                            border-radius: 2px;
                            gap: 8px;
                        "#,
                        for selectable in props.selectable {
                            div {
                                onclick: {
                                    let mut selected: Signal<_> = selected.to_owned();
                                    let mut toggled: Signal<_> = toggled.to_owned();
                                    move |_| {
                                        selected.set(Some(selectable));
                                        toggled.set(false);
                                    }
                                },
                                style: r#"display: contents"#,
                                ::diogen::layout::Row {
                                    style: r#"
                                        justify-content: start;
                                        font-size: 16px;
                                        font-family: br cobane;
                                        font-weight: normal;
                                        cursor: pointer;
                                        user-select: none;
                                    "#,
                                    Icon {
                                        url: asset!("asset/icon/chev_r.svg"),
                                        size: "50px"
                                    }
                                    { selectable }
                                }
                            }
                        }
                    }
                }
            }
        }
    )
}