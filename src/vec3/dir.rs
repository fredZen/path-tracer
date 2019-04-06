use crate::vec3::{Float, ParseVec3Error, Vec3, Vector};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Dir(pub(super) Vec3);

impl Dir {
    fn zero() -> Self {
        Self(Vec3::zero())
    }

    pub fn new(x: Float, y: Float, z: Float) -> Self {
        Self(Vec3::new(x, y, z))
    }

    pub fn x(self) -> Float {
        self.0[0]
    }

    pub fn y(self) -> Float {
        self.0[1]
    }

    pub fn z(self) -> Float {
        self.0[2]
    }
}

impl Vector for Dir {
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

    fn unit_vector(self) -> Self {
        Self(self.0.unit_vector())
    }
}

impl std::str::FromStr for Dir {
    type Err = ParseVec3Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.parse()?;
        Ok(Self(v))
    }
}

impl std::fmt::Display for Dir {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

impl std::ops::Neg for Dir {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Dir(-self.0)
    }
}

impl std::ops::Index<usize> for Dir {
    type Output = Float;

    fn index(&self, i: usize) -> &Float {
        &self.0[i]
    }
}

impl std::ops::IndexMut<usize> for Dir {
    fn index_mut(&mut self, i: usize) -> &mut Float {
        &mut self.0[i]
    }
}

impl std::ops::Add for Dir {
    type Output = Self;

    fn add(self, o: Self) -> Self::Output {
        Dir(self.0 + o.0)
    }
}

impl std::iter::Sum for Dir {
    fn sum<I: Iterator<Item = Dir>>(iter: I) -> Dir {
        Dir(iter.map(|d| d.0).sum())
    }
}

impl std::ops::Sub for Dir {
    type Output = Self;

    fn sub(self, o: Self) -> Self::Output {
        Dir(self.0 - o.0)
    }
}

impl std::ops::Mul for Dir {
    type Output = Self;

    fn mul(self, o: Self) -> Self::Output {
        Dir(self.0 * o.0)
    }
}

impl std::ops::Div for Dir {
    type Output = Self;

    fn div(self, o: Self) -> Self::Output {
        Dir(self.0 / o.0)
    }
}

impl std::ops::Mul<Dir> for Float {
    type Output = Dir;

    fn mul(self, o: Dir) -> Self::Output {
        Dir(self * o.0)
    }
}

impl std::ops::Div<Float> for Dir {
    type Output = Self;

    fn div(self, o: Float) -> Self::Output {
        Dir(self.0 / o)
    }
}

impl std::ops::Mul<Float> for Dir {
    type Output = Self;

    fn mul(self, o: Float) -> Self::Output {
        Dir(self.0 * o)
    }
}

impl std::ops::AddAssign for Dir {
    fn add_assign(&mut self, i: Self) {
        self.0 += i.0
    }
}

impl std::ops::SubAssign for Dir {
    fn sub_assign(&mut self, i: Self) {
        self.0 -= i.0
    }
}

impl std::ops::MulAssign for Dir {
    fn mul_assign(&mut self, i: Self) {
        self.0 *= i.0
    }
}

impl std::ops::DivAssign for Dir {
    fn div_assign(&mut self, i: Self) {
        self.0 /= i.0
    }
}

impl std::ops::MulAssign<Float> for Dir {
    fn mul_assign(&mut self, t: Float) {
        self.0 *= t
    }
}

impl std::ops::DivAssign<Float> for Dir {
    fn div_assign(&mut self, t: Float) {
        self.0 /= t
    }
}
