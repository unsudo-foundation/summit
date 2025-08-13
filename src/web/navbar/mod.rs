use super::*;

::modwire::expose!(
    logo
    option
    scaffold
);

#[component]
pub fn Navbar() -> Element {
    rsx!(
        Scaffold {
            left: rsx!(
                Logo {}
            )
        }
    )
}