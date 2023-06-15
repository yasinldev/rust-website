use yew::prelude::*;

pub struct NavbarText {
    link_1: String,
    link_2: String,
    link_3: String,
    link_4: String,
}

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let create_navbar = NavbarText {
        link_1: "🏠 Home".to_string(),
        link_2: "📰 Blog".to_string(),
        link_3: "🎨 Github".to_string(),
        link_4: "🌐 Discord".to_string(),
    };

    html! {
        <>
            <div class="mai-container pt-4">
                <div class="w-full h-12 border-secondary rounded-3">
                    <div class="float-start pt-1 pr-10" style="padding-top: -9px">
                        <img src="https://avatars.githubusercontent.com/u/90653146?s=48&v=4" class="w-9 ml-5 mr-5 rounded-circle" />
                    </div>
                    <div class="float-end pr-5 pt-2 pb-2">
                        <a href="#" class="fw-bold text-dark" style="padding-top: -3px">{"😎🦀"}</a>
                    </div>
                    <div class="text-center pt-3 pb-3">
                        <a href="#" class="fw-bold pr-5 pl-5">{create_navbar.link_1}</a>
                        <a href="#" class="fw-bold pr-5 pl-5">{create_navbar.link_2}</a>
                        <a href="https://github.com/yasinldev" class="fw-bold pr-5 pl-5 media-none">{create_navbar.link_3}</a>
                        <a href="https://discord.gg/UhwChu7e6Y" class="fw-bold pr-5 pl-5 media-none">{create_navbar.link_4}</a>
                    </div>
                </div>
            </div>
        </>
    }
}