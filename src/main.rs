mod app;
mod textarea;
mod encoding_decoding;

use app::App;

fn main() {
    yew::start_app::<App>();
}
