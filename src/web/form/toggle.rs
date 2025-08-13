use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToggleProps {
    pub toggled: Signal<bool>,
    pub class: Option<String>,
    pub style: Option<String>
}


// get element dimensions to do dynamic sizing and positioning..
// TODO
#[component]
pub fn Toggle(props: ToggleProps) -> Element {
    let toggled: Signal<_> = props.toggled.to_owned();
    let knob_0_x_offset_on_not_toggled: f64 = -100.0;
    let knob_0_x_offset_on_toggled: f64 = 0.0;
    let knob_0_x_offset: Signal<f64> = use_signal(|| if toggled() {
        knob_0_x_offset_on_toggled
    } else {
        knob_0_x_offset_on_not_toggled
    });
    let knob_1_x_offset_on_not_toggled: f64 = 0.0;
    let knob_1_x_offset_on_toggled: f64 = 100.0;
    let knob_1_x_offset: Signal<f64> = use_signal(|| if toggled() {
        knob_1_x_offset_on_toggled
    } else {
        knob_1_x_offset_on_not_toggled
    });

    {
        let mut knob_0_x_offset: Signal<_> = knob_0_x_offset.to_owned();
        let mut knob_1_x_offset: Signal<_> = knob_1_x_offset.to_owned();
        if toggled() {
            knob_0_x_offset.set(knob_0_x_offset_on_toggled);
            knob_1_x_offset.set(knob_1_x_offset_on_toggled);
        } else {
            knob_0_x_offset.set(knob_0_x_offset_on_not_toggled);
            knob_1_x_offset.set(knob_0_x_offset_on_not_toggled);
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

                "#
            }
            div {
                style: r#"
                    position: absolute;
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    align-items: center;
                    min-width: 50px;
                    aspect-ratio: 1 / 1;
                    border-radius: 2px;
                    background: {color::SILVER};
                    transform: translate({x_offset()}%, 0);
                    transition: transform ease-in 0.3s;
                "#
            }
        }
    )
}