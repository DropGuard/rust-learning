// ============================================================================
// HTTP 服务端 - Axum
// ============================================================================
//
// Axum 是现代化的 Rust Web 框架，基于 Tokio 和 Tower。
//
// 主要特点：
// 1. 路由系统 - RESTful API
// 2. 中间件 - 请求/响应处理
// 3. JSON 自动序列化
// 4. 提取器（Extractors）- 从请求中提取数据
// 5. WebSocket 支持
// 6. 状态共享
//
// 依赖：axum = "0.7", tokio = { version = "1", features = ["full"] }

// 注意：这些示例需要在 Cargo.toml 中添加 axum 依赖
// [dependencies]
// axum = "0.7"
// tokio = { version = "1", features = ["full"] }
// serde = { version = "1", features = ["derive"] }
// serde_json = "1"

// ============================================================================
// 示例 1: 最简单的 Hello World 服务
// ============================================================================
/*
use axum::{routing::get, Router};

async fn hello() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn example1_hello_world() {
    let app = Router::new().route("/", get(hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
*/

// ============================================================================
// 示例 2: 多个路由
// ============================================================================
/*
use axum::{
    routing::{get, post},
    Router,
    Json,
};
use serde_json::Value;

async fn root() -> &'static str {
    "欢迎使用 Axum！"
}

async fn hello() -> &'static str {
    "Hello, Axum!"
}

async fn goodbye() -> &'static str {
    "Goodbye!"
}

#[tokio::main]
async fn example2_multiple_routes() {
    let app = Router::new()
        .route("/", get(root))
        .route("/hello", get(hello))
        .route("/goodbye", get(goodbye));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3001");
    axum::serve(listener, app).await.unwrap();
}
*/

// ============================================================================
// 示例 3: JSON 请求和响应
// ============================================================================
/*
use axum::{
    routing::post,
    Router,
    Json,
    extract::Path,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    age: u32,
}

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
    age: u32,
}

static mut USER_ID: u32 = 0;

async fn create_user(Json(payload): Json<CreateUser>) -> Json<User> {
    unsafe {
        USER_ID += 1;
        Json(User {
            id: USER_ID,
            name: payload.name,
            age: payload.age,
        })
    }
}

#[tokio::main]
async fn example3_json_api() {
    let app = Router::new().route("/users", post(create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3002").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3002");
    println!("POST /users - 创建用户");
    println!("请求体: {\"name\":\"Alice\",\"age\":30}");
    axum::serve(listener, app).await.unwrap();
}
*/

// ============================================================================
// 示例 4: 路径参数
// ============================================================================
/*
use axum::{
    routing::get,
    Router,
    Path,
};

async fn get_user(Path(user_id): Path<String>) -> String {
    format!("用户 ID: {}", user_id)
}

async fn get_post(Path((user_id, post_id)): Path<(String, String)>) -> String {
    format!("用户 ID: {}, 文章 ID: {}", user_id, post_id)
}

#[tokio::main]
async fn example4_path_params() {
    let app = Router::new()
        .route("/users/:id", get(get_user))
        .route("/users/:id/posts/:post_id", get(get_post));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3003").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3003");
    println!("GET /users/:id - 获取用户");
    println!("GET /users/:id/posts/:post_id - 获取文章");
    axum::serve(listener, app).await.unwrap();
}
*/

// ============================================================================
// 示例 5: 查询参数（Query Parameters）
// ============================================================================
/*
use axum::{
    routing::get,
    Router,
    Query,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Pagination {
    page: Option<u32>,
    limit: Option<u32>,
}

async fn list_users(Query(params): Query<Pagination>) -> String {
    let page = params.page.unwrap_or(1);
    let limit = params.limit.unwrap_or(10);
    format!("第 {} 页，每页 {} 条", page, limit)
}

#[tokio::main]
async fn example5_query_params() {
    let app = Router::new().route("/users", get(list_users));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3004").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3004");
    println!("GET /users?page=1&limit=10 - 用户列表");
    axum::serve(listener, app).await.unwrap();
}
*/

// ============================================================================
// 示例 6: 表单数据处理
// ============================================================================
/*
use axum::{
    routing::post,
    Router,
    Form,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

async fn login(Form(form): Form<LoginForm>) -> String {
    format!("登录: {}", form.username)
}

#[tokio::main]
async fn example6_form_data() {
    let app = Router::new().route("/login", post(login));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3005").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3005");
    println!("POST /login - 登录");
    println!("Content-Type: application/x-www-form-urlencoded");
    axum::serve(listener, app).await.unwrap();
}
*/

