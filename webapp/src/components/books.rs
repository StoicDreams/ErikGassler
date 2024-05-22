use crate::prelude::*;

pub(crate) fn render_books(_contexts: &Contexts) -> Html {
    html!(
        <CaseBooks link_to_blog={true} />
    )
}

pub(crate) fn render_case_books(_contexts: &Contexts) -> Html {
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
            <Paper>
                <Quote color={Theme::Info} cite="May 6th, 2023">
                    <p>
                        <span>{r#"Today, I am excited to announce the publishing of my first book "CASE: Continuous Agile Software Engineering", now available in "#}</span>
                        <Link href="https://amzn.to/3wutCPc">{"Kindle, Paperback, and Hardcover editions on Amazon"}</Link>
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
            </Paper>
            <Paper class="d-flex flex-column ma-2">
                <Image src="https://cdn.myfi.ws/img/case/CASE_Digital_Cover.webp" class="mb-2" style="max-height:500px" />
                <Paper class="d-flex flex-row gap-2 justify-center flex-wrap">
                    <Paper elevation={ELEVATION_STANDARD} class="pa-2">
                        <Link target="_blank" href="https://amzn.to/3JNRTD7">
                            <Paper class="theme-secondary text-center pa-3">{"Kindle"}</Paper>
                        </Link>
                    </Paper>
                    <Paper elevation={ELEVATION_STANDARD} class="pa-2">
                        <Link target="_blank" href="https://amzn.to/3wutCPc">
                            <Paper class="theme-tertiary text-center pa-3">{"Paperback"}</Paper>
                        </Link>
                    </Paper>
                    <Paper elevation={ELEVATION_STANDARD} class="pa-2">
                        <Link target="_blank" href="https://amzn.to/3WqBOdX">
                            <Paper class="theme-primary text-center pa-3">{"Hardcover"}</Paper>
                        </Link>
                    </Paper>
                </Paper>
                <Paper class="text-center">
                    <em>{"As an Amazon Associate I earn from qualifying purchases."}</em>
                </Paper>
            </Paper>
        </Paper>
    }
}
