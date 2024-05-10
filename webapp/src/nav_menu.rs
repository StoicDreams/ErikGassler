use crate::prelude::*;

pub fn nav_menu_info() -> DrawerToggleInfo {
    DrawerToggleInfo::builder(
        |_| String::from("Navigation Menu"),
        |_| html! {FaIcon::solid("bars").to_html()},
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
        NavLinkInfo::link(
            "Home",
            "/",
            &FaIcon::duotone("house"),
            roles::PUBLIC,
            page_home,
        ),
        NavLinkInfo::link(
            "CASE Book",
            "/blogs/book_published_for_case_continuous_agile_software_engineering",
            &FaIcon::duotone("book-sparkles"),
            roles::PUBLIC,
            page_blog_case_book,
        ),
        NavLinkInfo::link(
            "Software Engineering Standards & Practices",
            "/software-engineering-standards-and-practices",
            &FaIcon::duotone("book-open"),
            roles::PUBLIC,
            page_info_my_dev_standards,
        ),
        NavGroupInfo::link(
            "Contract Services",
            &FaIcon::duotone("bell-concierge"),
            roles::INVALID,
            vec![
                NavLinkInfo::link(
                    "Consulting",
                    "/how-i-can-help-improve-your-teams-software-development-productivity",
                    &FaIcon::duotone("hand-holding-seedling"),
                    roles::PUBLIC,
                    page_services_consulting,
                ),
                NavLinkInfo::link(
                    "Software Development",
                    "/software-development-services",
                    &FaIcon::duotone("window"),
                    roles::PUBLIC,
                    page_services_software_dev,
                ),
            ],
        ),
        NavGroupInfo::link(
            "Blogs",
            &FaIcon::duotone("blog"),
            roles::PUBLIC,
            vec![NavLinkInfo::link(
                "Worth $750k",
                "/blogs/why-as-a-principal-softaware-engineer-i-am-worth-a-high-salary",
                &FaIcon::duotone("money-bill-wave"),
                roles::PUBLIC,
                page_blogs_worth_750k,
            )],
        ),
        NavLinkInfo::link(
            "About Erik Gassler",
            "/about",
            &FaIcon::duotone("circle-info"),
            roles::PUBLIC,
            page_about,
        ),
        NavLinkInfo::link(
            "Terms",
            "/terms",
            &FaIcon::duotone("handshake"),
            roles::PUBLIC,
            starter_page_terms,
        ),
        NavLinkInfo::link(
            "Privacy",
            "/privacy",
            &FaIcon::duotone("shield-exclamation"),
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
