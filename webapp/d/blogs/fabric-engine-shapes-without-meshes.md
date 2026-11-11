<webui-data data-page-title="Drawing Shapes Without Meshes" data-page-subtitle="How Fabric Engine renders exact shapes on the GPU"></webui-data>
<webui-data data-page-description="How Fabric Engine draws models without polygon meshes: proxy cubes, exact ray intersections solved per pixel in WGSL, and a screen-space voxel grid for a 3D pixel-art look."></webui-data>
<webui-data data-page-next-page='{"name":"Fabric Engine","href":"/fabric-engine"}'></webui-data>

<webui-page-segment elevation="10">

Most 3D engines draw triangles. A model is a mesh of thousands of them, built in a modeling tool, and the GPU fills in each one. [Fabric Engine](/fabric-engine), the Rust engine I'm building for Wyrdloom Studios, draws no triangle meshes for its models. Every model is a small tree of mathematical shapes, and the GPU solves each shape's exact surface for every pixel.

This article explains how that works, and why it suits the stylized 3D pixel-art look of our first game, Faryn. The engine's code is private, so this is a walk through the ideas rather than the source.

</webui-page-segment>

## Models are trees of shapes

<webui-page-segment elevation="10">

A Fabric model is a tree of nodes. Each node has a position and rotation relative to its parent, animation offsets with limits, and a material: color, roughness, metalness, index of refraction, subsurface scattering, emission, edge rounding, and animated procedural noise.

Each node is one of eight shapes: a box, ellipsoid, cylinder, pyramid, cone, bowl, pipe, or hollow box. A character might be an ellipsoid head on a rounded box body, with cylinders for limbs. Changing a model in the Fabric Creator editor means changing numbers, such as a size, an offset, or a rounding, not reshaping a mesh.

</webui-page-segment>

## Step 1: a proxy cube for each shape

<webui-page-segment elevation="10">

The GPU still needs something to rasterize, so each node is drawn as a simple cube, 36 vertices generated in the vertex shader with no vertex buffer. The whole scene is one instanced draw call, with one cube instance per node.

The cube is a stand-in, not the shape. It is sized from the shape's bounding sphere, so it encloses the shape at any rotation. Its only job is to make the GPU run the fragment shader on every pixel where the real shape could appear.

</webui-page-segment>

## Step 2: solve the exact surface per pixel

<webui-page-segment elevation="10">

For each of those pixels, the fragment shader casts a ray from the camera through the pixel and solves where it hits the real shape:

1. **Move the ray into the shape's own space.** Rotating the ray by the inverse of the node's rotation means every shape can be solved as if it sits at the origin, unrotated.
2. **Solve the intersection exactly.** Each shape has its own closed-form solution. For an ellipsoid, dividing the ray by the shape's three radii turns it into a unit sphere, so the hit is the smaller root of a quadratic equation. A box uses the classic slab method: find where the ray enters and leaves each pair of faces, and take the latest entry. Hollow shapes such as pipes and hollow boxes subtract an inner shape from an outer one along the ray.
3. **Find the surface normal.** The normal comes from the shape's math too. For an ellipsoid, it is the hit point divided by the square of each radius. For a rounded box, it blends toward the corner when the hit is within the edge radius.
4. **Discard the misses.** If the ray misses the shape, the pixel is discarded, and the parts of the proxy cube outside the shape never appear.
5. **Write the real depth.** The shader writes the depth of the true hit point, not of the proxy cube. Where two shapes overlap, the depth test keeps whichever surface is really in front, so shapes meet exactly at any angle.

There is no ray marching here. Ray marching steps along a ray until it gets close enough to a surface, which costs many steps per pixel and gives an approximate answer. A closed-form intersection costs a few operations and gives the exact one.

The pass writes a G-buffer: color, normal, position, depth, and the id of the node under each pixel. The ids also let the editor know which node is under the cursor, so picking needs no extra work.

</webui-page-segment>

## Step 3: snap the scene to voxels

<webui-page-segment elevation="10">

Faryn's look is 3D pixel art, with the scene appearing to be built from small cubes. Rather than modeling voxels, Fabric creates them from the image:

1. The G-buffer is drawn at the resolution of a screen-space grid, so each pixel is one cell of the grid.
2. A compute shader turns every covered cell into a voxel. It places the voxel at the surface point under that cell, gives it the cell's color and the rotation of the node it came from, and sizes it from its distance to the camera, so that every voxel covers one grid cell on screen whether it is near or far.
3. The voxels are drawn as instanced cubes, then lit.

Because the voxels come from the screen rather than the models, the pixel-art style holds at any camera distance, and the models themselves stay exact.

</webui-page-segment>

## Lighting the same shapes

<webui-page-segment elevation="10">

The global illumination pass traces rays against the same eight shapes, with the same exact intersection math, using low-discrepancy sampling for indirect light and ambient occlusion. One set of shape definitions serves drawing, lighting, and picking.

</webui-page-segment>

## Trade-offs

<webui-page-segment elevation="10">

This approach fits a game built from simple, stylized shapes. It would be the wrong choice for detailed, organic models, where meshes are the better tool. The engine keeps a polygon renderer for that reason, although it is not used today.

What it gives Fabric:

- **No meshes to build or store.** A model is a short list of numbers, which keeps models small and easy to edit, version, and animate.
- **Exact surfaces at any distance.** There is no level of detail to manage, because there are no triangles to simplify.
- **One source of truth.** Rendering, lighting, and editor picking all use the same shape definitions.

</webui-page-segment>

## Keeping shaders safe

<webui-page-segment elevation="10">

With most of the engine's work in WGSL shaders, a shader error is as serious as a Rust error. Every shader is parsed and validated with naga as part of `cargo test`, so a broken shader fails the build instead of crashing the game.

You can see an early look at the engine in the [first development video](https://www.youtube.com/watch?v=Gn99fCoEPdM), and follow along on [Wyrdloom Studios' YouTube channel](https://www.youtube.com/@wyrdloom).

</webui-page-segment>
