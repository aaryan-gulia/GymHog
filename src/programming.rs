use dioxus::{
    html::{img, text},
    prelude::*,
};
use serde::{Deserialize, Serialize};
use std::dbg;

use crate::Route;

#[cfg(feature = "server")]
use super::backend::get_db;

#[component]
pub fn Programming_dummy() -> Element {
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
            program_name.set(String::from(""));
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

#[component]
pub fn Programming() -> Element {}

#[component]
pub fn add_program_interface() -> Element {
    let mut program_name = use_signal(|| String::from("Program Name"));

    rsx!(
    div {
        Link{ to: Route::Programming, "Add New Program" }
    }
    )
}

pub fn add_program_day_interface() -> Element {
    todo!()
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
