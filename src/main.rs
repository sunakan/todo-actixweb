use actix_web::{get, App, HttpResponse, HttpServer};

#[get("/")]
async fn index() -> Result<HttpResponse, actix_web::Error> {
    let response_body = "Hello World!";
    // HttpResponse::Ok()はステータスコード200を持つHttpResponseBuilderという構造体を返す
    // HttpResponseBuilderのbody()という関数にレスポンスのボディを渡すとHttpResponseが返る
    Ok(HttpResponse::Ok().body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error>{
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}
