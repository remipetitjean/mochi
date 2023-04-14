use api::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    api::main().await
}
