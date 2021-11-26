use std::ops::{Add, Mul, Sub};

#[inline]
pub fn any_sized_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    unsafe {
        ::std::slice::from_raw_parts((p as *const T) as *const u8, ::std::mem::size_of::<T>())
    }
}

#[inline]
pub fn any_slice_as_u8_slice<T>(p: &[T]) -> &[u8] {
    unsafe {
        ::std::slice::from_raw_parts(
            p.as_ptr() as *const u8,
            p.len() * ::std::mem::size_of::<T>(),
        )
    }
}

#[inline]
pub fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T {
    debug_assert!(min <= max, "min must be less than or equal to max");
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}

#[inline]
pub fn lerp<T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy>(
    start: T,
    end: T,
    pos: T,
) -> T {
    start + (end - start) * pos
}
