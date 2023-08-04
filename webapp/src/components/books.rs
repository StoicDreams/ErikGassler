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
            <Paper class="d-flex flex-row ma-2 gap-2 justify-center flex-wrap">
                <Paper elevation={ELEVATION_STANDARD} class="pa-2">
                    <Paper class="theme-secondary text-center pa-3 mb-2">{"Kindle"}</Paper>
                    <HtmlContent html={r#"
                    <iframe sandbox="allow-popups allow-scripts allow-modals allow-forms allow-same-origin" style="width:120px;height:240px;" marginwidth="0" marginheight="0" scrolling="no" frameborder="0" src="//ws-na.amazon-adsystem.com/widgets/q?ServiceVersion=20070822&OneJS=1&Operation=GetAdHtml&MarketPlace=US&source=ss&ref=as_ss_li_til&ad_type=product_link&tracking_id=erikgassler-20&language=en_US&marketplace=amazon&region=US&placement=B0C4FX1F1S&asins=B0C4FX1F1S&linkId=f90be5ed0aa2e0038a6cddb219a5487c&show_border=true&link_opens_in_new_window=true"></iframe>
                    "#} />
                </Paper>
                <Paper elevation={ELEVATION_STANDARD} class="pa-2">
                    <Paper class="theme-tertiary text-center pa-3 mb-2">{"Paperback"}</Paper>
                    <HtmlContent html={r#"
                    <iframe sandbox="allow-popups allow-scripts allow-modals allow-forms allow-same-origin" style="width:120px;height:240px;" marginwidth="0" marginheight="0" scrolling="no" frameborder="0" src="//ws-na.amazon-adsystem.com/widgets/q?ServiceVersion=20070822&OneJS=1&Operation=GetAdHtml&MarketPlace=US&source=ss&ref=as_ss_li_til&ad_type=product_link&tracking_id=erikgassler-20&language=en_US&marketplace=amazon&region=US&placement=B0C47LV1Y8&asins=B0C47LV1Y8&linkId=73a4b5d19008b9eac1c02ba85efcb29c&show_border=true&link_opens_in_new_window=true"></iframe>
                    "#} />
                </Paper>
                <Paper elevation={ELEVATION_STANDARD} class="pa-2">
                    <Paper class="theme-primary text-center pa-3 mb-2">{"Hardcover"}</Paper>
                    <HtmlContent html={r#"
                    <iframe sandbox="allow-popups allow-scripts allow-modals allow-forms allow-same-origin" style="width:120px;height:240px;" marginwidth="0" marginheight="0" scrolling="no" frameborder="0" src="//ws-na.amazon-adsystem.com/widgets/q?ServiceVersion=20070822&OneJS=1&Operation=GetAdHtml&MarketPlace=US&source=ss&ref=as_ss_li_til&ad_type=product_link&tracking_id=erikgassler-20&language=en_US&marketplace=amazon&region=US&placement=B0C47YGGYH&asins=B0C47YGGYH&linkId=ad750159f22ea299d1ed66f4b21538cc&show_border=true&link_opens_in_new_window=true"></iframe>
                    "#} />
                </Paper>
            </Paper>
        </Paper>
    }
}
