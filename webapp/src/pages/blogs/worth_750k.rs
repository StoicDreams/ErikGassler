use crate::prelude::*;

/// App home page
pub(crate) fn page_blogs_worth_750k(_contexts: &Contexts) -> Html {
    set_title("Why as a Principal Software Engineer I am worth a $750,000 yearly salary");
    html! {
        <>
            <MarkdownContent href="/d/en-US/blogs/worth_750k.md" />
            <CaseBooks />
            <NextPageButton url="/blogs/book_published_for_case_continuous_agile_software_engineering" />
        </>
    }
}
