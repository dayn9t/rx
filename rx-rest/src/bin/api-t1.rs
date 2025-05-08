use poem::{Route, listener::TcpListener};
use poem_openapi::{OpenApi, OpenApiService, param::Query, payload::PlainText};
use rx_rest::api::UrlPath;

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("hello, {}!", name)),
            None => PlainText("hello!\n".to_string()),
        }
    }

    #[oai(path = "/items/:id", method = "get")]
    async fn get_item(&self, id: UrlPath<i32>) -> PlainText<String> {
        PlainText(format!("{}", id.0))
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service =
        OpenApiService::new(Api, "Hello World", "1.0").server("http://localhost:3000/api");

    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", api_service).nest("/docs", ui);

    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
