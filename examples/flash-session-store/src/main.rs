use std::fmt::Write;

use salvo::prelude::*;
use salvo_flash::{FlashDepotExt, SessionStore};

#[handler]
pub async fn set_flash(depot: &mut Depot, res: &mut Response) {
    let flash = depot.outgoing_flash_mut();
    flash.info("Hey there!").debug("How is it going?");
    res.render(Redirect::other("/get").unwrap());
}

#[handler]
pub async fn get_flash(depot: &mut Depot, _res: &mut Response) -> String {
    let mut body = String::new();
    if let Some(flash) = depot.incoming_flash() {
        for message in flash.iter() {
            writeln!(body, "{} - {}", message.value, message.level).unwrap();
        }
    }
    body
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    tracing::info!("Listening on http://127.0.0.1:7878");
    let session_handler = salvo_session::SessionHandler::builder(
        salvo_session::MemoryStore::new(),
        b"secretabsecretabsecretabsecretabsecretabsecretabsecretabsecretab",
    )
    .build()
    .unwrap();
    let router = Router::new()
        .hoop(session_handler)
        .hoop(SessionStore::new().into_handler())
        .push(Router::with_path("get").get(get_flash))
        .push(Router::with_path("set").get(set_flash));
    Server::new(TcpListener::bind("127.0.0.1:7878")).serve(router).await;
}
