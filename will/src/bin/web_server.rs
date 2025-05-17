use my_crate::web;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    web::run_server().await
} 