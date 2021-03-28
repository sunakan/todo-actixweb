use actix_web::{get, App, HttpResponse, HttpServer};
use std::env;

#[get("/")]
async fn index() -> Result<HttpResponse, actix_web::Error> {
    let response_body = "Hello World!";
    // HttpResponse::Ok()はステータスコード200を持つHttpResponseBuilderという構造体を返す
    // HttpResponseBuilderのbody()という関数にレスポンスのボディを渡すとHttpResponseが返る
    Ok(HttpResponse::Ok().body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    HttpServer::new(move || App::new().service(index))
        .bind(("0.0.0.0", port))?
        .run()
        .await?;
    Ok(())
}
