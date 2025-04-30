mod common;

use anyhow::Result;
use common::{HEIGHT, WIDTH};
use tokio::fs;

#[tokio::test]
async fn should_get_full_score() -> Result<()> {
    let html = r#"
<div></div>
<style>
  * {margin: 0;
  }
  body {
    background: #5d3a3a;
  }
  div {
    width: 200px;
    height: 200px;
    background: #b5e0ba;
  }
</style>"#;
    let expected_image = fs::read("tests/data/simply-square.png").await?;

    let browser = common::init_browser().await?;
    let page = browser.new_page("about:blank").await?;

    let (score, _) = html_image_comparer::diff(html, &expected_image, WIDTH, HEIGHT, &page).await?;

    assert_eq!(score, 1.);

    Ok(())
}
