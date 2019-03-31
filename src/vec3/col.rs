ase crate::vec3::{Float, ParseVec3Error, Vec3, Vector};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Col(Vec3);

impl Col {
    fn zero() -> Col {
        Col(Vec3::zero())
    }

    pub fn new(r: Float, g: Float, b: Float) -> Col {
        Col(Vec3::new(r, g, b))
    }

    pub fn r(self) -> Float {
        self.0[0]
    }

    pub fn g(self) -> Float {
        self.0[1]
    }

    pub fn b(self) -> Float {
        self.0[2]
    }
}

impl Vector for Col {
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

impl std::str::FromStr for Col {
    type Err = ParseVec3Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.parse()?;
        Ok(Self(v))
    }
}

impl std::fmt::Display for Col {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

impl std::ops::Neg for Col {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Col(-self.0)
    }
}

impl std::ops::Index<usize> for Col {
    type Output = Float;

    fn index(&self, i: usize) -> &Float {
        &self.0[i]
    }
}

impl std::ops::IndexMut<usize> for Col {
    fn index_mut(&mut self, i: usize) -> &mut Float {
        &mut self.0[i]
    }
}

impl std::ops::Add for Col {
    type Output = Self;

    fn add(self, o: Self) -> Self::Output {
        Col(self.0 + o.0)
    }
}

impl std::ops::Sub for Col {
    type Output = Self;

    fn sub(self, o: Self) -> Self::Output {
        Col(self.0 - o.0)
    }
}

impl std::ops::Mul for Col {
    type Output = Self;

    fn mul(self, o: Self) -> Self::Output {
        Col(self.0 * o.0)
    }
}

impl std::ops::Div for Col {
    type Output = Self;

    fn div(self, o: Self) -> Self::Output {
        Col(self.0 / o.0)
    }
}

impl std::ops::Mul<Col> for Float {
    type Output = Col;

    fn mul(self, o: Col) -> Self::Output {
        Col(self * o.0)
    }
}

impl std::ops::Div<Float> for Col {
    type Output = Self;

    fn div(self, o: Float) -> Self::Output {
        Col(self.0 / o)
    }
}

impl std::ops::Mul<Float> for Col {
    type Output = Self;

    fn mul(self, o: Float) -> Self::Output {
        Col(self.0 * o)
    }
}

impl std::ops::AddAssign for Col {
    fn add_assign(&mut self, i: Self) {
        self.0 += i.0
    }
}

impl std::ops::SubAssign for Col {
    fn sub_assign(&mut self, i: Self) {
        self.0 -= i.0
    }
}

impl std::ops::MulAssign for Col {
    fn mul_assign(&mut self, i: Self) {
        self.0 *= i.0
    }
}

impl std::ops::DivAssign for Col {
    fn div_assign(&mut self, i: Self) {
        self.0 /= i.0
    }
}

impl std::ops::MulAssign<Float> for Col {
    fn mul_assign(&mut self, t: Float) {
        self.0 *= t
    }
}

impl std::ops::DivAssign<Float> for Col {
    fn div_assign(&mut self, t: Float) {
        self.0 /= t
    }
}