// ============================================================================
// 示例 7: 请求头处理
// ============================================================================
/*
use axum::{
    routing::get,
    Router,
    TypedHeader,
    headers::UserAgent,
};

async fn get_headers(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
    format!("User-Agent: {}", user_agent.to_str().unwrap())
}

#[tokio::main]
async fn example7_headers() {
    let app = Router::new().route("/headers", get(get_headers));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3006").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3006");
    println!("GET /headers - 获取请求头");
    axum::serve(listener, app).await.unwrap();
}
*/

// ============================================================================
// 示例 8: 状态共享
// ============================================================================
/*
use axum::{
    routing::get,
    Router,
    extract::State,
};
use std::sync::Arc;
use tokio::sync::Mutex;

struct AppState {
    counter: Arc<Mutex<i32>>,
}

async fn get_counter(State(state): State<AppState>) -> String {
    let mut counter = state.counter.lock().await;
    *counter += 1;
    format!("计数: {}", *counter)
}

#[tokio::main]
async fn example8_state() {
    let state = AppState {
        counter: Arc::new(Mutex::new(0)),
    };

    let app = Router::new()
        .route("/counter", get(get_counter))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3007").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3007");
    println!("GET /counter - 获取计数器");
    axum::serve(listener, app).await.unwrap();
}
*/

// ============================================================================
// 示例 9: 中间件
// ============================================================================
/*
use axum::{
    routing::get,
    Router,
    middleware::{self, Next},
    http::Request,
    response::Response,
};
use std::time::Instant;

async fn my_middleware<B>(
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, Response> {
    let start = Instant::now();
    let uri = req.uri().clone();

    println!("{} - 开始处理", uri);

    let response = next.run(req).await;

    println!("{} - 完成，耗时 {:?}", uri, start.elapsed());

    Ok(response)
}

async fn hello() -> &'static str {
    "Hello with middleware!"
}

#[tokio::main]
async fn example9_middleware() {
    let app = Router::new()
        .route("/", get(hello))
        .layer(middleware::from_fn(my_middleware));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3008").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3008");
    println!("所有请求都会经过中间件");
    axum::serve(listener, app).await.unwrap();
}
*/

// ============================================================================
// 示例 10: CORS 跨域处理
// ============================================================================
/*
use axum::{
    routing::get,
    Router,
};
use tower_http::cors::{Any, CorsLayer};

async fn cors_handler() -> &'static str {
    "CORS enabled!"
}

#[tokio::main]
async fn example10_cors() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(cors_handler))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3009").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3009");
    println!("已启用 CORS");
    axum::serve(listener, app).await.unwrap();
}
*/

// ============================================================================
// 示例 11: 静态文件服务
// ============================================================================
/*
use axum::{
    routing::get,
    Router,
};
use tower_http::services::ServeDir;

#[tokio::main]
async fn example11_static_files() {
    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3010").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3010");
    println!("访问 /static/fileName 获取静态文件");
    axum::serve(listener, app).await.unwrap();
}
*/

// ============================================================================
// 示例 12: 日志中间件
// ============================================================================
/*
use axum::{
    routing::get,
    Router,
};
use tower_http::trace::TraceLayer;

async fn hello() -> &'static str {
    "Hello with logging!"
}

#[tokio::main]
async fn example12_logging() {
    let app = Router::new()
        .route("/", get(hello))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3011").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3011");
    println!("已启用请求日志");
    axum::serve(listener, app).await.unwrap();
}
*/

// ============================================================================
// 示例 13: 错误处理
// ============================================================================
/*
use axum::{
    routing::get,
    Router,
    Json,
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

async fn handler_with_error() -> Result<Json<serde_json::Value>, StatusCode> {
    // 模拟错误
    if true {
        Err(StatusCode::BAD_REQUEST)
    } else {
        Ok(Json(json!({ "message": "成功" })))
    }
}

#[tokio::main]
async fn example13_error_handling() {
    let app = Router::new().route("/", get(handler_with_error));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3012").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3012");
    axum::serve(listener, app).await.unwrap();
}
*/

