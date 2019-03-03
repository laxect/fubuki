use std::env;
use actix_web::{App, fs, server, Result, HttpRequest, http::Method, fs::NamedFile};

fn index(_req: &HttpRequest) -> Result<NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
}

fn main() {
    // server init
    let port = match env::var("PORT") {
        Ok(p_str) => p_str,
        _ => String::from("3000"),
    };
    let mut bind_address = String::from("0.0.0.0:");
    bind_address.push_str(port.as_str());
    println!("Server listen on {}", bind_address);

    server::new(
        || App::new()
            .resource(r"/", |r| r.method(Method::GET).f(index))
            .handler(
                "/",
                fs::StaticFiles::new("./static")
                    .unwrap())
    )
    .bind(bind_address).unwrap()
    .run();
}
