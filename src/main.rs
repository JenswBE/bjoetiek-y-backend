use bjoetiek;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = bjoetiek::models::Config::from_env();
    bjoetiek::run(config).await
}
