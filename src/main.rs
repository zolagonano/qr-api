#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use base64::{decode as b64decode, encode as b64encode};
use image::{load_from_memory, DynamicImage, ImageOutputFormat, Luma};
use qrcode::QrCode;
use rocket_contrib::json::Json;
use rqrr::PreparedImage;

#[get("/encode/<text>?<width>&<height>")]
fn encode(
    text: String,
    width: Option<u32>,
    height: Option<u32>,
) -> Result<Json<String>, Json<String>> {
    let width = match width {
        Some(v) => v,
        None => 128,
    };
    let height = match height {
        Some(v) => v,
        None => 128,
    };

    if let Ok(qrcode) = QrCode::new(text.as_bytes()) {
        let qrcode_image_buffer = qrcode
            .render::<Luma<u8>>()
            .max_dimensions(width, height)
            .build();

        let qrcode_dynamic_image = DynamicImage::ImageLuma8(qrcode_image_buffer);

        let mut image_bytes: Vec<u8> = Vec::new();

        if let Ok(_v) = qrcode_dynamic_image.write_to(&mut image_bytes, ImageOutputFormat::Png) {
            Ok(Json(b64encode(image_bytes)))
        } else {
            Err(Json(String::from("Error: Cannot get image bytes")))
        }
    } else {
        Err(Json(String::from("Error: Cannot encode this text")))
    }
}

#[get("/decode/<qr_base64>")]
fn decode(qr_base64: String) -> Result<Json<String>, Json<String>> {
    if let Ok(image_bytes) = b64decode(qr_base64) {
        if let Ok(qrcode_dynamic_image) = load_from_memory(&image_bytes) {
            let mut prepared_qr = PreparedImage::prepare(qrcode_dynamic_image.into_luma8());

            let qr_grids = prepared_qr.detect_grids();

            if let Ok((_meta, content)) = qr_grids[0].decode() {
                Ok(Json(content))
            } else {
                Err(Json(String::from("Error: Cannot decode qr grids")))
            }
        } else {
            Err(Json(String::from("Error: Cannot load this image")))
        }
    } else {
        Err(Json(String::from("Error: Cannot decode this base64 encode")))
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![encode, decode])
        .launch();
}
