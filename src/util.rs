use std::ops::{Add, AddAssign, Mul, Sub};

pub type Dim = i64;
pub type DimReal = f32;

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Coord {
    pub row: Dim,
    pub col: Dim,
}

impl Coord {
    pub const ZERO: Self = Self { row: 0, col: 0 };

    pub fn to_real(self) -> CoordReal {
        CoordReal {
            row: self.row as DimReal,
            col: self.col as DimReal,
        }
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq)]
pub struct CoordReal {
    pub row: DimReal,
    pub col: DimReal,
}

impl CoordReal {
    pub const ZERO: Self = Self { row: 0.0, col: 0.0 };
}

impl Add for CoordReal {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl AddAssign for CoordReal {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for CoordReal {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (rhs * -1.0)
    }
}

impl Mul<DimReal> for CoordReal {
    type Output = Self;

    fn mul(self, rhs: DimReal) -> Self::Output {
        Self::Output {
            row: self.row * rhs,
            col: self.col * rhs,
        }
    }
}
