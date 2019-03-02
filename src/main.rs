use actix_web::{App, fs, server};

fn main() {
    server::new(
        || App::new()
            .handler(
                "/",
                fs::StaticFiles::new("./static")
                    .unwrap())
    )
    .bind("127.0.0.1:8080").unwrap()
    .run();
}
