use fubuki::{logger, Blog};

fn main() {
    logger::init();
    yew::start_app::<Blog>();
}
