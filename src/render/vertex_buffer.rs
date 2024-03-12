use bevy::{
    math::{vec2, Vec2, Vec3},
    render::{
        color::Color,
        mesh::{Indices, Mesh},
        render_asset::RenderAssetUsages,
        render_resource::PrimitiveTopology,
    },
    transform::components::Transform,
};
use copyless::VecHelper;
use lyon_tessellation::{
    self, FillVertex, FillVertexConstructor, StrokeVertex, StrokeVertexConstructor,
};

use crate::Convert;

/// A vertex with all the necessary attributes to be inserted into a Bevy
/// [`Mesh`](bevy::render::mesh::Mesh).
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Vertex {
    position: [f32; 3],
    color: [f32; 4],
}

/// The index type of a Bevy [`Mesh`](bevy::render::mesh::Mesh).
pub(crate) type IndexType = u32;

/// Lyon's [`VertexBuffers`] generic data type defined for [`Vertex`].
pub(crate) type VertexBuffers = lyon_tessellation::VertexBuffers<Vertex, IndexType>;

impl Convert<Mesh> for (VertexBuffers, Vec2, Vec2) {
    fn convert(self) -> Mesh {
        let (buffer, size, origin) = self;
        let mut positions = Vec::with_capacity(buffer.vertices.len());
        let mut colors = Vec::with_capacity(buffer.vertices.len());
        let offset = vec2(-size.x * origin.x, size.y * origin.y);

        for vert in buffer.vertices.into_iter() {
            let position = vert.position;
            positions
                .alloc()
                .init([position[0] + offset.x, position[1] + offset.y, position[2]]);
            colors.alloc().init(vert.color);
        }

        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        mesh.insert_indices(Indices::U32(buffer.indices));

        mesh
    }
}

/// Zero-sized type used to implement various vertex construction traits from Lyon.
pub(crate) struct VertexConstructor {
    pub(crate) color: Color,
    pub(crate) transform: Transform,
}

/// Enables the construction of a [`Vertex`] when using a `FillTessellator`.
impl FillVertexConstructor<Vertex> for VertexConstructor {
    fn new_vertex(&mut self, vertex: FillVertex) -> Vertex {
        let vertex = vertex.position();
        let pos = self.transform * Vec3::new(vertex.x, vertex.y, 0.0);

        Vertex {
            position: [pos.x, pos.y, pos.z],
            color: self.color.as_linear_rgba_f32(),
        }
    }
}

/// Enables the construction of a [`Vertex`] when using a `StrokeTessellator`.
impl StrokeVertexConstructor<Vertex> for VertexConstructor {
    fn new_vertex(&mut self, vertex: StrokeVertex) -> Vertex {
        let vertex = vertex.position();
        let pos = self.transform * Vec3::new(vertex.x, vertex.y, 0.0);

        Vertex {
            position: [pos.x, pos.y, pos.z],
            color: self.color.as_linear_rgba_f32(),
        }
    }
}

pub(crate) trait BufferExt<A> {
    fn extend_one(&mut self, item: A);
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T);
}

impl BufferExt<VertexBuffers> for VertexBuffers {
    fn extend_one(&mut self, item: VertexBuffers) {
        let offset = self.vertices.len() as u32;

        for vert in item.vertices.into_iter() {
            self.vertices.alloc().init(vert);
        }
        for idx in item.indices.into_iter() {
            self.indices.alloc().init(idx + offset);
        }
    }

    fn extend<T: IntoIterator<Item = VertexBuffers>>(&mut self, iter: T) {
        let mut offset = self.vertices.len() as u32;

        for buf in iter.into_iter() {
            let num_verts = buf.vertices.len() as u32;
            for vert in buf.vertices.into_iter() {
                self.vertices.alloc().init(vert);
            }
            for idx in buf.indices.into_iter() {
                self.indices.alloc().init(idx + offset);
            }
            offset += num_verts;
        }
    }
}
