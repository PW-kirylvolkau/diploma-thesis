use axum::extract::multipart::Field;
use reqwest::Client;

pub async fn upload_file_to_blob<'a>(field: Field<'_>) -> Option<String> {
    let upload_url = std::env::var("BLOB_UPLOAD_URL").expect("blob upload url should be set");

    let name = field.file_name().unwrap().to_owned();
    let mime = field.content_type().unwrap().to_owned();
    let bytes = field.bytes().await.unwrap();

    let file = reqwest::multipart::Part::stream(bytes)
        .file_name(name)
        .mime_str(mime.as_str())
        .unwrap();

    let body = reqwest::multipart::Form::new().part("File", file);

    let client = Client::new();
    let response = client
        .post(upload_url)
        .multipart(body)
        .send()
        .await
        .unwrap();

    match response.text().await {
        Ok(text) => Some(text),
        Err(_) => None,
    }
}
