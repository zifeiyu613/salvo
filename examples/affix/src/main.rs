use salvo::extra::affix;
use salvo::prelude::*;

#[fn_handler]
async fn hello_world(depot: &mut Depot) -> String {
    let config = depot.obtain::<Config>().unwrap();
    let custom_data = depot.get::<&str>("custom_data").unwrap();
    format!("Hello World\nConfig: {:#?}\nCustom Data: {}", config, custom_data)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    tracing::info!("Listening on http://127.0.0.1:7878");
    Server::new(TcpListener::bind("127.0.0.1:7878")).serve(route()).await;
}

#[derive(Default, Clone, Debug)]
struct Config {
    username: String,
    password: String,
}

fn route() -> Router {
    let config = Config {
        username: "root".to_string(),
        password: "pwd".to_string(),
    };
    Router::new()
        .hoop(affix::inject(config).insert("custom_data", "I love this world!"))
        .get(hello_world)
        .push(Router::with_path("hello").get(hello_world))
}
