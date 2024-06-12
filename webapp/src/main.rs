#![allow(unused)] // TODO: Remove me when needing to check for dead code / unused methods/variables.
mod components;
mod nav_menu;
mod pages;
mod prelude;

use prelude::*;

fn main() {
    webui::start_app(setup_app_config());
}

fn setup_app_config() -> AppConfig {
    AppConfig::builder(
        "Erik Gassler".to_owned(),
        "Stoic Dreams".to_owned(),
        "https://www.stoicdreams.com".to_owned(),
        "ErikGassler.com".to_owned(),
    )
    .set_header_logo_src("Logo.svg".to_owned())
    .set_nav_routing(NavRoutingCallback::new(nav_menu::get_nav_routing))
    .set_navigation(nav_menu::nav_content)
    .set_header(myfi_app_header)
    .set_copyright_start(2013)
    .register_component("CaseBooks", render_case_books)
    .register_component("Books", render_books)
    .build()
}
