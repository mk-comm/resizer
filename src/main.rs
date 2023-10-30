use anyhow::Result;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::Json;
use axum::Router;
use base64::encode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::Cursor;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let port = match std::env::var("8080") {
        Ok(val) => val,
        Err(_e) => "8080".to_string(),
    };
    let address: SocketAddr = format!("0.0.0.0:{}", port)
        .parse()
        .expect("Failed to parse address");

    let app = Router::new().route("/convert", post(convert));

    hyper::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .expect("Server failed to start")
}

async fn convert(json: Json<ConvertRequest>) -> impl IntoResponse {
    let result = convert_file(json.0.url).await;
    let fin = match result {
        Ok(v) => v,
        Err(e) => {
            println!("Error: {:?}", e);
            return Json(json!({"status": "error"}));
        }
    };

    Json(json!({
        "status": "ok",
        "base64": fin
    }))
}

#[derive(Debug, Deserialize, Serialize)]
struct ConvertRequest {
    url: String,
}

async fn convert_file(url: String) -> Result<String> {
    // Fetch image data from the URL
    let resp = reqwest::get(&url).await?;
    let bytes = resp.bytes().await?;

    // Decode the image data
    let img = image::load_from_memory(&bytes).unwrap().to_rgb8();

    // Set the new dimensions (e.g., downscale by 2x)
    let (w1, h1) = img.dimensions();
    let (w2, h2) = (w1 / 2, h1 / 2);

    // Resize the image
    let new_img = image::imageops::resize(&img, w2, h2, image::imageops::Lanczos3);

    // Write the resized image to a Vec<u8> using a Cursor
    let mut buffer = Cursor::new(Vec::new());
    new_img
        .write_to(&mut buffer, image::ImageOutputFormat::Jpeg(50))
        .unwrap();

    // Reset the cursor position to the start of the buffer
    buffer.set_position(0);

    // Encode the buffer as Base64
    let base64_string = encode(&buffer.into_inner());

    // Now you can use base64_string for your purposes
    //println!("{}", base64_string);
    Ok(base64_string)
}
