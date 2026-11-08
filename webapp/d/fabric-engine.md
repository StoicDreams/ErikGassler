<webui-data data-page-title="Fabric Engine" data-page-subtitle="A 3D engine and world-building suite in Rust"></webui-data>
<webui-data data-page-next-page='{"name":"Projects","href":"/projects"}'></webui-data>

<webui-page-segment elevation="10">

Fabric Engine is the custom 3D game engine and world-building suite I'm building from the ground up in Rust and WGSL at my studio, [Wyrdloom Studios](https://www.wyrdloom.com). I started it in February 2026, and it is in early development.

It powers **Faryn**, the studio's first game: an action-adventure inspired by the 8-bit and 16-bit classics, with modern graphics and controls. The game is the engine's proving ground, so every engine feature is built for, and tested against, a real product.

</webui-page-segment>

## See it in action

<webui-page-segment elevation="10">

The first development video gives a very early look at the engine.

</webui-page-segment>

<webui-youtube video="Gn99fCoEPdM" label="Fabric Engine introduction video" width="960" class="my-3"></webui-youtube>

## What makes it different

<webui-cards card-width="340">
    <webui-card name="Models built from math, not meshes" avatar="emoji-crab" theme="primary">
        Each model is a tree of nodes, and each node is a shape: a box, ellipsoid, cylinder, cone, pyramid, bowl, pipe, or hollow box. The GPU draws every node by solving the exact ray intersection with its shape for each pixel. There are no polygon meshes and no ray marching.
    </webui-card>
    <webui-card name="A stylized 3D pixel-art look" avatar="emoji-video_game" theme="primary">
        After the scene is drawn, the engine snaps it to a screen-space voxel grid and redraws it as instanced cubes. The result is a tilt-shift, 3D pixel-art style built for Faryn, rather than a general-purpose look.
    </webui-card>
    <webui-card name="An editor as well as an engine" avatar="emoji-hammer_and_wrench" theme="primary">
        Fabric Creator is the desktop editor built on the engine, for building models, node hierarchies, scenes, tile maps, worlds, text, and characters, with on-screen handles and picking of whatever is under the cursor.
    </webui-card>
</webui-cards>

## How a frame is rendered

<webui-page-segment elevation="10">

Fabric uses deferred rendering, as a chain of GPU passes:

1. **G-buffer:** draws a simple proxy cube for each node, then solves the exact intersection with the node's real shape for each pixel, writing color, normal, position, depth, and the model and node ids.
2. **Global illumination:** traces rays against the same analytic shapes, using low-discrepancy sampling, for indirect lighting and ambient occlusion.
3. **Screen-space voxelization:** maps each pixel to a cell in a screen-space grid. Each cell takes the node nearest the camera and the average color of its pixels.
4. **Instanced rendering:** redraws the grid as instanced cubes.
5. **Lighting, background, and post-processing:** lights the voxels, fills the background, runs any post-processing passes the game adds, and composites the final frame.

Large tile-map worlds take their own path: a compute pass culls tiles and builds the instance list on the GPU, then the engine draws it with indirect draw calls and a dedicated global illumination pass.

</webui-page-segment>

## Engineering highlights

<webui-page-segment elevation="10">

- **Materials per node:** color, roughness, metalness, index of refraction, subsurface scattering, emission, and animated procedural noise, with parent-relative transforms and animation limits on every node.
- **Rendering on its own thread:** the render loop runs separately from the window's event loop, paced to 60 frames per second or unlocked.
- **Modular design:** seven Rust crates, for core types, events, GPU pipelines, shaders, backgrounds, the engine runtime, and the Creator editor.
- **Shaders tested like code:** every WGSL shader is parsed and validated with naga in `cargo test`, so a shader error fails the build instead of the running game.
- **Built on** wgpu for GPU access and winit for windows and input.
- Developed in small, tested steps, using the continuous delivery practices from my book, [CASE](/blogs/book_published_for_case_continuous_agile_software_engineering).

</webui-page-segment>

## Why it matters for my work

<webui-page-segment elevation="10">

Fabric Engine is where I do my most demanding engineering: designing a system architecture from scratch, working directly with the GPU, and keeping performance high as features grow. It is the same approach I've brought to the companies I've worked for, from re-architecting a slow reporting service at Microsoft to building a reporting portal held to under 100 ms per request. See my [experience](/experience) for the full record.

</webui-page-segment>

## Follow the project

<webui-page-segment elevation="10">

<webui-flex gap="2" wrap>
    <webui-link href="https://www.wyrdloom.com" target="_blank" start-icon="emoji-globe_with_meridians">Wyrdloom Studios</webui-link>
    <webui-link href="https://www.youtube.com/@wyrdloom" target="_blank" start-icon="emoji-video_game">Wyrdloom on YouTube</webui-link>
    <webui-link href="https://www.linkedin.com/in/erikgassler" target="_blank" start-icon="emoji-link">Follow me on LinkedIn</webui-link>
</webui-flex>

</webui-page-segment>
