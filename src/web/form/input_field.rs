use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct InputFieldProps {
    pub label: String,
    pub input: Signal<Option<String>>,
    pub class: Option<String>,
    pub style: Option<String>
}

#[component]
pub fn InputField(props: InputFieldProps) -> Element {
    let input: Signal<_> = props.input.to_owned();
    let mut top_label_opacity: Signal<f64> = use_signal(|| 0.0);
    
    match input() {
        None => top_label_opacity.set(1.0),
        Some(input) => if input.is_empty() {
            top_label_opacity.set(1.0)
        } else {
            top_label_opacity.set(0.5)
        }
    };

    rsx!(
        div {
            style: r#"
                display: flex;
                flex-direction: column;
                justify-content: start;
                align-items: start;
                min-width: 0%;
                max-width: 100%;
                width: 100%;
                flex: 1;
                background: linear-gradient(to right, {color::CARBON}, {color::OBSIDIAN});
            "#,
            div {
                style: r#"
                    display: flex;
                    flex-direction: row;
                    justify-content: start;
                    align-items: center;
                    font-size: 0.5em;
                    font-family: br cobane;
                    font-weight: normal;
                    opacity: {top_label_opacity};
                    transition: opacity ease 0.3s;
                    user-select: none;
                "#,
                { props.label.to_owned() }
            }
            input {
                oninput: {
                    let mut input: Signal<_> = input.to_owned();
                    move |e: Event<FormData>| {
                        let e: String = e.value();
                        input.set(Some(e));
                    }
                },
                r#type: "text",
                style: r#"
                    all: unset;
                    display: flex;
                    flex-direction: row;
                    flex: 1;
                    font-size: 1em;
                    font-family: br cobane;
                    font-weight: normal;
                    color: {color::SILVER};
                    border-radius: 2px;
                "#
            }
        }
    )
}