use crate::prelude::*;

/// App home page
pub(crate) fn page_home(_contexts: Contexts) -> Html {
    set_title("Professional Software Engineer & Website Developer");
    let video = r#"<iframe width="3840" height="2160" src="https://www.youtube-nocookie.com/embed/IVPHtC0H2fU" title="YouTube video player" style="width:calc(0.8 * var(--window-width));height:calc(0.5625 * 0.8 * var(--window-width));" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>"#;
    html! {
        <>
            <MarkdownContent href="/d/en-US/home.md" />
            <Paper class="ma-2">
                <HtmlContent html={video} />
            </Paper>
            <MarkdownContent href="/d/en-US/cards/interest_cards.md" />
            <NextPageButton url="/software-engineering-standards-and-practices" />
        </>
    }
}
