use actix_files::Files;
use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use std::{env, fmt::Display};

#[derive(Debug)]
pub struct AppError(&'static str);

impl Display for AppError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl std::error::Error for AppError {
  fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
    None
  }
}

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let static_dir = env::var("STATIC_DIR").map_err(|_| AppError("STATIC_DIR not set"))?;
  HttpServer::new(move || App::new().service(Files::new("/", &static_dir).index_file("index.html")))
    .bind("0.0.0.0:8080")?
    .run()
    .await
    .map_err(Into::into)
}
