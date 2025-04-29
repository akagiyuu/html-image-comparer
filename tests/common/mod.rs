use chromiumoxide::{Browser, BrowserConfig, handler::viewport::Viewport};
use futures::StreamExt;

pub const WIDTH: u32 = 400;
pub const HEIGHT: u32 = 300;

pub async fn init_browser() -> chromiumoxide::Result<Browser> {
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