// ============================================================================
// 示例 14: 文件上传
// ============================================================================
/*
use axum::{
    routing::post,
    Router,
    Multipart,
    extract::Multipart,
};
use std::io::Write;
use tokio::fs::File;

async fn upload(mut multipart: Multipart) -> Result<String, String> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let filename = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        let path = format!("uploads/{}", filename);

        tokio::fs::create_dir_all("uploads").await.unwrap();
        let mut file = File::create(&path).await.unwrap();
        file.write_all(&data).await.unwrap();

        return Ok(format!("上传成功: {}", path));
    }

    Ok("没有文件".to_string())
}

#[tokio::main]
async fn example14_file_upload() {
    let app = Router::new().route("/upload", post(upload));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3013").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3013");
    println!("POST /upload - 上传文件");
    axum::serve(listener, app).await.unwrap();
}
*/

// ============================================================================
// 示例 15: WebSocket 支持
// ============================================================================
/*
use axum::{
    routing::get,
    Router,
    extract::{
        State,
        ws::{WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use std::sync::Arc;
use tokio::sync::broadcast;

struct AppState {
    tx: broadcast::Sender<String>,
}

async fn ws_handler(
    State(state): State<AppState>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(
    mut socket: WebSocket,
    state: AppState,
) {
    let mut rx = state.tx.subscribe();

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if socket.send(axum::extract::ws::Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = socket.next().await {
            if let axum::extract::ws::Message::Text(text) = msg {
                println!("收到: {}", text);
            }
        }
    });

    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }
}

#[tokio::main]
async fn example15_websocket() {
    let (tx, _) = broadcast::channel(100);
    let state = AppState { tx };

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3014").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3014");
    println!("WebSocket: ws://127.0.0.1:3014/ws");
    axum::serve(listener, app).await.unwrap();
}
*/

// ============================================================================
// 示例 16: 路由嵌套（API 版本管理）
// ============================================================================
/*
use axum::{
    routing::{get, post},
    Router,
};

async fn v1_hello() -> &'static str {
    "API v1"
}

async fn v2_hello() -> &'static str {
    "API v2"
}

#[tokio::main]
async fn example16_nested_routes() {
    let app = Router::new()
        .route("/hello", get(v1_hello))
        .nest("/api/v2", Router::new()
            .route("/hello", get(v2_hello)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3015").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3015");
    println!("GET /hello - API v1");
    println!("GET /api/v2/hello - API v2");
    axum::serve(listener, app).await.unwrap();
}
*/

// ============================================================================
// 示例 17: 数据库集成（简化版）
// ============================================================================
/*
use axum::{
    routing::get,
    Router,
};
use std::collections::HashMap;

async fn get_users() -> String {
    let users = HashMap::from([
        ("1", "Alice"),
        ("2", "Bob"),
    ]);

    serde_json::to_string(&users).unwrap()
}

#[tokio::main]
async fn example17_database() {
    let app = Router::new().route("/users", get(get_users));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3016").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3016");
    println!("GET /users - 获取用户列表");
    axum::serve(listener, app).await.unwrap();
}
*/

// ============================================================================
// 示例 18: JWT 认证（简化版）
// ============================================================================
/*
use axum::{
    routing::get,
    Router,
    TypedHeader,
    headers::authorization::{Authorization, Bearer},
    http::StatusCode,
};

async fn protected(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Result<&'static str, StatusCode> {
    if auth.token() == "valid_token" {
        Ok("认证成功！")
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

#[tokio::main]
async fn example18_jwt_auth() {
    let app = Router::new().route("/protected", get(protected));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3017").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3017");
    println!("GET /protected - 需要认证");
    println!("Header: Authorization: Bearer valid_token");
    axum::serve(listener, app).await.unwrap();
}
*/

// ============================================================================
// 示例 19: 响应压缩
// ============================================================================
/*
use axum::{
    routing::get,
    Router,
};
use tower_http::compression::CompressionLayer;

async fn large_response() -> String {
    "A".repeat(10000)
}

#[tokio::main]
async fn example19_compression() {
    let app = Router::new()
        .route("/", get(large_response))
        .layer(CompressionLayer::new());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3018").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3018");
    println!("响应已启用压缩");
    axum::serve(listener, app).await.unwrap();
}
*/

