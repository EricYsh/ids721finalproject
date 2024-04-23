use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ChangeResult{
    dollars: u32,
    cents: u32,
    change: Vec<u32>
}

#[get("/")]
async fn root() -> impl Responder {
    "Summarize Your Text
    
    **Primary Route:**
    /summarize/{text}/{max_length}
    "
}

#[get("/summarize/{text}/{max_length}")]
async fn change(info: web::Path<(u32, u32)>) -> HttpResponse{
    let (dollars, cents) = info.into_inner();
    let total_cents = dollars * 100 + cents;
    let change = greedy_coin_change(total_cents);
    let result = ChangeResult{
        dollars,
        cents,
        change
    };
    HttpResponse::Ok().json(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(root)
            .service(change)
            //.route("/change/{dollars}/{cents}", web::get().to(change))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

