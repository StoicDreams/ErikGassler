<webui-data data-page-title="Projects" data-page-subtitle="Tools, frameworks, and platforms I've built"></webui-data>
<webui-data data-page-description="Tools, frameworks, and platforms I've built: Fabric Engine, the Web UI framework, a platform for about two dozen websites, Task Proxy, and more."></webui-data>
<webui-data data-page-next-page='{"name":"How I Work","href":"/how-i-work"}'></webui-data>

<webui-page-segment elevation="10">

Alongside my day job, I build and run my own software through my companies, Stoic Dreams and Wyrdloom Studios. These projects are where I set my own standards: small, tested changes that deploy automatically, and tools that remove friction for the people using them.

</webui-page-segment>

## Featured

<webui-cards card-width="340">
    <webui-card name="Fabric Engine" avatar="emoji-video_game" theme="primary" link="/fabric-engine">
        A 3D game engine and world-building suite in Rust and WGSL. Models are trees of mathematical shapes drawn exactly on the GPU, then snapped to a voxel grid for a stylized 3D pixel-art look. It powers Faryn, Wyrdloom Studios' first game.

        **Rust, WGSL, wgpu, winit**
    </webui-card>
    <webui-card name="Web UI" avatar="emoji-building_construction" theme="primary" link="https://webui.stoicdreams.com">
        A vanilla JavaScript web component framework, with about 70 components, a data binding system, and Rust WebAssembly workers for heavy processing. Pages are written in markdown, and components load themselves on first use, so a new site starts with almost no boilerplate.

        **JavaScript, HTML, CSS, Rust, WebAssembly**

        [Read the case study](/case-studies/web-ui)
    </webui-card>
    <webui-card name="A platform for two dozen sites" avatar="emoji-rocket" theme="primary" link="https://www.stoicdreams.com">
        About two dozen websites, including this one, run on Web UI from one shared CDN. Every push to main runs each site's Playwright tests, checking config, SEO files, links, and that every page loads without errors, and deploys only if they pass.

        **GitHub Actions, Playwright, Rust, PowerShell**
    </webui-card>
</webui-cards>

## Developer tools and services

<webui-cards card-width="340">
    <webui-card name="Task Proxy" avatar="emoji-hammer_and_wrench" theme="secondary" link="https://www.taskproxy.com">
        A lightweight, cross-platform desktop tool for managing project documentation, scripts, and automations. It keeps pages and settings in the repository so teams share them through git, runs parameterized scripts with generated inputs, and stores secrets in the operating system's credential manager.

        **Rust**
    </webui-card>
    <webui-card name="My Fidelity" avatar="emoji-cloud" theme="secondary" link="https://www.myfi.ws">
        The Backend as a Service from Stoic Dreams, providing the APIs behind its sites and apps.
    </webui-card>
    <webui-card name="Ap Tul" avatar="emoji-laptop" theme="secondary" link="https://www.aptul.com">
        A library of privacy-focused browser tools for developers, such as image conversion and resizing, PDF merging and splitting, and a JWT decoder. Files are processed in the browser rather than uploaded.
    </webui-card>
    <webui-card name="StoicDreams.TestFramework" avatar="emoji-test_tube" theme="secondary">
        A testing framework for C# and .NET that standardizes Arrange, Act, Assert tests, with an extension for testing Blazor components built on bUnit.

        **C#, .NET, Blazor, bUnit**
    </webui-card>
</webui-cards>

## Writing

<webui-page-segment elevation="10">

- **[CASE: Continuous Agile Software Engineering](/blogs/book_published_for_case_continuous_agile_software_engineering)**, my book on the practices I use to ship continuously, and its [twelve tenets](/software-engineering-standards-and-practices).
- [SoftwareStandards.dev](https://www.softwarestandards.dev), a site documenting those standards and practices in more depth.
- More articles on my [publications](/publications) page.

</webui-page-segment>
