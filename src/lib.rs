mod error;

use chromiumoxide::{
    Page,
    cdp::browser_protocol::page::{CaptureScreenshotFormat, CaptureScreenshotParams},
    page::ScreenshotParams,
};
pub use error::*;

async fn render(code: &str, page: &Page) -> Result<Vec<u8>> {
    page.set_content(code).await?;

    let image_buffer = page
        .screenshot(ScreenshotParams {
            cdp_params: CaptureScreenshotParams {
                format: Some(CaptureScreenshotFormat::Png),
                ..Default::default()
            },
            ..Default::default()
        })
        .await?;

    Ok(image_buffer)
}

pub async fn diff(
    html: &str,
    expected_image: &[u8],
    width: u32,
    height: u32,
    page: &Page,
) -> Result<(f64, Vec<u8>)> {
    let html_image = render(html, page).await?;
    let rendered_size = (width * height) as usize;
    let mut diff_buffer = Vec::with_capacity(rendered_size);

    let diff = pixelmatch::pixelmatch::<&[u8], &[u8], _>(
        &html_image,
        expected_image,
        Some(&mut diff_buffer),
        Some(width),
        Some(height),
        Some(pixelmatch::Options {
            threshold: 0.1,
            ..Default::default()
        }),
    )
    .map_err(|_| Error::Image("Failed to compare images"))?;

    let match_percent = 1. - (diff as f64) / (rendered_size as f64);

    Ok((match_percent, diff_buffer))
}
