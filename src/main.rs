mod generate_kana_table;

use generate_kana_table::azik_config::AzikConfig;
use generate_kana_table::gen_kana_table::gen_hiragana_table;

pub use crate::generate_kana_table::gen_kana_table;
use axum::{http::StatusCode, routing::post, Json, Router};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", post(gen_kana_text));

    Ok(router.into())
}

async fn gen_kana_text(Json(payload): Json<AzikConfig>) -> (StatusCode, String) {
    let azik = gen_hiragana_table(payload);
    (StatusCode::OK, azik)
}
