use crate::prelude::*;

/// App home page
pub(crate) fn page_info_my_dev_standards(_contexts: Contexts) -> Html {
    set_title("Software Engineering Standards & Practices");
    html! {
        <>
            <MarkdownContent href="/d/en-US/cards/case_concepts_and_benefits.md" />
            <NextPageButton url="/about" />
        </>
    }
}