// ============================================================================
// 示例 20: 速率限制（简化版）
// ============================================================================
/*
use axum::{
    routing::get,
    Router,
    extract::State,
};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::Mutex;
use std::net::SocketAddr;

struct RateLimiter {
    requests: Arc<Mutex<HashMap<String, u32>>>,
}

async fn rate_limited(
    State(limiter): State<RateLimiter>,
    addr: axum::extract::ConnectInfo<SocketAddr>,
) -> String {
    let ip = addr.ip().to_string();

    let mut requests = limiter.requests.lock().await;
    let count = requests.entry(ip).or_insert(0);
    *count += 1;

    if *count > 10 {
        "速率限制: 每分钟 10 次请求".to_string()
    } else {
        format!("请求次数: {}", *count)
    }
}

#[tokio::main]
async fn example20_rate_limit() {
    let limiter = RateLimiter {
        requests: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/limited", get(rate_limited))
        .with_state(limiter);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3019").await.unwrap();
    println!("服务运行在 http://127.0.0.1:3019");
    println!("GET /limited - 速率限制（每分钟 10 次）");
    axum::serve(listener, app).await.unwrap();
}
*/

// ============================================================================
// 主函数（仅作为展示，实际代码需要取消注释）
// ============================================================================
fn main() {
    println!("=== HTTP 服务端 - Axum 示例 ===\n");

    println!("注意: 以下示例需要在 Cargo.toml 中添加依赖:");
    println!("  [dependencies]");
    println!("  axum = \"0.7\"");
    println!("  tokio = {{ version = \"1\", features = [\"full\"] }}");
    println!("  serde = {{ version = \"1\", features = [\"derive\"] }}");
    println!("  serde_json = \"1\"");
    println!("  tower-http = {{ version = \"0.4\", features = [\"fs\", \"cors\", \"compression\", \"trace\"] }}");
    println!();

    println!("示例 1: 最简单的 Hello World");
    println!("  Router::new().route(\"/\", get(handler))\n");

    println!("示例 2: 多个路由");
    println!("  多个 .route() 调用\n");

    println!("示例 3: JSON 请求和响应");
    println!("  Json<T> 提取器和返回类型\n");

    println!("示例 4: 路径参数");
    println!("  Path<T> 提取器\n");

    println!("示例 5: 查询参数");
    println!("  Query<T> 提取器\n");

    println!("示例 6: 表单数据处理");
    println!("  Form<T> 提取器\n");

    println!("示例 7: 请求头处理");
    println!("  TypedHeader<T> 提取器\n");

    println!("示例 8: 状态共享");
    println!("  .with_state() 和 State<T> 提取器\n");

    println!("示例 9: 中间件");
    println!("  .layer(middleware::from_fn())\n");

    println!("示例 10: CORS 跨域处理");
    println!("  CorsLayer::new()\n");

    println!("示例 11: 静态文件服务");
    println!("  ServeDir::new(\"static\")\n");

    println!("示例 12: 日志中间件");
    println!("  TraceLayer::new_for_http()\n");

    println!("示例 13: 错误处理");
    println!("  Result<T, StatusCode>\n");

    println!("示例 14: 文件上传");
    println!("  Multipart 提取器\n");

    println!("示例 15: WebSocket 支持");
    println!("  WebSocketUpgrade.on_upgrade()\n");

    println!("示例 16: 路由嵌套");
    println!("  .nest(\"/api/v2\", router)\n");

    println!("示例 17: 数据库集成");
    println!("  结合 SQLx、Diesel 等\n");

    println!("示例 18: JWT 认证");
    println!("  Authorization<Bearer> 提取器\n");

    println!("示例 19: 响应压缩");
    println!("  CompressionLayer::new()\n");

    println!("示例 20: 速率限制");
    println!("  自定义中间件实现\n");

    println!("=== 总结 ===");
    println!("Axum 特点:");
    println!("  - 基于 Tokio 和 Tower");
    println!("  - 类型安全的路由");
    println!("  - 强大的提取器系统");
    println!("  - 中间件支持");
    println!("  - WebSocket 原生支持");
    println!("  - JSON 自动序列化");
    println!("  - 状态管理");
    println!("  - 高性能");
    println!("\n常用提取器:");
    println!("  - Path<T> - 路径参数");
    println!("  - Query<T> - 查询参数");
    println!("  - Json<T> - JSON 请求体");
    println!("  - Form<T> - 表单数据");
    println!("  - State<T> - 应用状态");
    println!("  - TypedHeader<T> - 请求头");
    println!("\n常用中间件:");
    println!("  - TraceLayer - 日志");
    println!("  - CorsLayer - CORS");
    println!("  - CompressionLayer - 压缩");
    println!("  - ServeDir - 静态文件");
}
