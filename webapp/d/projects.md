<webui-data data-page-title="Projects" data-page-subtitle="Tools, frameworks, and platforms I've built"></webui-data>
<webui-data data-page-description="Screenshots and clips of the software I build: Fabric Engine, Task Proxy, Task Story, Blue Cherub, Vocal Bin, Ap Tul, and the Web UI framework behind about two dozen websites."></webui-data>
<webui-data data-page-next-page='{"name":"How I Work","href":"/how-i-work"}'></webui-data>

<webui-page-segment elevation="10">

Alongside my day job, I build and run my own software through my companies, Stoic Dreams and Wyrdloom Studios. These projects are where I set my own standards: small, tested changes that deploy automatically, and tools that remove friction for the people using them.

The screenshots and clips below load straight from each live site, which captures them from its real interface when it releases, so they show each site as it is today.

</webui-page-segment>

## Fabric Engine

<webui-feature>
<webui-youtube slot="media" video="Gn99fCoEPdM" label="Fabric Engine introduction video"></webui-youtube>

A 3D game engine and world-building suite in Rust and WGSL. Models are trees of mathematical shapes drawn exactly on the GPU, then snapped to a voxel grid for a stylized 3D pixel-art look. It powers Faryn, Wyrdloom Studios' first game.

**Rust, WGSL, wgpu, winit**

<webui-flex gap="2" wrap>
    <webui-link href="/fabric-engine" icon="cube">How it works</webui-link>
    <webui-link href="/blogs/fabric-engine-shapes-without-meshes" icon="newspaper">Drawing shapes without meshes</webui-link>
</webui-flex>
</webui-feature>

## Task Proxy

<webui-feature src="https://www.taskproxy.com/video/loops/git-commit-ready.webp" video="https://www.taskproxy.com/video/loops/git-commit-ready.mp4" alt="The Task Proxy Git Controller, showing changed files, a commit message, and a side-by-side diff" reverse>

A lightweight, cross-platform desktop tool for project documentation, scripts, and automations, for Windows, Mac, and Linux. It keeps pages and settings in the repository so teams share them through git, runs parameterized scripts with generated inputs, and stores secrets in the operating system's credential manager. Its Git Controller reviews, commits, and syncs changes from the same window.

**Rust**

<webui-flex gap="2" wrap>
    <webui-link href="https://www.taskproxy.com" target="_blank" icon="globe">taskproxy.com</webui-link>
    <webui-link href="https://github.com/StoicDreams/TaskProxyApp" target="_blank" icon="github">Source on GitHub</webui-link>
</webui-flex>
</webui-feature>

<webui-cards card-width="300">
    <webui-card name="Task Manager" avatar="list">
        <webui-loop-video src="https://www.taskproxy.com/video/loops/task-manager.mp4" poster="https://www.taskproxy.com/video/loops/task-manager.webp" alt="The Task Manager's Orphaned preset finding a Storybook server left running"></webui-loop-video>

        Finds dev servers and tools left running, and stops them.
    </webui-card>
    <webui-card name="Team pages" avatar="users">
        <webui-loop-video src="https://www.taskproxy.com/video/loops/page-onboarding.mp4" poster="https://www.taskproxy.com/video/loops/page-onboarding.webp" alt="An onboarding page in Task Proxy, with steps and a run button on each script"></webui-loop-video>

        Onboarding guides and runbooks with a run button on every step.
    </webui-card>
    <webui-card name="Jira plugin" avatar="puzzle-piece">
        <webui-loop-video src="https://www.taskproxy.com/video/loops/jira/views.mp4" poster="https://www.taskproxy.com/video/loops/jira/views.webp" alt="The Jira plugin's Views page, listing the built-in View and a project View"></webui-loop-video>

        Plugins add features such as Jira issue tracking.
    </webui-card>
</webui-cards>

## Blue Cherub

