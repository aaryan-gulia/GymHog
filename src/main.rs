use dioxus::{html::li, prelude::*};

mod backend;
mod programming;

use programming::*;

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    Home {},
    #[route("/programming")]
    Programming,
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
            }
        }
        Outlet::<Route>{}
    )
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
