use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::Route;

#[cfg(feature = "server")]
use super::backend::get_db;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MuscleGroup {
    Abbs,
    Back,
    Biceps,
    Calves,
    Chest,
    Forearms,
    Glutes,
    Hamstrings,
    Quads,
    Shoulders,
    Triceps,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Program {
    pub name: String,
    pub days: Vec<ProgramDay>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProgramDay {
    pub name: String,
    pub exercises: Vec<Exercise>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Exercise {
    pub name: String,
    pub muscle_groups: Vec<MuscleGroup>,
}

#[component]
pub fn list_programs(all_programs: Resource<Vec<Program>>) -> Element {
    rsx!(
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
pub fn Programming() -> Element {
    let mut all_programs =
        use_resource(|| async move { get_all_programs().await.unwrap_or_default() });

    rsx!(
        div{
            Link{to : Route::AddProgramInterface, "Add New Program"}
        }
        div{
            list_programs { all_programs }
        }
    )
}

#[component]
pub fn AddProgramInterface() -> Element {
    let mut program: Signal<Program> = use_signal(|| Program::default());
    let mut currDay: Signal<ProgramDay> = use_signal(|| ProgramDay {
        name: format!("Day {}", program.read().days.iter().len() + 1),
        exercises: vec![],
    });
    let mut currExercise: Signal<Exercise> = use_signal(|| Exercise {
        name: String::from(""),
        muscle_groups: vec![MuscleGroup::Abbs],
    });
    let mut program_added: Signal<bool> = use_signal(|| false);
    let mut adding_day: Signal<bool> = use_signal(|| false);
    let mut adding_exercise: Signal<bool> = use_signal(|| false);

    let mut program_add_result = use_signal(|| String::from(""));

    let set_name = move |name: String| {
        program.write().name = name;
    };

    let add_day = move |day: ProgramDay| {
        program.write().days.push(day);
    };

    let add_exercise = move |exercise: Exercise| {
        currDay.write().exercises.push(exercise);
    };

    rsx!(
    div {
        Link{ to: Route::Programming, "Go Back" }
    }
    div {
        id: "program_name",
        textarea {
            placeholder: "eg: Push/Pull/Leg - 1",
            value: "{program.read().name}",
            oninput: move |e| program.write().name = e.value(),
        }
    }
    div {
        id: "days",
        ol {
            for day in program.read().days.iter(){
                li {
                    "{day.name} : ",
                    ul {
                        for exercise in day.exercises.iter(){
                            li { "{exercise.name}" }
                         }
                    }

                }
            }
        }
    }
    if format!("{adding_day}") == "false" {
    div {
        id: "new_day",
        button {
            onclick : move|_| adding_day.set(true),
            "Add New Day"
        }
    }} else {
    div{
        id: "new_day",
        textarea {
            value: "{currDay.read().name}",
            oninput: move |e| currDay.write().name = e.value(),
            placeholder: "eg: Day 1"
        }
        ol {
            for exercise in currDay.read().exercises.iter(){
                li { "{exercise.name}" }
            }
        }
        if format!("{adding_exercise}") == "false" {
            div {
                id: "new_exercise",
                button {
                onclick : move|_| adding_exercise.set(true),
                "Add Exercise Set"
                }
            }} else {
            div{
                id: "new_exercise",
                textarea {
                    value: "{currExercise.read().name}",
                    oninput: move |e| currExercise.write().name = e.value(),
                    placeholder: "eg: Bench Press"
                }
                button {
                    onclick : move |_| {
                        currDay.write().exercises.push(currExercise.cloned().into());
                        currExercise.set(Exercise{name:String::from(""),muscle_groups:vec![MuscleGroup::Abbs]});
                        adding_exercise.set(false);
                    },
                    "Add Exercise In {currDay.read().name}"
                }
            }
        }
        button {
            onclick : move |_| {
                program.write().days.push(currDay.cloned().into());
                currDay.set(ProgramDay{
                    name: format!("Day {}",program.read().days.iter().len()+1),
                    exercises: vec![]
                });
                adding_day.set(false);
            },
            "Add Day : {currDay.read().name} to the program"
        }
    }
    }
    div{
        button {
            onclick : move |_| {
                let payload: Program = program.read().clone();
            spawn(async move {
                match add_program(payload).await {
                    Ok(s) => program_add_result.set(format!("program add ok: {s}")),
                    Err(e) => program_add_result.set(format!("program add error: {e}")),
                }
            });
            },
            "Save Program wrong",
        },
    }
    div { "{program_add_result}" }
    )
}

#[component]
pub fn ProgramView(program: String) -> Element {
    todo!()
}

#[server]
pub async fn add_program(program_str: Program) -> Result<String, ServerFnError> {
    // PROVE we reached the server function
    let mut program = Program::default();
    program = program_str;
    println!("server: entered add_program with name='{}'", program.name);

    let db = get_db().await;

    // Use lowercase table name (Surreal is case-sensitive)
    let created: Option<Program> = db
        .create("program")
        .content(program.clone())
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    if created.is_none() {
        return Err(ServerFnError::new("create returned None"));
    }

    println!("server: created program '{}'", program.name);
    Ok(String::from("Program Added"))
}

#[server]
pub async fn get_all_programs() -> Result<Vec<Program>, ServerFnError> {
    let all_programs: Vec<Program> = get_db()
        .await
        .select("Program")
        .await
        .expect("Can't select from table");

    Ok(all_programs)
}
