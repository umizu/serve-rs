use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:8888")?;
    hc.do_get("/books?title=space").await?.print().await?;
    
    Ok(())
}
