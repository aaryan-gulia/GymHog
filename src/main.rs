use dioxus::prelude::*;

pub mod backend;
pub mod programming;

use programming::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx!(
        header {  }
        Programming {}
        footer {  }
    )
}
