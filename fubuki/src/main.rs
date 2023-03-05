use fubuki::{logger, Blog};

fn main() {
    logger::init();
    // CSR
    yew::Renderer::<Blog>::new().render();
}
