use dioxus::prelude::*;

mod backend;
mod programming;

use programming::*;

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Home {},

    #[route("/ping_test")]
    PingTest,

    #[nest("/programming")]
    #[route("/")]
    Programming,
    #[route("/program/:program")]
    ProgramView { program: String },
    #[route("/add")]
    AddProgramInterface,
    #[end_nest]
    #[route("/calendar")]
    Calendar,

    #[end_layout]
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx!(Router::<Route> {})
}

#[component]
fn NavBar() -> Element {
    rsx!(
        nav{
            ul {
                li {Link{to : Route::Home {}, "Home"}},
                li {Link{to : Route::Programming, "Your Programs"}},
                li {Link{to : Route::Calendar, "Workout Calendar"}},
                li {Link{to : Route::PingTest, "Ping Test"}},
            }
        }
        Outlet::<Route>{}
    )
}

#[component]
fn PingTest() -> Element {
    let mut ping_result = use_signal(|| String::new());
    rsx!(

    button {
        onclick: move |_| {
            spawn(async move {
                match ping().await {
                    Ok(s) => ping_result.set(format!("ping ok: {s}")),
                    Err(e) => ping_result.set(format!("ping error: {e}")),
                }
            });
        },
        "Ping server"
    }
    div { "{ping_result}" }
    )
}

#[server]
async fn ping() -> Result<String, ServerFnError> {
    println!("server ping reached"); // you MUST see this in the dx terminal
    Ok("pong".to_string())
}

#[component]
fn Home() -> Element {
    //TODO
    rsx!(div { h1 { "COMING SOON" } })
}

#[component]
fn Calendar() -> Element {
    //TODO
    rsx!(div { h1 { "COMING SOON" } })
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    //TODO
    rsx!(div { h1 { "COMING SOON" } })
}