<webui-feature src="https://www.bluecherub.com/img/screenshots/studio/animate-timeline.png" video="https://www.bluecherub.com/img/screenshots/studio/animate-timeline.mp4" alt="The Blue Cherub studio's Animate tab, with a timeline of keys for each part below the drawing">

A free animation studio in the browser. Rig any SVG drawing by giving its parts pivots and parents, then pose it and animate it on a keyframe timeline. Scenes are evaluated in Rust compiled to WebAssembly, and exported as MP4 video, encoded in the browser with WebCodecs, or as PNG frames.

**JavaScript, Rust, WebAssembly, SVG, WebCodecs**

<webui-link href="https://www.bluecherub.com" target="_blank" icon="globe">bluecherub.com</webui-link>
</webui-feature>

## Vocal Bin

<webui-feature src="https://www.vocalbin.com/img/screenshots/pages/text-to-speech.png" href="https://www.vocalbin.com" alt="Vocal Bin's Text to Speech tool, with sliders for pitch, speed, intonation, and voice quality" reverse>

A speech synthesizer I'm writing from scratch, with no recordings and no AI. Type English and hear it read aloud, shape the voice with sliders, and save WAV files. It is a classic formant synthesizer, a voice source feeding filters that shape each sound, written in Rust and run in the browser as WebAssembly in an AudioWorklet.

**Rust, WebAssembly, Web Audio**

<webui-link href="https://www.vocalbin.com" target="_blank" icon="globe">vocalbin.com</webui-link>
</webui-feature>

## Task Story

<webui-feature src="https://www.taskstory.com/img/screenshots/dashboard.png" href="https://www.taskstory.com" alt="The Task Story dashboard, with a team's tasks, solutions, and problems side by side">

Project management for software teams that ship in short, continuous iterations. Teams define the problem first, try more than one solution, and track the tasks behind each one, the workflow from my book, [CASE](/blogs/book_published_for_case_continuous_agile_software_engineering). I'm rebuilding it on Web UI, replacing the earlier Blazor app.

**JavaScript, Web UI**

<webui-link href="https://www.taskstory.com" target="_blank" icon="globe">taskstory.com</webui-link>
</webui-feature>

## Ap Tul

<webui-feature src="https://www.aptul.com/img/screenshots/pages/image-converter.png" href="https://www.aptul.com" alt="Ap Tul's Image Converter, with resize, format, and quality options" reverse>

A library of over 30 privacy-focused browser tools: image conversion and resizing, PDF merging and splitting, a JWT decoder, a regex tester, and more. Files are processed on the device, much of it in Rust compiled to WebAssembly, rather than uploaded to a server.

**JavaScript, Rust, WebAssembly**

<webui-link href="https://www.aptul.com" target="_blank" icon="globe">aptul.com</webui-link>
</webui-feature>

## The platform behind them

<webui-cards card-width="340">
    <webui-card name="Web UI" avatar="puzzle-piece" theme="primary" link="https://webui.stoicdreams.com">
        A vanilla JavaScript web component framework, with about 70 components, a data binding system, and Rust WebAssembly workers for heavy processing. Pages are written in markdown, and components load themselves on first use, so a new site starts with almost no boilerplate.

        **JavaScript, HTML, CSS, Rust, WebAssembly**

        [Read the case study](/case-studies/web-ui)
    </webui-card>
    <webui-card name="A platform for two dozen sites" avatar="layers" theme="primary" link="https://www.stoicdreams.com">
        About two dozen websites, including this one, run on Web UI from one shared CDN. Every push to main runs each site's Playwright tests, checking config, SEO files, links, and that every page loads without errors, and deploys only if they pass. Each release also captures a fresh screenshot of every page, like the ones on this page.

        **GitHub Actions, Playwright, Rust, PowerShell**
    </webui-card>
    <webui-card name="My Fidelity" avatar="cloud" theme="secondary" link="https://www.myfi.ws">
        The Backend as a Service from Stoic Dreams, providing the APIs behind its sites and apps.
    </webui-card>
    <webui-card name="StoicDreams.TestFramework" avatar="test" theme="secondary">
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
