#![allow(dead_code)]

// Helper trait to unify numeric types
trait IntoValue {
    fn to_i64(&self) -> i64;
    fn to_i32(&self) -> i32;
    fn to_i16(&self) -> i16;
    fn  to_i8(&self) ->  i8;
    fn to_u64(&self) -> u64;
    fn to_u32(&self) -> u32;
    fn to_u16(&self) -> u16;
    fn  to_u8(&self) ->  u8;
    fn to_f64(&self) -> f64;
    fn to_f32(&self) -> f32;
}

// Implement for all standard numeric types
macro_rules! impl_into_value {
    ($($t:ty),*) => {
        $(
            impl IntoValue for $t {
                fn to_i64(&self) -> i64 { *self as i64 }
                fn to_i32(&self) -> i32 { *self as i32 }
                fn to_i16(&self) -> i16 { *self as i16 }
                fn  to_i8(&self) ->  i8 { *self as i8  }
                fn to_u64(&self) -> u64 { *self as u64 }
                fn to_u32(&self) -> u32 { *self as u32 }
                fn to_u16(&self) -> u16 { *self as u16 }
                fn  to_u8(&self) ->  u8 { *self as u8  }
                fn to_f64(&self) -> f64 { *self as f64 }
                fn to_f32(&self) -> f32 { *self as f32 }
            }
        )*
    };
}

impl_into_value!(i8, i16, i32, i64, u8, u16, u32, u64, usize, isize, f32, f64);