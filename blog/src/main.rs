#![recursion_limit = "4096"]

fn main() {
    web_logger::custom_init(web_logger::Config {
        level: log::Level::Info,
    });
    yew::start_app::<blog::Blog>();
}
