use dioxus::{
    html::{img, text},
    prelude::*,
};
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
    let mut all_programs =
        use_resource(|| async move { get_all_programs().await.unwrap_or_default() });
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
            all_programs.restart();
        },
        "add program"
        }
    }
    div {
        ul {
            for program in all_programs.cloned().unwrap_or_default(){
                div{id: program.name, "{program.name}"}
            }
        }

    }
    )
}

#[server]
pub async fn add_program(program: ProgramTestModel1) -> Result<(), ServerFnError> {
    let program_added: Option<ProgramTestModel1> = get_db()
        .await
        .create("Program")
        .content(program)
        .await
        .expect("Can't add program");

    dbg!(program_added);

    Ok(())
}

#[server]
pub async fn get_all_programs() -> Result<Vec<ProgramTestModel1>, ServerFnError> {
    let all_programs: Vec<ProgramTestModel1> = get_db()
        .await
        .select("Program")
        .await
        .expect("Can't select from table");

    Ok(all_programs)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProgramTestModel1 {
    pub name: String,
}
