use chromiumoxide::{Browser, BrowserConfig, handler::viewport::Viewport};
use futures::StreamExt;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 300;

async fn init_browser() -> chromiumoxide::Result<Browser> {
    let viewport = Viewport {
        width: WIDTH,
        height: HEIGHT,
        ..Default::default()
    };

    let config = BrowserConfig::builder()
        .viewport(viewport)
        .no_sandbox()
        .build()
        .map_err(chromiumoxide::error::CdpError::msg)?;

    let (browser, mut handler) = Browser::launch(config).await?;

    tokio::task::spawn(async move {
        loop {
            let _ = handler.next().await;
        }
    });

    Ok(browser)
}

#[tokio::main]
async fn main() {
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

    let expected_image = include_bytes!("expected.png");

    let browser = init_browser().await.unwrap();
    let page = browser.new_page("about:blank").await.unwrap();

    let (score, _) = html_image_comparer::diff(html, expected_image, WIDTH, HEIGHT, &page)
        .await
        .unwrap();
    println!("Score: {}", score);
}
