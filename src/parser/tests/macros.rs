macro_rules! test_data {
    ($($ty:ident { $($val:expr),* $(,)* })*) => {{
        use byteorder::{ByteOrder, LittleEndian};
        use std::mem;
        let mut vec = Vec::new();
        let mut _n = 0;
        $(
            let size = mem::size_of::<$ty>();
            $(
                for _ in 0..size {
                    vec.push(0);
                }
                write_test_data!(&mut vec[_n.._n+size], $ty, $val);
            _n += size;
            )*
        )*
        vec
    }};
}

macro_rules! write_test_data {
    ($slice:expr, i16, $val:expr) => {
        LittleEndian::write_i16($slice, $val);
    };
    ($slice:expr, i32, $val:expr) => {
        LittleEndian::write_i32($slice, $val);
    };
    ($slice:expr, i64, $val:expr) => {
        LittleEndian::write_i64($slice, $val);
    };
    ($slice:expr, f32, $val:expr) => {
        LittleEndian::write_f32($slice, $val);
    };
    ($slice:expr, f64, $val:expr) => {
        LittleEndian::write_f64($slice, $val);
    };
    ($slice:expr, u8, $val:expr) => {
        $slice[0] = $val;
    };
    ($slice:expr, u16, $val:expr) => {
        LittleEndian::write_u16($slice, $val);
    };
    ($slice:expr, u32, $val:expr) => {
        LittleEndian::write_u32($slice, $val);
    };
    ($slice:expr, u64, $val:expr) => {
        LittleEndian::write_u64($slice, $val);
    };
}
