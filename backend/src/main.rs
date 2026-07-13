use clash_ai_backend::app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("ClashAI backend listening on :8080");
    axum::serve(listener, app()).await?;
    Ok(())
}
