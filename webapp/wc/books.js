/* Display my books */
"use strict"
{
    const template = `
<webui-side-by-side>
    <webui-flex column align="center" justify="center">
        <webui-quote theme="info" cite="May 6th, 2023">
            Today, I am excited to announce the publishing of my first book "CASE: Continuous Agile Software Engineering", now available in <a href="https://amzn.to/3wutCPc">Kindle, Paperback, and Hardcover editions on Amazon</a>.
            This book documents my best practices and methodologies for engineering software that has allowed me to produce high quality software and features at a rate of productivity that has consistently been 10 to 100+ times higher than that of teams and peers I have worked with who used Scrum, Sprints, and other Agile methodologies.
            {LINK_TO_BLOG}
        </webui-quote>
    </webui-flex>
    <webui-flex column gap="2">
        <webui-paper class="image mb-2" style="max-height:500px">
            <image style="max-height:500px" src="https://cdn.myfi.ws/img/case/CASE_Digital_Cover.webp" />
        </webui-paper>
        <webui-flex gap="2" justify="center" wrap>
            <webui-paper elevation="10" class="pa-2">
                <a target="_blank" href="https://amzn.to/3JNRTD7" class="theme-secondary text-center pa-3">Kindle</a>
            </webui-paper>
            <webui-paper elevation="10" class="pa-2">
                <a target="_blank" href="https://amzn.to/3wutCPc" class="theme-tertiary text-center pa-3">Paperback</a>
            </webui-paper>
            <webui-paper elevation="10" class="pa-2">
                <a target="_blank" href="https://amzn.to/3WqBOdX" class="theme-primary text-center pa-3">Hardcover</a>
            </webui-paper>
        </webui-flex>
        <webui-paper class="text-center">
            <em>As an Amazon Associate I earn from qualifying purchases.</em>
        </webui-paper>
    </webui-flex>
</webui-side-by-side>
`;
    webui.define("app-books", {
        attr: ['link-to-blog'],
        connected: (t) => {
            t.innerHTML = webui.trimLinePreTabs(template.replace('{LINK_TO_BLOG}', t.isFlagged('linkToBlog') ? `
    <p>See <a href="/blogs/book_published_for_case_continuous_agile_software_engineering">my accouncement blog</a> for more details.</p>` : ''));
        }
    });
}