<webui-data data-page-title="Experience" data-page-subtitle="19 years of shipping production software"></webui-data>
<webui-data data-page-next-page='{"name":"About Erik","href":"/about"}'></webui-data>

<webui-page-segment elevation="10">

I taught myself to program in 2007 and have shipped production software ever since: web platforms, cloud services, internal tools, browser extensions, and now a Rust game engine. Most of my career has been in roles where I owned a product end to end, from architecture to deployment.

For the full record, see my [LinkedIn profile](https://www.linkedin.com/in/erikgassler).

</webui-page-segment>

## At a glance

<webui-page-segment elevation="10">

| Role | Company | Dates |
|---|---|---|
| Senior Software Engineer, UI Lead | Opyn Health | 2022 to present |
| Founder and Lead Architect | Wyrdloom Studios | 2026 to present |
| Principal Software Engineer | Stoic Dreams | 2008 to present |
| Senior Software Engineer | Costco Wholesale (contract) | 2022 |
| Senior Software Engineer | Floating Point Group | 2021 to 2022 |
| Software Engineer 2 | Microsoft | 2019 to 2021 |
| Senior Software Engineer | General Services Corp | 2016 to 2018 |
| Software Engineer II | SoftRock | 2011 to 2016 |
| Website Manager | Opinions R Fun | 2007 to 2008 |

</webui-page-segment>

## Current roles

<webui-page-segment elevation="10">

### Senior Software Engineer, UI Lead · Opyn Health

**Nov 2022 to present · Remote**

I lead the UI work on several of Opyn Health's healthcare web applications, working with the team on its platform applications and services.

- **[Index](https://index.opynhealth.com):** Opyn's public "Find Care" search, where people find healthcare providers by location, specialty, or name, with a choice of language.
- **Iris:** the client, partner, and administrator portal for Opyn's health service search and matching.
- **Admin site and internal dashboards** for running the platform.

**Technologies:** C# Blazor, HTML, CSS

</webui-page-segment>

<webui-page-segment elevation="10">

### Founder and Lead Architect · Wyrdloom Studios

**Mar 2026 to present · Remote**

My own game studio, where I develop games, development tools, and game engines in Rust.

- Building **[Fabric Engine](/fabric-engine)**, a custom 3D engine and world-building suite written entirely in Rust on wgpu and winit. Its models are trees of mathematical shapes drawn exactly on the GPU instead of polygon meshes, rendered through a deferred pipeline with global illumination and screen-space voxelization. Watch the [introduction video](https://www.youtube.com/watch?v=Gn99fCoEPdM).
- Building **Faryn**, the studio's first game and the proving ground for the engine.
- Ship engine features and optimizations in small steps, with production updates daily.

**Technologies:** Rust, WGSL, wgpu, winit

</webui-page-segment>

<webui-page-segment elevation="10">

### Principal Software Engineer · Stoic Dreams

**Oct 2008 to present · Remote**

My software company, for open source developer tools, development practices, and backend services.

- Created [Web UI](https://webui.stoicdreams.com), a vanilla JavaScript web component framework with Rust WebAssembly workers. It runs about two dozen websites, including this one, from one shared CDN.
- Every one of those sites deploys on each push to main, behind automated Playwright tests that stop a deploy when they fail.
- Built Task Proxy, a developer tool for project documentation and scripts, and an Arrange/Act/Assert test framework for C# and Blazor.
- Wrote and published [CASE: Continuous Agile Software Engineering](/blogs/book_published_for_case_continuous_agile_software_engineering), on the practices I use to ship continuously.
- Released my first game, Cosmos, on Xbox 360 in 2008.

**Technologies:** Rust, JavaScript, HTML, CSS, WebAssembly, C#, .NET, GitHub Actions, Playwright

</webui-page-segment>

## Previous roles

<webui-page-segment elevation="10">

### Senior Software Engineer · Costco Wholesale

**May 2022 to Aug 2022 · Contract · Issaquah, WA**

Brought in on a contract-to-hire basis to take over as lead of a platform infrastructure and integrations UI team. The team's tool simplified how Costco's internal developer teams request and get access to Azure resources.

- Focused on knowledge transfer with the team.
- Helped the team add missing tests and debug problems to stabilize the product for its release.

</webui-page-segment>

<webui-page-segment elevation="10">

### Senior Software Engineer · Floating Point Group

**Sep 2021 to Feb 2022 · Duvall, WA**

Started on a contract building a proof of concept with a small team, then joined full-time.

- Built features and services for a product for trading across many cryptocurrency markets, with automated trade settlement and secure asset management.

**Technologies:** Python, AWS

</webui-page-segment>

<webui-page-segment elevation="10">

### Software Engineer 2 · Microsoft

**Feb 2019 to Dec 2021 · Redmond, WA**

Worked on support analytics products in Core Services Engineering and Operations, and later the Azure + AI organization. I joined as a contractor through Motiv Inc and moved to full-time later that year.

- **Faster reporting:** prototyped and partly built a re-architecture of a customer-facing reporting service. Queries that took from tens of seconds up to hundreds of minutes came down to between milliseconds and a few seconds.
- **Greenfield product:** learned Azure, Azure DevOps, and React with TypeScript, then built and published a new website and API services as the sole developer for about four months before handing it to a team in India.
- **Real-time call analytics:** built much of the UI for an application showing Microsoft support calls as they happen, including customer sentiment, call time, and when the agent or caller is speaking or the line is silent.
- **Raising the team:** ran training sessions on unit testing and test-driven development, and mentored a new graduate engineer who went on to lead the team's UI work.
- Built full stack features and API connections, and managed CI/CD pipelines and Azure service configuration.

**Technologies:** C#, .NET Core, TypeScript, React, Azure (DevOps, Functions, Service Fabric, Storage, Cosmos DB, App Services, Bot Services, SQL Database, CDN), PowerShell

</webui-page-segment>

<webui-page-segment elevation="10">

### Senior Software Engineer · General Services Corp

**Mar 2016 to Jun 2018 · Orlando, FL**

Worked directly with the owner, turning business problems into architected and shipped solutions, and pushing to production several times a day.

- **Joe Morning:** architected and built a reporting and data management portal used by employees and business partners. It launched with role-based security and its first reports in its first week of development.
- Grew Joe Morning to hundreds of reports, each with filtering, column toggles, and sorting, plus error tracking for every company website and service, and content management for several company websites.
- Wrote over 95% of Joe Morning's code within 18 months.
- Worked to a standard of under 100 ms for every browser to server request, and treated anything over 300 ms as an urgent bug.
- Built a browser extension for Chrome, Firefox, and Edge that remembered and filled in job application details.

**Technologies:** ColdFusion, JavaScript, HTML, CSS, SQL Server

</webui-page-segment>

<webui-page-segment elevation="10">

### Software Engineer II · SoftRock

**Jan 2011 to Mar 2016 · Orlando, FL**

- Architected a call-scripting platform that spoke to callers with pre-recorded segments. Agents could handle two calls at once while outperforming top agents who spoke to callers directly.
- Built a role-based security system, including SQL Server stored procedures, used on several of the company's major sites and later carried over to General Services Corp.
- A key programmer on Gigats.com, building many of its shared libraries: security, server-side form validation, photo cropping, and a general JavaScript library.
- Moved the team toward object-oriented code and consistent best practices.
- Started with no ColdFusion experience, and within six months was teaching experienced ColdFusion developers parts of the language they hadn't used.

**Technologies:** ColdFusion, JavaScript, HTML, CSS, SQL Server

</webui-page-segment>

## Early career

<webui-page-segment elevation="10">

### Website Manager · Opinions R Fun

**Dec 2007 to Apr 2008 · Wichita, KS**

The first in-house software engineer for a small online merchandise business, hired to fix its website and grow its traffic.

- Fixed the site's bugs, and built a product importer that added 30,000 products.
- Grew traffic from a few visitors a week to over 100 a day through SEO.
- Moved the site to a dedicated server, saving about $500 a month in hosting.

**Technologies:** PHP, MySQL, Apache, HTML, CSS

</webui-page-segment>

<webui-page-segment elevation="10">

### Self-taught developer

**May 2007 to Dec 2007**

Left a full-time job to teach myself web development with Apache, PHP, MySQL, HTML, and CSS, and built a social networking site that landed my first engineering job.

</webui-page-segment>

## Certifications

<webui-page-segment elevation="10">

- Microsoft Global Hackathon, 2021
- DevOps Foundations, LinkedIn Learning, 2019

</webui-page-segment>
