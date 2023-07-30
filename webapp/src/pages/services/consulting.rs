use crate::prelude::*;

/// App home page
pub(crate) fn page_services_consulting(_contexts: Contexts) -> Html {
    set_title("Consulting Services for Agile Software Development & Continuous Delivery Workflows");
    html! {
        <>
            <MarkdownContent href="/d/en-US/services/consulting.md" />
            <NextPageButton url="/software-development-services" />
        </>
    }
}
