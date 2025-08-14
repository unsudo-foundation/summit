use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToggleProps {
    pub toggled: Signal<bool>,
    pub class: Option<String>,
    pub style: Option<String>
}

#[component]
pub fn Toggle(props: ToggleProps) -> Element {
    let toggled: Signal<_> = props.toggled.to_owned();
    let x_offset_on_not_toggled_0: f64 = -100.0;
    let x_offset_on_toggled_0: f64 = 0.0;
    let x_offset_0: Signal<f64> = use_signal(|| if toggled() {
        x_offset_on_toggled_0
    } else {
        x_offset_on_not_toggled_0
    });
    let x_offset_on_not_toggled_1: f64 = 0.0;
    let x_offset_on_toggled_1: f64 = 100.0;
    let x_offset_1: Signal<f64> = use_signal(|| if toggled() {
        x_offset_on_toggled_1
    } else {
        x_offset_on_not_toggled_1
    });
    let knob_size: f64 = 50.0;
    let transform_seconds: f64 = 0.3;

    {
        let mut x_offset_0: Signal<_> = x_offset_0.to_owned();
        let mut x_offset_1: Signal<_> = x_offset_1.to_owned();
        if toggled() {
            x_offset_0.set(x_offset_on_toggled_0);
            x_offset_1.set(x_offset_on_toggled_1);
        } else {
            x_offset_0.set(x_offset_on_not_toggled_0);
            x_offset_1.set(x_offset_on_not_toggled_1);
        }
    }

    rsx!(
        div {
            onclick: {
                let mut toggled: Signal<_> = toggled.to_owned();
                move |_| {
                    toggled.set(!toggled());
                }
            },
            class: props.class,
            style: r#"
                display: flex;
                flex-direction: row;
                justify-content: start;
                align-items: center;
                cursor: pointer;
                border-width: 1px;
                border-style: solid;
                border-color: {color::CARBON};
                overflow-x: hidden;
                overflow-y: hidden;
                {props.style.to_owned().unwrap_or_default()}
            "#,
            div {
                style: r#"
                    min-width: 50px;
                    aspect-ratio: 1 / 1;
                "#
            }
            div {
                style: r#"
                    position: absolute;
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    min-width: {knob_size}px;
                    aspect-ratio: 1 / 1;
                    border-radius: 2px;
                    background: {color::SILVER};
                    transform: translate({x_offset_0()}%, 0);
                    transition: transform ease-in {transform_seconds}s;
                "#
            }
            div {
                style: r#"
                    position: absolute;
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    align-items: center;
                    min-width: {knob_size}px;
                    aspect-ratio: 1 / 1;
                    border-radius: 50px;
                    background: {color::SILVER};
                    transform: translate({x_offset_1()}%, 0);
                    transition: transform ease-in {transform_seconds}s;
                "#
            }
        }
    )
}