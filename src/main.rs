use dioxus::prelude::*;

pub mod backend;

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

#[component]
fn Programming() -> Element {
    let mut program_name = use_signal(|| String::from(""));
    rsx!(
    div {
        input  {
        value :"{program_name}",
        oninput : move |e| program_name.set(e.value())
        }
        button {
        id : "add_program",
        onclick : move |_| async move {_ = add_program(program_name.to_string()).await;},
        "add program"
        }
    }
    )
}

#[server]
async fn add_program(name: String) -> Result<(), ServerFnError> {
    println!("server received program : {}", name);
    Ok(())
}
