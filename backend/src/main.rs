use axum::{
    routing::get,
    Router,
    Json,
};
use serde::Serialize;
use std::env;
use tower_http::cors::{Any, CorsLayer};

#[derive(Serialize)]
struct HelloResponse {
    message: String,
}

#[tokio::main]
async fn main() {
    // 1. .envファイルから環境変数を読み込む
    dotenvy::dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    // 2. CORSの設定（Reactからの通信を許可する）
    // ※本番環境ではAnyではなく、VercelのURLのみを許可するように後で変更します
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // 3. ルーティングの設定
    let app = Router::new()
        .route("/api/hello", get(hello_handler))
        .layer(cors); // ルーターにCORS設定を適用

    // 4. サーバーの起動
    // 注: Dockerで動かすことを見据え、"127.0.0.1" ではなく "0.0.0.0" にバインドします
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Backend server is running on http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

// /api/hello にアクセスが来た時の処理
async fn hello_handler() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "Hello from Axum! クリエイター交流プラットフォームの裏側へようこそ！".to_string(),
    })
}