use app;
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{
    payload::Json,
    types::{ParseFromJSON, ToJSON},
    Object, OpenApi, OpenApiService,
};

#[derive(Object)]
struct MyObject<T: ParseFromJSON + ToJSON> {
    value: T,
}

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/u8", method = "post")]
    async fn u8(&self, value: Json<MyObject<u8>>) -> Json<MyObject<u8>> {
        match app::execute().await {
            Ok(_) => println!("Success"),
            Err(e) => println!("Error: {}", e),
        }
        value
    }

    #[oai(path = "/string", method = "post")]
    async fn string(&self, value: Json<MyObject<String>>) -> Json<MyObject<String>> {
        Json(MyObject {
            value: "Hello, world!".to_string(),
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(Route::new().nest("/api", api_service).nest("/", ui))
        .await
}
