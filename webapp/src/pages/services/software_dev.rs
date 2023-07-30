use crate::prelude::*;

/// App home page
pub(crate) fn page_services_software_dev(_contexts: Contexts) -> Html {
    set_title("Software Development Services");
    html! {
        <>
            <MarkdownContent href="/d/en-US/services/software_dev.md" />
            <NextPageButton url="/how-i-can-help-improve-your-teams-software-development-productivity" />
        </>
    }
}
