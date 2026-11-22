// Modified for the VMNL fallback package, which omits the upstream derive macros.

use super::VertexInputRate;
use crate::{buffer::BufferContents, format::Format};
use std::collections::HashMap;

/// Describes an individual `Vertex`. In other words a collection of attributes that can be read
/// from a vertex shader.
///
/// At this stage, the vertex is in a "raw" format. For example a `[f32; 4]` can match both a
/// `vec4` or a `float[4]`. The way the things are bound depends on the shader.
///
/// The vertex trait can be derived and the format has to be specified using the `format`
/// field-level attribute:
///
/// ```
/// use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex};
///
/// #[derive(BufferContents, Vertex)]
/// #[repr(C)]
/// struct MyVertex {
///     // Every field needs to explicitly state the desired shader input format
///     #[format(R32G32B32_SFLOAT)]
///     pos: [f32; 3],
///     // The `name` attribute can be used to specify shader input names to match.
///     // By default the field-name is used.
///     #[name("in_proj", "cam_proj")]
///     #[format(R32G32B32_SFLOAT)]
///     proj: [f32; 16],
/// }
/// ```
pub unsafe trait Vertex: BufferContents + Sized {
    /// Returns the information about this Vertex type.
    fn per_vertex() -> VertexBufferDescription;

    fn per_instance() -> VertexBufferDescription;

    fn per_instance_with_divisor(divisor: u32) -> VertexBufferDescription;
}

/// Describes the contents of a VertexBuffer.
#[derive(Clone, Debug)]
pub struct VertexBufferDescription {
    /// List of member names with their detailed information.
    pub members: HashMap<String, VertexMemberInfo>,
    /// Stride of the vertex type in a buffer.
    pub stride: u32,
    /// How the vertex buffer is unrolled in the shader.
    pub input_rate: VertexInputRate,
}

impl VertexBufferDescription {
    #[inline]
    pub fn per_vertex(self) -> VertexBufferDescription {
        let VertexBufferDescription {
            members, stride, ..
        } = self;
        VertexBufferDescription {
            members,
            stride,
            input_rate: VertexInputRate::Vertex,
        }
    }

    #[inline]
    pub fn per_instance(self) -> VertexBufferDescription {
        self.per_instance_with_divisor(1)
    }

    #[inline]
    pub fn per_instance_with_divisor(self, divisor: u32) -> VertexBufferDescription {
        let VertexBufferDescription {
            members, stride, ..
        } = self;
        VertexBufferDescription {
            members,
            stride,
            input_rate: VertexInputRate::Instance { divisor },
        }
    }
}

/// Information about a member of a vertex struct.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VertexMemberInfo {
    /// The offset of the member in bytes from the start of the struct.
    pub offset: u32,

    /// The attribute format of the member. Implicitly provides the number of components.
    pub format: Format,

    /// The number of consecutive array elements or matrix columns using `format`.
    /// The corresponding number of locations might differ depending on the size of the format.
    pub num_elements: u32,

    /// If `num_elements` is greater than 1, the stride in bytes between the start of consecutive
    /// elements.
    pub stride: u32,
}

impl VertexMemberInfo {
    #[inline]
    pub fn num_components(&self) -> u32 {
        self.format
            .components()
            .iter()
            .filter(|&bits| *bits > 0)
            .count() as u32
    }
}
