use nom::{le_f32, le_u16, le_u32};
use super::{Stl, Triangle, Vertex};

#[cfg(test)]
mod tests;

named!(pub parse(&[u8]) -> Stl, do_parse!(
    take!(80) >>
    facet_count: le_u32 >>
    triangles: count!(triangle, facet_count as usize) >>
    eof!() >>
    (Stl { triangles: triangles })
));

named!(triangle(&[u8]) -> Triangle, do_parse!(
    norm: vertex >>
    v1: vertex >>
    v2: vertex >>
    v3: vertex >>
    verify!(le_u16, |attr_count| attr_count == 0) >>
    (Triangle {
        norm: norm,
        vertices: [v1, v2, v3],
    })
));

named!(vertex(&[u8]) -> Vertex, do_parse!(
    x: le_f32 >>
    y: le_f32 >>
    z: le_f32 >>
    (Vertex(x, y, z))
));
