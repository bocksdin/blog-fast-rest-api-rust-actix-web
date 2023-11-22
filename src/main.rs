use actix_web::{get, web::Data, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

mod todolist;
use todolist::services;

struct AppState {
    todolist_entries: Mutex<Vec<TodolistEntry>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct TodolistEntry {
    id: i32,
    date: i64,
    title: String,
}

#[get("/")] // GET method for the "/" path
async fn index() -> impl Responder {
    HttpResponse::Ok().json("{ status: OK }")
}

// This tells our program to utilize the actix_web runtime
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data: Data<AppState> = Data::new(AppState {
        todolist_entries: Mutex::new(vec![])
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            .configure(services::config)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
