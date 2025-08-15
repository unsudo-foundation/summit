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
        input {
            oninput: {
                let mut input: Signal<_> = input.to_owned();
                move |e: Event<FormData>| {
                    let e: String = e.value();
                    input.set(Some(e));
                }
            },
            r#type: "text",
            style: format!(
                r#"
                    border: none;
                    border-radius: 2px;
                    -webkit-appearance: none;
                    -ms-appearance: none;
                    -moz-appearance: none;
                    appearance: none;
                    background: {};
                    padding: 12px;
                    min-width: 250px;
                    outline: none;
                "#,
                color::CARBON,
                
            )
        }
    )
}