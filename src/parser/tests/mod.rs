use nom::IResult::*;
use super::*;

#[macro_use]
mod macros;

#[test]
fn parse_stl() {
    let bytes = test_data! {
        // Header
        u64 {
            // The spec says this has to be ASCII, so let's make it spaces.
            0x2020202020202020, 0x2020202020202020, 0x2020202020202020, 0x2020202020202020,
            0x2020202020202020, 0x2020202020202020, 0x2020202020202020, 0x2020202020202020,
            0x2020202020202020, 0x2020202020202020
        }
        u32 { 2 }
        // Vertex One
        f32 { 1.2, 3.4, 5.6 }
        f32 { 7.8, 9.10, 11.12 }
        f32 { 13.14, 15.16, 17.18 }
        f32 { 19.20, 21.22, 23.24 }
        u16 { 0 }
        // Vertex Two
        f32 { 25.26, 27.28, 29.30 }
        f32 { 31.32, 33.34, 35.36 }
        f32 { 37.38, 39.40, 41.42 }
        f32 { 43.44, 45.46, 47.48 }
        u16 { 0 }
    };
    assert_eq!(parse(&bytes), Done(&[] as &[u8], Stl(vec![
        Triangle {
            norm: Vertex(1.2, 3.4, 5.6),
            vertices: [
                Vertex(7.8, 9.10, 11.12),
                Vertex(13.14, 15.16, 17.18),
                Vertex(19.20, 21.22, 23.24),
            ],
        },
        Triangle {
            norm: Vertex(25.26, 27.28, 29.30),
            vertices: [
                Vertex(31.32, 33.34, 35.36),
                Vertex(37.38, 39.40, 41.42),
                Vertex(43.44, 45.46, 47.48),
            ],
        },
    ])));
}

#[test]
fn parse_triangle() {
    let bytes = test_data! {
        f32 { 1.2, 3.4, 5.6 }
        f32 { 7.8, 9.10, 11.12 }
        f32 { 13.14, 15.16, 17.18 }
        f32 { 19.20, 21.22, 23.24 }
        u16 { 0 }
    };
    assert_eq!(triangle(&bytes), Done(&[] as &[u8], Triangle {
        norm: Vertex(1.2, 3.4, 5.6),
        vertices: [
            Vertex(7.8, 9.10, 11.12),
            Vertex(13.14, 15.16, 17.18),
            Vertex(19.20, 21.22, 23.24),
        ],
    }))
}

#[test]
#[should_panic]
fn parse_triangle_bad_attrs() {
    let bytes = test_data! {
        f32 { 1.2, 3.4, 5.6 }
        f32 { 7.8, 9.10, 11.12 }
        f32 { 13.14, 15.16, 17.18 }
        f32 { 19.20, 21.22, 23.24 }
        u16 { 1 }
        u8 { 0x00 }
    };
    assert_eq!(triangle(&bytes), Done(&[] as &[u8], Triangle {
        norm: Vertex(1.2, 3.4, 5.6),
        vertices: [
            Vertex(7.8, 9.10, 11.12),
            Vertex(13.14, 15.16, 17.18),
            Vertex(19.20, 21.22, 23.24),
        ],
    }))
}

#[test]
fn parse_vertex() {
    let bytes = test_data! {
        f32 {
            1.2,
            3.4,
            5.6,
        }
    };
    assert_eq!(vertex(&bytes), Done(&[] as &[u8], Vertex(1.2, 3.4, 5.6)))
}
