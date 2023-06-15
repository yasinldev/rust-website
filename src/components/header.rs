use yew::prelude::*;

pub struct HeaderText {
    header_title: String,
    header_text: String,
    header_sm: String,
    header_emoji: char,
}

pub struct Cards {
    card_title: String,
    card_desc: String,
}

// you can change text's

impl Cards {
    pub fn card_1() -> Cards {
        Cards {
            card_title: String::from("About me"),
            card_desc: String::from("Hello! My name is Yasin. I'm passionate about exploring the fascinating world of language to express myself and connect with others. Software development, technology, literature, and art are important areas of interest for me. I consider myself curious and open-minded, always eager to learn. Engaging with people and building connections is something I truly enjoy. I believe that constant growth and embracing new experiences are key to personal and professional development. Let's connect and embark on a journey of knowledge and discovery together!"),
        }
    }

    pub fn cards_2() -> Cards {
        Cards {
            card_title: String::from("The Technologies I Use"),
            card_desc: String::from("I have experience with PHP, SCSS, and Rust. PHP is commonly used for web development, enabling dynamic websites. SCSS is a powerful extension of CSS, enhancing styling with variables, mixins, and nesting, i used in Mai UI. Rust is a systems programming language known for safety, performance, and concurrency. It's ideal for network programming and systems development. Currently, I am focused on enhancing my skills with Rust. These languages help me tackle diverse projects and grow as a developer.")
        }
    }

    pub fn cards_3() -> Cards {
        Cards {
            card_title: String::from(""),
            card_desc: String::from("")
        }
    }
}

#[function_component(Header)]
pub fn header() -> Html {
    let use_header = HeaderText {
        header_title: String::from("Hi, I am Yasin"),
        header_text: String::from("I have been trying to be a good developer \n for +3 years  ^^"),
        header_sm: String::from("This site developed with WASM and designed by Mai UI"),
        header_emoji: '🫡',
    };

    let use_card_1 = Cards::card_1();
    let use_card_2 = Cards::cards_2();

    static ADD_FONT_FAMILY: &str = "font-family: 'Open Sans', sans-serif;";

    #[warn(unused_variables)]
    static CENTER: &str = "display: flex; justify-content: center; align-items: center;";

    html! {
        <div class="mai-container is-fluid mt-8">
            <div class="text-center fw-bold fs-7" style={ADD_FONT_FAMILY}>{use_header.header_title} {" "} {use_header.header_emoji}</div>
            <div class="text-center fw-bold fs-6 mt-8" style={ADD_FONT_FAMILY}>{use_header.header_text}</div>
            <div class="text-center text-error fw-bold mt-5" style={ADD_FONT_FAMILY}>{use_header.header_sm}</div>

            <div class="mt-8"></div>

            <div class="mai-row mt-8">
                <div class="column-lg-4 column-md-4 column-sm-12 column-xs-12 d-flex justify-content-center align-items-center mt-6">
                    <div class="mai-card-default mt-7 rounded-3">
                        <div class="card-header">
                            <div class="float-end mr-2"><i class="bi bi-heart"></i></div>
                            <div class="card-title fw-bold" style={ADD_FONT_FAMILY}>{use_card_1.card_title}</div>
                        </div>
                        <div class="card-body fw-bold">
                            {use_card_1.card_desc}
                        </div>
                    </div>
                </div>
                <div class="column-lg-4 column-md-4 column-sm-12 column-xs-12 d-flex pb-10 justify-content-center align-items-center">
                    <div class="mai-card-default mt-7 rounded-3">
                        <div class="card-header">
                            <div class="float-end mr-2"><i class="bi bi-heart-fill text-error"></i></div>
                            <div class="card-title fw-bold" style={ADD_FONT_FAMILY}>{use_card_2.card_title}</div>
                        </div>
                        <div class="card-body fw-bold">
                            {use_card_2.card_desc}
                        </div>
                    </div>
                </div>
                <div class="column-lg-4 column-md-4 column-sm-12 column-xs-12 d-flex justify-content-center align-items-center">
                    <div class="mai-card-default mt-7 rounded-3">
                        <div class="card-title fw-bold" style={ADD_FONT_FAMILY}>{""}</div>
                    </div>
                </div>
            </div>
        </div>
    }
}
