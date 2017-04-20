#[cfg(test)]
extern crate byteorder;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate nom;

mod parser;

error_chain! {
    errors {
        Nom(err: nom::ErrorKind) {
            description("Invalid STL content")
            display("Invalid STL content: {}", err)
        }
        TooLong {
            description("The STL contained extra content")
        }
        TooShort {
            description("The STL ended early")
        }
    }
}

/// An STL file.
#[derive(Clone, Debug, PartialEq)]
pub struct Stl {
    pub triangles: Vec<Triangle>
}

impl Stl {
    pub fn parse(bytes: &[u8]) -> Result<Stl> {
        use nom::IResult::*;
        match parser::parse(bytes) {
            Done(r, stl) => {
                assert_eq!(r.len(), 0);
                Ok(stl)
            },
            Incomplete(..) => Err(ErrorKind::TooShort.into()),
            Error(err) => Err(ErrorKind::Nom(err).into()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Triangle {
    pub norm: Vertex,
    pub vertices: [Vertex; 3],
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vertex(f32, f32, f32);
