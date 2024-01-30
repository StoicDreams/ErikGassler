use crate::prelude::*;

pub(crate) fn render_books(_contexts: Contexts) -> Html {
    html!(
        <CaseBooks link_to_blog={true} />
    )
}

pub(crate) fn render_case_books(_contexts: Contexts) -> Html {
    html!(
        <CaseBooks />
    )
}

#[derive(Clone, Properties, PartialEq)]
pub(crate) struct CaseBooksProps {
    #[prop_or_default]
    pub(crate) link_to_blog: bool,
}

#[function_component(CaseBooks)]
pub(crate) fn case_books(props: &CaseBooksProps) -> Html {
    html! {
        <Paper class={format!("{} gap-2", CLASSES_SIDE_BY_SIDE)}>
            <Quote color={Theme::Info} cite="May 6th, 2023">
                <p>
                    <span>{r#"Today, I am excited to announce the publishing of my first book "CASE: Continuous Agile Software Engineering", now available in "#}</span>
                    <Link href="https://amzn.to/3VzXW32">{"Kindle, Paperback, and Hardcover editions on Amazon"}</Link>
                    <span>{"."}</span>
                </p>
                <p>
                    {"This book documents my best practices and methodologies for engineering software that has allowed me to produce high quality software and features at a rate of productivity that has consistently been 10 to 100+ times higher than that of teams and peers I have worked with who used Scrum, Sprints, and other Agile methodologies."}
                </p>
                {if props.link_to_blog {
                    html! {
                        <p>
                            <span>{"See "}</span>
                            <Link href="/blogs/book_published_for_case_continuous_agile_software_engineering">{"my accouncement blog"}</Link>
                            <span>{" for more details."}</span>
                        </p>
                    }
                } else {
                    html! {}
                }}
            </Quote>
            <Paper class="d-flex flex-column ma-2">
                <Paper class="d-flex flex-row gap-2 justify-center flex-wrap">
                    <Paper elevation={ELEVATION_STANDARD} class="pa-2">
                        <Link target="_blank" href="https://amzn.to/48S30VU">
                            <Paper class="theme-secondary text-center pa-3">{"Kindle"}</Paper>
                        </Link>
                    </Paper>
                    <Paper elevation={ELEVATION_STANDARD} class="pa-2">
                        <Link target="_blank" href="https://amzn.to/3Sgpyu3">
                            <Paper class="theme-tertiary text-center pa-3">{"Paperback"}</Paper>
                        </Link>
                    </Paper>
                    <Paper elevation={ELEVATION_STANDARD} class="pa-2">
                        <Link target="_blank" href="https://amzn.to/3vue8db">
                            <Paper class="theme-primary text-center pa-3">{"Hardcover"}</Paper>
                        </Link>
                    </Paper>
                </Paper>
                <Paper class="text-center">
                    <em>{"Paid links"}</em>
                </Paper>
            </Paper>
        </Paper>
    }
}
