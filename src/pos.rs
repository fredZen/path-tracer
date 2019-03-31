#[cfg(test)]
mod tests;

use crate::vec3::{Float, ParseVec3Error, Vec3, Vector};

struct Pos(Vec3);

impl Pos {
    fn zero() -> Self {
        Self(Vec3::zero())
    }

    fn new(x: Float, y: Float, z: Float) -> Self {
        Self(Vec3::new(x, y, z))
    }

    fn x(self) -> Float {
        self.0[0]
    }

    fn y(self) -> Float {
        self.0[1]
    }

    fn z(self) -> Float {
        self.0[2]
    }
}

impl Vector for Pos {
    fn squared_length(self) -> Float {
        self.0.squared_length()
    }

    fn length(self) -> Float {
        self.0.length()
    }

    fn make_unit_vector(&mut self) {
        self.0.make_unit_vector()
    }

    fn dot(self, o: Self) -> Float {
        self.0.dot(o.0)
    }

    fn cross(self, o: Self) -> Self {
        Self(self.0.cross(o.0))
    }
}

impl std::str::FromStr for Pos {
    type Err = ParseVec3Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.parse()?;
        Ok(Self(v))
    }
}

impl std::fmt::Display for Pos {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

impl std::ops::Neg for Pos {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Pos(-self.0)
    }
}

impl std::ops::Index<usize> for Pos {
    type Output = Float;

    fn index(&self, i: usize) -> &Float {
        &self.0[i]
    }
}

impl std::ops::IndexMut<usize> for Pos {
    fn index_mut(&mut self, i: usize) -> &mut Float {
        &mut self.0[i]
    }
}

impl std::ops::Add for Pos {
    type Output = Self;

    fn add(self, o: Self) -> Self::Output {
        Pos(self.0 + o.0)
    }
}

impl std::ops::Sub for Pos {
    type Output = Self;

    fn sub(self, o: Self) -> Self::Output {
        Pos(self.0 - o.0)
    }
}

impl std::ops::Mul for Pos {
    type Output = Self;

    fn mul(self, o: Self) -> Self::Output {
        Pos(self.0 * o.0)
    }
}

impl std::ops::Div for Pos {
    type Output = Self;

    fn div(self, o: Self) -> Self::Output {
        Pos(self.0 / o.0)
    }
}

impl std::ops::Mul<Pos> for Float {
    type Output = Pos;

    fn mul(self, o: Pos) -> Self::Output {
        Pos(self * o.0)
    }
}

impl std::ops::Div<Float> for Pos {
    type Output = Self;

    fn div(self, o: Float) -> Self::Output {
        Pos(self.0 / o)
    }
}

impl std::ops::Mul<Float> for Pos {
    type Output = Self;

    fn mul(self, o: Float) -> Self::Output {
        Pos(self.0 * o)
    }
}

impl std::ops::AddAssign for Pos {
    fn add_assign(&mut self, i: Self) {
        self.0 += i.0
    }
}

impl std::ops::SubAssign for Pos {
    fn sub_assign(&mut self, i: Self) {
        self.0 -= i.0
    }
}

impl std::ops::MulAssign for Pos {
    fn mul_assign(&mut self, i: Self) {
        self.0 *= i.0
    }
}

impl std::ops::DivAssign for Pos {
    fn div_assign(&mut self, i: Self) {
        self.0 /= i.0
    }
}

impl std::ops::MulAssign<Float> for Pos {
    fn mul_assign(&mut self, t: Float) {
        self.0 *= t
    }
}

impl std::ops::DivAssign<Float> for Pos {
    fn div_assign(&mut self, t: Float) {
        self.0 /= t
    }
}
