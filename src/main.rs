use actix_files::Files;
use actix_web::{get, post, web::{self, Data}, App, HttpServer, HttpResponse, Responder};

use std::sync::Mutex;

mod broadcaster;
use broadcaster::Broadcaster;

mod pccontroller;
use pccontroller::MouseRequest;
use pccontroller::KeyRequest;

#[get("/")]
async fn index() -> impl Responder {
    let content = include_str!("public/index.html");

    HttpResponse::Ok()
        .header("Content-Type", "text/html")
        .body(content)
}

#[post("/api/mouse")]
async fn api_mouse(req: web::Json<MouseRequest>) -> impl Responder {
    req.clone().process();
    format!("ok")
}

#[post("/api/key")]
async fn api_key(req: web::Json<KeyRequest>) -> impl Responder {
    req.clone().process();
    format!("ok")
}

/// Register a new client and return a response
async fn new_client(broadcaster: Data<Mutex<Broadcaster>>) -> impl Responder {
    let rx = broadcaster.lock().unwrap().new_client();

    HttpResponse::Ok()
        .set_header("Cache-Control", "no-store, must-revalidate")
        .set_header("Pragma", "no-cache")
        .set_header("Expires", "0")
        .set_header("Connection", "close")
        .set_header(
            "Content-Type",
            "multipart/x-mixed-replace;boundary=boundarydonotcross",
        )
        .streaming(rx)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let data = Broadcaster::create();

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(index)
            .service(api_mouse)
            .service(api_key)
                        .route("/streaming", web::get().to(new_client))
            .service(Files::new("/", "src/public/").prefer_utf8(true))
    })
    .bind("0.0.0.0:8080")
    .expect("Unable to bind port")
    .run()
    .await
}

mod app_context {
    use once_cell::sync::Lazy;
    use screenshots::Screen;

    pub static HEIGHT: Lazy<f32> = Lazy::new(|| {
        let screens = Screen::all();
        let main_screen = screens.unwrap()[0];
        let pixels = main_screen.capture().unwrap();
        pixels.height() as f32
    });
}