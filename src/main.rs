#![feature(proc_macro_hygiene, decl_macro)]
#![allow(unused_imports)]
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use rocket::routes;
use rocket::get;
use rocket::post;
use rocket::response::Redirect;
use rocket_contrib::json;
use rocket_contrib::templates::tera::{Context};
use rocket_contrib::templates::Template;
use serde::Serialize;
use serde::Deserialize;
use rocket::FromForm;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

#[derive(Clone,FromForm, Serialize, Deserialize, Debug)]
struct Task {
    id: u64,
    description: String,
}

#[derive(Clone,FromForm, Serialize, Deserialize, Debug)]
struct TaskData {
    description:String
}
// Define a global static variable for tasks using lazy_static
lazy_static! {
    static ref TASKS: Arc<Mutex<Vec<Task>>> = {
        let mut tasks = Vec::new();
        tasks.push(Task {
            id: 1,
            description: "Initial Task".to_string(),
        });

        Arc::new(Mutex::new(tasks))
    };
}

// This function creates a new Rocket instance
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage(TASKS.clone())  // Share the tasks vector across Rocket instances
        .mount("/", routes![index, create,remove])
        .attach(Template::fairing())
}

// This function initializes the Rocket instance and launches the server
fn main() {
    rocket().launch();
}

#[get("/")]
fn index() -> Template {
    // Access the tasks vector from the shared state
    let tasks = TASKS.lock().unwrap();
    println!("{:?}",tasks.clone());
    println!("{}",tasks.len());
    
    Template::render("home", tasks.clone())
}

#[post("/create", data="<form_data>")]
fn create(form_data: rocket::request::Form<TaskData>) -> Redirect {
    let mut tasks = TASKS.lock().unwrap();

    println!("{:?}", form_data);
    let new_task = Task {
        id: (tasks.len()+1) as u64,
        description: form_data.description.clone(),
    };
    tasks.push(new_task.clone());
    println!("{:?}", new_task);

    Redirect::to("/")
}

#[get("/remove/<id>")]
fn remove(id: u64) -> Redirect {
    let mut tasks = TASKS.lock().unwrap();
    if let Some(index) = tasks.iter().position(|task| task.id == id) {
        tasks.remove(index);
    }

    Redirect::to("/")
}


