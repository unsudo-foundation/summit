use super::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {}
}

#[component]
fn Home() -> Element {
    rsx!(
        ::diogen::layout::Page {
            scroll_snap: diogen::layout::PageScrollSnap::Proximity,
            style: format!(
                r#"
                    background: grey;
                    color: red;
                "#
            ),
            ::diogen::layout::PageItem {
                "Hello World"
            }
        }
    )
}