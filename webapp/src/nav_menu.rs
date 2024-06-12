use crate::prelude::*;

pub fn nav_content(contexts: &Contexts) -> Html {
    html! {
        <>
            <webui-flex justify="center" slot="header">
                <webui-stoic-dreams-logo title="Erik Gassler Website Logo" text="Erik"></webui-stoic-dreams-logo>
            </webui-flex>
            <NavDisplay routes={get_nav_routing(contexts)} class="d-flex flex-column pa-1" />
        </>
    }
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
            "SWE Standards & Practices",
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
