use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Expense {
    id: i32,
    description: String,
    amount: f64,
}

async fn get_expenses() -> impl Responder {
    web::Json(vec![
        Expense { id: 1, description: "Groceries".into(), amount: 50.0 },
        Expense { id: 2, description: "Rent".into(), amount: 1000.0 },
    ])
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/api/expenses", web::get().to(get_expenses))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
