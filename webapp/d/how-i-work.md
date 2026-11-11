<webui-data data-page-title="How I Work" data-page-subtitle="Small, tested changes, shipped continuously"></webui-data>
<webui-data data-page-description="How I build software: small changes shipped to production continuously, automated tests on every deploy, measured performance, and teams that learn as they go."></webui-data>
<webui-data data-page-next-page='{"name":"Services","href":"/services"}'></webui-data>

<webui-page-segment elevation="10">

I build software in small, tested steps and ship each one to production as soon as it is ready. I've refined this way of working over my career and wrote it up as a book, [CASE: Continuous Agile Software Engineering](/blogs/book_published_for_case_continuous_agile_software_engineering). This page shows what it looks like in practice, and the results it has produced.

</webui-page-segment>

## Ship small, ship often

<webui-page-segment elevation="10">

Every change is small enough to review, test, and release on its own, so problems are found while they are still small, and users get improvements as soon as they exist.

- **At General Services Corp**, I pushed to production several times a day. The reporting portal I built went live with its first reports in its first week of development, then grew to hundreds of reports.
- **On Fabric Engine**, I ship engine features and optimizations in hours or days, not in large batches.
- **On my own sites**, every push to main deploys, so a fix goes live as soon as its tests pass.

</webui-page-segment>

## Let tests decide what ships

<webui-page-segment elevation="10">

Shipping often only works if each release is safe, so the checks are automated and run every time.

- **Every deploy of my sites runs its Playwright tests first**, and a failing test stops the deploy.
- **The Web UI framework can't add a component or method without tests**, because a coverage test fails until they exist. See the [Web UI case study](/case-studies/web-ui).
- **Fabric Engine validates every shader in `cargo test`**, so a shader error fails the build instead of the running game.

</webui-page-segment>

## Measure performance, and treat slowness as a bug

<webui-page-segment elevation="10">

Speed is a feature users notice first, so I set a standard and hold to it.

- **At General Services Corp**, every browser-to-server request had to finish in under 100 ms, and anything over 300 ms was treated as an urgent bug, across hundreds of reports.
- **At Microsoft**, I prototyped and partly built a re-architecture of a customer-facing reporting service, bringing queries that took from tens of seconds up to hundreds of minutes down to between milliseconds and a few seconds.

</webui-page-segment>

## Keep it simple

<webui-page-segment elevation="10">

The simplest thing that delivers value is usually the easiest to change later. In practice that means removing boilerplate instead of copying it: a layout used in more than one place becomes one shared component, and unused code is deleted rather than kept "just in case".

</webui-page-segment>

## Grow the team, not just the code

<webui-page-segment elevation="10">

- **At Microsoft**, I ran training sessions on unit testing and test-driven development, and mentored a new graduate engineer who went on to become the team's UI lead.
- **At SoftRock**, I moved the team toward object-oriented code and consistent best practices, and within six months of first learning ColdFusion I was teaching new parts of the language to experienced ColdFusion developers.

</webui-page-segment>

<webui-quote theme="tertiary" cite="Joel Ferreira, LinkedIn recommendation" class="my-5">
    Erik is an out of the box thinker and a great collaborator.
</webui-quote>

## Learn more

<webui-page-segment elevation="10">

<webui-flex gap="2" wrap>
    <webui-link href="/software-engineering-standards-and-practices" start-icon="emoji-scroll">The twelve tenets of CASE</webui-link>
    <webui-link href="/blogs/book_published_for_case_continuous_agile_software_engineering" start-icon="emoji-ledger">The CASE book</webui-link>
    <webui-link href="https://www.softwarestandards.dev" target="_blank" start-icon="emoji-globe_with_meridians">SoftwareStandards.dev</webui-link>
</webui-flex>

</webui-page-segment>
