use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    // /hello
    hc.do_get("/hello?name=rem").await?.print().await?;
    hc.do_get("/hello").await?.print().await?;

    // /hello2
    hc.do_get("/hello2").await?.print().await?;
    hc.do_get("/hello2/rem").await?.print().await?;

    // /login
    hc.do_post(
        "/login",
        json!({"username": "remi", "password": "password"}),
    )
    .await?
    .print()
    .await?;
    hc.do_post("/login", json!({"username": "remi", "password": "xxx"}))
        .await?
        .print()
        .await?;

    // bot
    hc.do_get("/bot").await?.print().await?;
    hc.do_post("/bot", json!({"name": "my bot"}))
        .await?
        .print()
        .await?;
    hc.do_post("/bot", json!({"name": "my bot2"}))
        .await?
        .print()
        .await?;
    hc.do_get("/bot").await?.print().await?;
    hc.do_delete("/bot/0").await?.print().await?;
    hc.do_get("/bot").await?.print().await?;

    Ok(())
}
