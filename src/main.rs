mod app;
mod config;
mod model;
mod routes;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // 构建应用（路由、静态资源等）
    let app = app::build_app();

    // 地址（以后可以从 config 读取）
    let addr: SocketAddr = "[::]:8880".parse().expect("解析地址失败");

    println!(">> 梨窝 已启动: http://{}", addr);

    // 启动服务
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app).await.expect("Server error");
}
