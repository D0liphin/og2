use crate::*;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
/// A 3-column, 2-row matrix
pub struct Matrix3x2 {
    pub i: Vector2,
    pub j: Vector2,
    pub k: Vector2,
}

// pub instance
impl Matrix3x2 {
    /// Instantiates a new `Matrix3x2` from the three provided column vectors:
    /// `i`, `j` and `k`.
    pub const fn new(i: Vector2, j: Vector2, k: Vector2) -> Self {
        Self { i, j, k }
    }
}

// pub static
impl Matrix3x2 {
    /// Creates a matrix that represents the composition of `lhs` and `rhs` (`lhs` ◦ `rhs`).
    pub fn compose(lhs: &Self, rhs: &Self) -> Self {
        let i = Vector2::new(lhs.i.x, lhs.i.y) * rhs.i.x + Vector2::new(lhs.j.x, lhs.j.y) * rhs.i.y;
        let j = Vector2::new(lhs.i.x, lhs.i.y) * rhs.j.x + Vector2::new(lhs.j.x, lhs.j.y) * rhs.j.y;
        let k = Vector2::new(lhs.k.x + rhs.k.x, lhs.k.y + rhs.k.y);
        let mat = Self { i, j, k };
        mat
    }
}

// pub(crate)
impl Matrix3x2 {
    /// Returns a raw buffer containing a copy of the data in this matrix.
    /// The buffer is formatted to satisfy the requirements of the wgsl uniform buffer
    pub(crate) fn create_raw_buffer(&self) -> [u8; 48] {
        use std::{mem, ptr};
        let mut buffer: [u8; 48] = unsafe { mem::MaybeUninit::uninit().assume_init() };
        for (i, vec) in [self.i, self.j, self.k].iter().enumerate() {
            let dst = &mut buffer[i << 4] as *mut u8;
            unsafe { ptr::copy(vec as *const Vector2 as *const u8, dst, 8) }
        }
        buffer
    }

    pub(crate) fn as_mat2x2(&self) -> Matrix2x2 {
        unsafe { *(self as *const Self as *const Matrix2x2) }.clone()
    }
}
