use crate::prelude::*;

pub fn nav_menu_info() -> DrawerToggleInfo {
    DrawerToggleInfo::builder(
        |_| String::from("Navigation Menu"),
        |_| html! {<i class="fa-solid fa-bars"></i>},
        DynContextsHtml::new(nav_menu_render),
    )
    .set_button_class("btn toggle theme-inherit")
    .hide_header()
    .hide_footer()
    .set_drawer(Direction::Left)
    .build()
}

pub(crate) fn get_nav_routing(_contexts: &Contexts) -> Vec<NavRoute> {
    let nav_routes = vec![
        NavLinkInfo::link("Home", "/", "fa-duotone fa-house", roles::PUBLIC, page_home),
        NavLinkInfo::link(
            "CASE Book",
            "/blogs/book_published_for_case_continuous_agile_software_engineering",
            "fa-duotone fa-book-sparkles",
            roles::PUBLIC,
            page_blog_case_book,
        ),
        NavLinkInfo::link(
            "Software Engineering Standards & Practices",
            "/software-engineering-standards-and-practices",
            "fa-duotone fa-book-open",
            roles::PUBLIC,
            page_info_my_dev_standards,
        ),
        NavGroupInfo::link(
            "Contract Services",
            "fa-duotone fa-bell-concierge",
            roles::INVALID,
            vec![
                NavLinkInfo::link(
                    "Consulting",
                    "/how-i-can-help-improve-your-teams-software-development-productivity",
                    "fa-duotone fa-hand-holding-seedling",
                    roles::PUBLIC,
                    page_services_consulting,
                ),
                NavLinkInfo::link(
                    "Software Development",
                    "/software-development-services",
                    "fa-duotone fa-window",
                    roles::PUBLIC,
                    page_services_software_dev,
                ),
            ],
        ),
        NavGroupInfo::link(
            "Blogs",
            "fa-duotone fa-blog",
            roles::PUBLIC,
            vec![NavLinkInfo::link(
                "Worth $750k",
                "/blogs/why-as-a-principal-softaware-engineer-i-am-worth-a-high-salary",
                "fa-duotone fa-money-bill-wave",
                roles::PUBLIC,
                page_blogs_worth_750k,
            )],
        ),
        NavLinkInfo::link(
            "About Erik Gassler",
            "/about",
            "fa-duotone fa-circle-info",
            roles::PUBLIC,
            page_about,
        ),
        NavLinkInfo::link(
            "Terms",
            "/terms",
            "fa-duotone fa-handshake",
            roles::PUBLIC,
            starter_page_terms,
        ),
        NavLinkInfo::link(
            "Privacy",
            "/privacy",
            "fa-duotone fa-shield-exclamation",
            roles::PUBLIC,
            starter_page_privacy,
        ),
    ];
    nav_routes
}

fn nav_menu_render(contexts: &Contexts) -> Html {
    html! {
        <>
            <Paper class="logo d-flex pa-1 justify-center ml-a mr-a">
                <AppLogo text="Erik" title="Erik Gassler Website Logo" />
            </Paper>
            <NavDisplay routes={get_nav_routing(contexts)} class="d-flex flex-column pa-1" />
        </>
    }
}
