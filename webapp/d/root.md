<webui-data data-page-title="Erik Gassler" data-page-subtitle="Principal Software Engineer"></webui-data>
<webui-data data-page-next-page='{"name":"Experience","href":"/experience"}'></webui-data>

<webui-sideimage src="https://cdn.myfi.ws/img/eg/me_512.webp" alt="Photo of Erik Gassler">
    <webui-page-segment elevation="10">

## I build fast, reliable software, from Rust engines to production web platforms

I'm a principal-level software engineer with 19 years of shipping production software, at Microsoft, at startups, and on my own products.

By day I'm the UI Lead for several of Opyn Health's healthcare web applications. I'm also the founder of Wyrdloom Studios, where I'm building **Fabric Engine**, a 3D engine and world-building suite written entirely in Rust.

I'm open to **Principal and Staff engineering roles**, and to **contract work**.

<webui-flex gap="2" class="mt-3" wrap>
    <webui-link href="https://www.linkedin.com/in/erikgassler" target="_blank" start-icon="emoji-link">LinkedIn</webui-link>
    <webui-link href="https://github.com/StoicDreams" target="_blank" start-icon="emoji-laptop">GitHub</webui-link>
    <webui-link href="/contact" start-icon="emoji-envelope">Contact me</webui-link>
</webui-flex>

    </webui-page-segment>
</webui-sideimage>

## What I'm building now: Fabric Engine

<webui-page-segment elevation="10">

Fabric Engine is a custom, high-performance 3D engine and world-building suite written in Rust on top of wgpu and winit. Instead of polygon meshes, its models are trees of mathematical shapes that the GPU draws exactly, pixel by pixel, before snapping the scene to a voxel grid for a stylized 3D pixel-art look.

It powers **Faryn**, the first game from my studio, [Wyrdloom Studios](https://www.wyrdloom.com). The game is the proving ground for the engine, so every feature is built against a real product.

<webui-link href="/fabric-engine" start-icon="emoji-video_game">More about Fabric Engine</webui-link>

</webui-page-segment>

<webui-content cache src="/d/youtube/fabric-engine-intro.md"></webui-content>

## Results I've delivered

<webui-cards card-width="340">
    <webui-card name="Report queries cut from minutes to milliseconds" avatar="emoji-stopwatch" theme="primary">
        **Microsoft.** Prototyped and partly built a re-architecture of a customer-facing reporting service. Queries that took from tens of seconds up to hundreds of minutes came down to between milliseconds and a few seconds.
    </webui-card>
    <webui-card name="A reporting portal live in its first week" avatar="emoji-bar_chart" theme="primary">
        **General Services Corp.** Architected and built Joe Morning, a reporting and data management portal for employees and business partners. It launched with role-based security and its first reports in week one, then grew to hundreds of reports, error tracking, and content management for the company's websites. I wrote over 95% of its code in 18 months, held to a standard of under 100 ms for every request.
    </webui-card>
    <webui-card name="Agents handling two calls at once" avatar="emoji-telephone_receiver" theme="primary">
        **SoftRock.** Architected a call-scripting platform that spoke to callers with pre-recorded segments. Agents could handle two calls at once while outperforming top agents who spoke to callers directly. I also built the role-based security framework later used across most of the company's websites.
    </webui-card>
    <webui-card name="A greenfield product as the sole developer" avatar="emoji-rocket" theme="primary">
        **Microsoft.** Learned Azure, Azure DevOps, and React with TypeScript on the job, then built and published a new website and API services as the sole developer for about four months before handing it to a team in India.
    </webui-card>
    <webui-card name="UI lead for healthcare platforms" avatar="emoji-hospital" theme="primary">
        **Opyn Health.** Lead the UI work on several of Opyn's web applications in C# Blazor, including the [Index provider search](https://index.opynhealth.com), the Iris client and partner portal, and the admin site.
    </webui-card>
    <webui-card name="A framework that runs two dozen sites" avatar="emoji-building_construction" theme="primary">
        **Stoic Dreams.** Created [Web UI](https://webui.stoicdreams.com), a vanilla JavaScript component framework with Rust WebAssembly workers. It runs about two dozen websites, including this one, from one shared CDN, with automated tests that gate every deploy.
    </webui-card>
</webui-cards>

<webui-flex justify="center" class="my-3">
    <webui-link href="/experience" start-icon="emoji-briefcase">See my full experience</webui-link>
</webui-flex>

## How I work

<webui-sideimage reverse src="https://cdn.myfi.ws/img/case/CASE_Hardcover_Circle.webp" alt="CASE book hardcover">
    <webui-page-segment elevation="10">

I ship in small, tested steps, straight to production, often several times a day. Every change is covered by automated tests, every deploy is automated, and problems are measured and fixed rather than debated. I also train and mentor teams in unit testing and test-driven development.

I wrote these practices up in my book, [CASE: Continuous Agile Software Engineering](/blogs/book_published_for_case_continuous_agile_software_engineering). Read a summary of its [twelve tenets](/software-engineering-standards-and-practices), or see my [other writing](/publications).

    </webui-page-segment>
</webui-sideimage>

## Skills

<webui-cards card-width="260">
    <webui-card name="Rust and systems" avatar="emoji-crab" theme="secondary">
        Rust, wgpu, winit, WebAssembly, Tauri, performance tuning
    </webui-card>
    <webui-card name="Web and UI" avatar="emoji-globe_with_meridians" theme="secondary">
        C# Blazor, JavaScript, TypeScript, React, HTML, CSS, Web UI
    </webui-card>
    <webui-card name="Cloud and delivery" avatar="emoji-cloud" theme="secondary">
        Azure, AWS, .NET, CI/CD with GitHub Actions and Azure DevOps, Playwright
    </webui-card>
    <webui-card name="Data" avatar="emoji-card_file_box" theme="secondary">
        SQL Server, PostgreSQL, MySQL, Cosmos DB
    </webui-card>
</webui-cards>

<webui-quote theme="tertiary" cite="Joel Ferreira, LinkedIn recommendation" class="my-5">
    Erik is an out of the box thinker and a great collaborator.
</webui-quote>

## Let's work together

<webui-page-segment elevation="10">

If you're hiring for a Principal or Staff engineer, or need help with Rust, engine or tooling work, performance problems, or getting a team shipping continuously, I'd like to hear from you.

<webui-flex gap="2" class="mt-3" wrap>
    <webui-link href="https://www.linkedin.com/in/erikgassler" target="_blank" start-icon="emoji-link">Message me on LinkedIn</webui-link>
    <webui-link href="/contact" start-icon="emoji-envelope">Contact me</webui-link>
    <webui-link href="/experience" start-icon="emoji-briefcase">See my experience</webui-link>
    <webui-link href="/about" start-icon="emoji-memo">Read my story</webui-link>
</webui-flex>

</webui-page-segment>
