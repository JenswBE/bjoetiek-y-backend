use bjoetiek;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let config = bjoetiek::models::Config::from_env();
    bjoetiek::run(config).await
}
