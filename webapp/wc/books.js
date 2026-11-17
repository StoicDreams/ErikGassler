/* Display my books */
"use strict"
{
    // The cover and a link to each edition. With the cover flag this is all the component shows, for narrow spaces such as a pinned side.
    const cover = `
    <webui-flex column gap="2">
        <webui-paper class="image mb-2" style="max-height:500px">
            <img style="max-height:500px" src="${webui.cdnRoot || 'https://cdn.myfi.ws/'}img/case/CASE_Digital_Cover.webp" alt="CASE book digital cover" />
        </webui-paper>
        <webui-flex gap="2" justify="center" wrap>
            <webui-paper elevation="10" class="pa-2">
                <a target="_blank" href="https://www.amazon.com/dp/B0C4FX1F1S" class="theme-secondary text-center pa-3">Kindle</a>
            </webui-paper>
            <webui-paper elevation="10" class="pa-2">
                <a target="_blank" href="https://www.amazon.com/dp/B0C47LV1Y8" class="theme-tertiary text-center pa-3">Paperback</a>
            </webui-paper>
            <webui-paper elevation="10" class="pa-2">
                <a target="_blank" href="https://www.amazon.com/dp/B0C47YGGYH" class="theme-primary text-center pa-3">Hardcover</a>
            </webui-paper>
        </webui-flex>
    </webui-flex>
`;
    const template = `
<webui-side-by-side>
    <webui-flex column align="center" justify="center">
        <webui-quote theme="info" cite="May 6th, 2023">
            Today, I am excited to announce the publishing of my first book "CASE: Continuous Agile Software Engineering", now available in <a href="https://www.amazon.com/dp/B0C47LV1Y8">Kindle, Paperback, and Hardcover editions on Amazon</a>.
            It documents the practices I developed over my career for building quality software in small, tested steps and delivering it to users continuously.
            {LINK_TO_BLOG}
        </webui-quote>
    </webui-flex>
    ${cover}
</webui-side-by-side>
`;
    webui.define("app-books", {
        attr: ['link-to-blog'],
        flags: ['cover'],
        connected() {
            const t = this;
            if (t.hasAttribute('cover')) {
                // A shorter cover, so it and the edition links fit on screen beside a page.
                t.innerHTML = webui.trimLinePreTabs(cover.replaceAll('max-height:500px', 'max-height:min(360px, 45vh)'));
                return;
            }
            t.innerHTML = webui.trimLinePreTabs(template.replace('{LINK_TO_BLOG}', t.isFlagged('linkToBlog') ? `
    <p>See <a href="/blogs/book_published_for_case_continuous_agile_software_engineering">my announcement</a> for more details.</p>` : ''));
        }
    });
}
