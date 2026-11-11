<webui-data data-page-title="Case Study: Web UI" data-page-subtitle="Rebuilding a framework around what each language does best"></webui-data>
<webui-data data-page-description="How I built Web UI, the web component framework behind about two dozen websites, and why I rebuilt it from Rust rendering to vanilla JavaScript components with Rust workers."></webui-data>
<webui-data data-page-next-page='{"name":"How I Work","href":"/how-i-work"}'></webui-data>

<webui-page-segment elevation="10">

Web UI is the framework I built to run my company's websites, including this one. It started in 2022 as a Rust framework that rendered the UI in WebAssembly. In 2024 I rebuilt it as vanilla JavaScript web components, with Rust kept for the heavy processing it is good at. Today about two dozen websites run on it from one shared CDN, and every deploy is gated by automated tests.

This page covers the problem, the decision to rebuild, the design, and how it is kept reliable.

</webui-page-segment>

## The problem

<webui-page-segment elevation="10">

I run about two dozen websites through Stoic Dreams. Each one needs the same things: navigation, drawers, a header and footer, page routing, forms, data binding, and accessibility. Rebuilding that on every site, or copying it between sites, multiplies the code I have to maintain by the number of sites.

I wanted a new site to start with almost no boilerplate, and a fix in shared code to reach every site at once.

</webui-page-segment>

## Version one: Rust in the browser

<webui-page-segment elevation="10">

The first version, started in September 2022, was a Rust library built on the Yew framework, compiled to WebAssembly. Within the first two weeks I replaced Yew's router with my own navigation system, and the library grew to cover the shared layout and components for my sites.

It worked, but it had Rust building HTML. The rebuild followed a different principle: Rust should not build or manage the UI. It belongs in a worker thread doing long-running or processing-heavy work, while the UI stays in HTML, CSS, and JavaScript, the languages the browser was built around.

</webui-page-segment>

## The rebuild

<webui-page-segment elevation="10">

In May 2024 I began a vanilla JavaScript loader for web components, and in June 2024 I moved the framework over to it and retired the Rust rendering.

The design that came out of the rebuild:

- **Components that load themselves.** A page uses a tag such as `<webui-cards>`, and the loader fetches that component's script the first time the tag appears. Sites don't import or bundle anything. A site adds its own components with an `app-` prefix, and they load the same way from the site.
- **Pages written in markdown.** Each page is a markdown file that can contain components, so adding a page is writing one file.
- **Data binding in attributes.** Elements subscribe to and update shared data with `data-subscribe`, `data-trigger`, and `data-bind`, so most interactive pages need no page-specific JavaScript.
- **Rust where it earns its place.** Each site can run Rust compiled to WebAssembly in a worker, called from JavaScript with `webui.worker`. The browser tools on [Ap Tul](https://www.aptul.com) use this to process files on the device instead of uploading them.
- **One CDN for every site.** The framework is served from one CDN, so a fix or a new component reaches every site without changing them.

The framework now has about 70 components, from layout pieces like drawers and tabs to forms, tables, and dialogs.

</webui-page-segment>

## Keeping two dozen sites safe

<webui-page-segment elevation="10">

When every site loads the same framework, a mistake in the framework is a mistake on every site. That makes testing the core of the design, not an extra:

- **A framework test suite of over 700 Playwright tests**, which runs against both the source files and the minified files the sites load. A coverage test fails if any component or public `webui` method has no tests, so new code can't skip them.
- **Tests on every site's deploy.** Every push to main runs that site's own Playwright tests first: config and SEO files, links between pages, and that every page loads with a title, without errors, and with all its components. A failing test stops the deploy.
- **Pages for search engines and link previews.** The build writes a static HTML file for each page, with its own title, description, and text, so crawlers and link previews that don't run JavaScript still see the real page.

</webui-page-segment>

## Results

<webui-cards card-width="300">
    <webui-card name="About two dozen sites" avatar="emoji-building_construction" theme="primary">
        Run on one framework and one CDN, including this site, [Ap Tul](https://www.aptul.com), and [Task Proxy](https://www.taskproxy.com).
    </webui-card>
    <webui-card name="New pages in one file" avatar="emoji-memo" theme="primary">
        A page is one markdown file. A layout used across sites becomes one component, instead of markup and CSS copied into each page.
    </webui-card>
    <webui-card name="Every deploy tested" avatar="emoji-test_tube" theme="primary">
        No site deploys unless its tests pass, and the framework can't add a component or method without tests.
    </webui-card>
</webui-cards>

## What I'd tell a team facing the same choice

<webui-page-segment elevation="10">

- **Put each language where it is strongest.** Moving Rust out of the UI and into workers made both halves simpler.
- **Share through components, not copies.** A layout that two pages need is one component, so a fix lands everywhere at once.
- **Make the shared code the most tested code.** The more places depend on something, the more its tests are worth.

</webui-page-segment>

<webui-flex gap="2" class="my-3" wrap justify="center">
    <webui-link href="https://webui.stoicdreams.com" target="_blank" start-icon="emoji-globe_with_meridians">Web UI documentation</webui-link>
    <webui-link href="https://github.com/StoicDreams/WebUI" target="_blank" start-icon="emoji-laptop">Web UI on GitHub</webui-link>
    <webui-link href="/projects" start-icon="emoji-hammer_and_wrench">More projects</webui-link>
</webui-flex>
