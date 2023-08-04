use crate::prelude::*;

/// App home page
pub(crate) fn page_blog_case_book(_contexts: Contexts) -> Html {
    set_title("CASE: Continuous Agile Software Engineering");
    html! {
        <>
            <MarkdownContent href="/d/en-US/blogs/case_book_intro.md" />
            <CaseBooks />
            <MarkdownContent href="/d/en-US/blogs/case_book.md" />
            <NextPageButton url="/" />
        </>
    }
}
