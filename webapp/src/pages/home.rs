use crate::prelude::*;

/// App home page
pub(crate) fn page_home(_contexts: &Contexts) -> Html {
    set_title("Professional Software Engineer & Website Developer");
    let video = r#"<iframe width="3840" height="2160" src="https://www.youtube-nocookie.com/embed/IVPHtC0H2fU" title="YouTube video player" style="width:calc(0.8 * (var(--window-width) - var(--drawer-left-width) - var(--drawer-right-width)));height:calc(0.5625 * 0.8 * (var(--window-width) - var(--drawer-left-width) - var(--drawer-right-width)));" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>"#;
    html! {
        <>
            <MarkdownContent href="/d/en-US/home.md" />
            <Paper class="ma-2">
                <HtmlContent html={video} />
            </Paper>
            <MarkdownContent href="/d/en-US/cards/interest_cards.md" />
            <MarkdownContent markdown={r#"
                `````sideimage "left" "https://cdn.myfi.ws/img/case/CASE_Hardcover_Circle.webp"

                ````quote "secondary" "Erik Gassler"
                Curious about CASE but want a format where you can ask questions about it instead of reading the book? Then check out the [CASE GPT AI assistant](https://chat.openai.com/g/g-V2lj0ZVcU-case-gpt).

                ```paper "d-flex justify-center mt-4 mb-3"
                https://chat.openai.com/g/g-V2lj0ZVcU-case-gpt
                ```
                ````

                `````
                "#} />
            <NextPageButton url="/software-engineering-standards-and-practices" />
        </>
    }
}
