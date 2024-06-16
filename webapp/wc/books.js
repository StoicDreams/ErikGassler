/* Display my books */
"use strict"
{
    const template = `
<webui-side-by-side>
<webui-flex column justify="center">
<webui-quote theme="info" cite="May 6th, 2023">
<p>Today, I am excited to announce the publishing of my first book "CASE: Continuous Agile Software Engineering", now available in <a href="https://amzn.to/3wutCPc">Kindle, Paperback, and Hardcover editions on Amazon</a>.</p>
<p>This book documents my best practices and methodologies for engineering software that has allowed me to produce high quality software and features at a rate of productivity that has consistently been 10 to 100+ times higher than that of teams and peers I have worked with who used Scrum, Sprints, and other Agile methodologies.</p>
{LINK_TO_BLOG}
</webui-quote>
</webui-flex>
<webui-flex column>
<webui-paper class="image mb-2" style="max-height:500px">
<image src="https://cdn.myfi.ws/img/case/CASE_Digital_Cover.webp" />
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
    const linkToBlogTemplate = `
<p>See <a href="/blogs/book_published_for_case_continuous_agile_software_engineering">my accouncement blog</a> for more details.</p>
`;
    class Books extends HTMLElement {
        constructor() {
            super();
        }
        static get observedAttributes() {
            return ['link-to-blog'];
        }
        attributeChangedCallback(property, oldValue, newValue) {
            if (oldValue === newValue) return;
            if (newValue === null || newValue === undefined) {
                delete this[property];
            } else {
                this[property] = newValue;
            }
            switch (property) {
                case 'link-to-blog':
                    this.linkToBlog = true;
                    break;
            }
        }
        connectedCallback() {
            if (this.parentNode.nodeName === 'P') {
                this.parentNode.parentNode.insertBefore(this, this.parentNode);
            }
            this.innerHTML = template.replace('{LINK_TO_BLOG}', this.linkToBlog ? linkToBlogTemplate : '');
        }
        disconnectedCallback() { }
    }
    customElements.define('app-books', Books);
}