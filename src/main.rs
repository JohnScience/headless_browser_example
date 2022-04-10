use std::fs;
use headless_chrome::{
    browser::{Browser, LaunchOptionsBuilder},
    protocol::page::ScreenshotFormat};
use failure::err_msg;

fn main() -> Result<(), failure::Error> {
    let mut lo_builder = LaunchOptionsBuilder::default();
    lo_builder.window_size(Some((1280, 1024)));
    let launch_options = lo_builder.build().map_err(err_msg)?;

    let browser = Browser::new(launch_options)?;

    let tab = browser.wait_for_initial_tab()?;

    tab.navigate_to("https://github.com/atroche/rust-headless-chrome")?;
    tab.wait_until_navigated()?;
    let jpeg_data = tab.capture_screenshot(
        ScreenshotFormat::JPEG(Some(75)),
        None,
        true)?;
    fs::write("screenshot.jpeg", jpeg_data)?;
    Ok(())
}
