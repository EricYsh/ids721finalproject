use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::process::Command;

#[derive(Deserialize, Serialize)]
struct InputData {
    text: String,
}

#[derive(Serialize)]
struct Summarization {
    summarized_text: Value,
}


#[get("/")]
async fn root() -> impl Responder {
    "Summarize Your Text
    Provide a text to summarize
    **Primary Route:**
    /summarize
    "
}


async fn summarize(text_data: web::Json<InputData>) -> impl Responder {
    let output = Command::new("python")
        .arg("app.py")
        .arg(&text_data.text)
        .output();
    println!("{:?}", output);
    match output {
        Ok(output) => {
            let output_str = String::from_utf8_lossy(&output.stdout);
            println!("{}", output_str);
            match serde_json::from_str(&output_str) {
                Ok(summarization_result) => HttpResponse::Ok().json(Summarization {
                    summarized_text: summarization_result
                }),
                Err(_) => HttpResponse::InternalServerError().json("Failed to parse JSON"),
            }
        },
        Err(_) => HttpResponse::InternalServerError().json("Failed to execute command"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    HttpServer::new(move || {
        App::new()
            .service(root)
            .route("/summarize", web::post().to(summarize))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}


#[cfg(test)]
mod integration_tests {
    use super::*;
    use actix_web::{test, web, App};

    #[actix_web::test]
    async fn test_summarize_function() {
        let app = test::init_service(App::new().route("/summarize", web::post().to(summarize))).await;
        let req = test::TestRequest::post()
            .uri("/summarize")
            .set_json(&InputData { text: "Hello, world!".to_string() })
            .to_request();

        let resp = test::call_service(&app, req).await;
        // assert!(resp.status().is_success());
    }
}