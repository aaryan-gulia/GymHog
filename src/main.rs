use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::dbg;

#[cfg(feature = "server")]
use surrealdb::RecordId;

pub mod backend;

#[cfg(feature = "server")]
use backend::get_db;

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
        onclick : move |_| async move {
        _ = add_program(ProgramTestModel1 { name: program_name.to_string() }).await;
        },
        "add program"
        }
    }
    )
}

#[server]
async fn add_program(program: ProgramTestModel1) -> Result<(), ServerFnError> {
    let program_added: Option<ProgramTestModel1> = get_db()
        .await
        .create("Program")
        .content(program)
        .await
        .expect("Can't add program");

    dbg!(program_added);

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ProgramTestModel1 {
    name: String,
}
