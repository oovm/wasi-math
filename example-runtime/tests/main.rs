use std::path::Path;
use wasi_math::MathRuntime;

#[tokio::test]
async fn run_math() -> anyhow::Result<()> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/component.wat");
    let wast = std::fs::read_to_string(&path)?;
    MathRuntime::load_wast(&wast).await?;
    Ok(())
}
