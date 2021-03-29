use actix_web::{App, HttpServer, Responder, web};
use std::{env, io};
use serde::{ Serialize, Deserialize };
use actix_web::http::StatusCode;

#[derive(Serialize)]
struct Status {
    status: String,
}

#[derive(Serialize)]
struct GainedTodo {
    id: u32,
    title: String,
    done: bool,
}

#[derive(Deserialize)]
struct TodoBeforeSave {
   title: String,
}

#[derive(Serialize, Deserialize)]
struct SavedTodo {
    id: u32,
    title: String,
    done: bool,
}

#[derive(Serialize)]
struct DeletedTodo {
    id: u32,
}

async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "Up".to_string(),
    })
}

async fn todos() -> impl Responder {
    let todos : Vec::<GainedTodo> = vec![];
    web::HttpResponse::Ok().json(todos)
}

async fn create_todo(todo: web::Json<TodoBeforeSave>) -> impl Responder {
    web::HttpResponse::Ok().json(SavedTodo {
        id: 1,
        title: todo.title.to_string(),
        done: false,
    }).with_status(StatusCode::CREATED)
}

async fn get_todo(id: web::Path<u32,>) -> impl Responder {
    web::HttpResponse::Ok().json(SavedTodo {
        id: id.0,
        title: "dummy".to_string(),
        done: false,
    }).with_status(StatusCode::OK)
}

async fn done_todo(id: web::Path<u32,>) -> impl Responder {
    web::HttpResponse::Ok().json(SavedTodo {
        id: id.0,
        title: "dummy".to_string(),
        done: true,
    }).with_status(StatusCode::OK)
}

async fn delete_todo(id: web::Path<u32,>) -> impl Responder {
    web::HttpResponse::Ok().body("".to_string())
        .with_status(StatusCode::NO_CONTENT)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(status))
            .route("/todos{_:/?}", web::get().to(todos))
            .route("/todos{_:/?}", web::post().to(create_todo))
            .route("/todos/{id}", web::get().to(get_todo))
            .route("/todos/{id}", web::post().to(done_todo))
            .route("/todos/{id}", web::delete().to(delete_todo))
    })
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
