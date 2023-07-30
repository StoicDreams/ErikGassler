use crate::prelude::*;

use yew::{function_component, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub html: String,
}

#[function_component(HtmlContent)]
pub fn html_conntet(props: &Props) -> Html {
    let window = web_sys::window().expect("Missing Window");
    let document = window.document().expect("Missing Document");
    let div = document
        .create_element("div")
        .expect("Failed to create div");
    div.set_inner_html(&props.html.clone());
    div.set_class_name("paper d-flex align-center justify-center");
    _ = div.set_attribute("style", "height: auto;width: auto;");
    html! {
        <div class="paper d-flex align-center justify-center">
            {Html::VRef(div.into())}
        </div>
    }
}
