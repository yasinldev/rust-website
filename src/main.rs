use yew::prelude::*;
use crate::components::navbar::Navbar;
use crate::components::header::Header;

mod components {
    pub mod navbar;
    pub mod header;
}

#[function_component(App)]
fn app() -> Html {

    html! {
        <>
            <Navbar />
            <Header />
        </>
    }
}

fn main() {
    yew::start_app::<Navbar>();
    yew::start_app::<Header>();
    yew::start_app::<App>();
}
