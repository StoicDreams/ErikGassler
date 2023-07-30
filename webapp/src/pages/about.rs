use crate::prelude::*;

/// App page body component - page specific content is rendered here
pub(crate) fn page_about(_contexts: Contexts) -> Html {
    set_title("About Erik Gassler");
    html! {
        <>
            <MarkdownContent href="/d/en-US/about.md" />
            <NextPageButton url="/" />
        </>
    }
}
