use bjoetiek;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    bjoetiek::run().await
}
