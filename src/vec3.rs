mod col;
mod dir;
mod pos;

#[cfg(test)]
mod tests;

pub use col::Col;
pub use dir::Dir;
pub use pos::Pos;

pub type Float = f32;

pub trait Vector {
    fn squared_length(self) -> Float;
    fn length(self) -> Float;
    fn make_unit_vector(&mut self);
    fn dot(self, o: Self) -> Float;
    fn cross(self, o: Self) -> Self;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3([Float; 3]);

impl Vec3 {
    pub fn zero() -> Vec3 {
        Vec3([0., 0., 0.])
    }

    pub fn new(e0: Float, e1: Float, e2: Float) -> Vec3 {
        Vec3([e0, e1, e2])
    }
}

impl Vector for Vec3 {
    fn squared_length(self) -> Float {
        self.dot(self)
    }

    fn length(self) -> Float {
        self.squared_length().sqrt()
    }

    fn make_unit_vector(&mut self) {
        *self /= self.length();
    }

    fn dot(self, o: Vec3) -> Float {
        self.0.iter().zip(o.0.iter()).map(|(c1, c2)| c1 * c2).sum()
    }

    fn cross(self, o: Vec3) -> Vec3 {
        Vec3::new(
            self[1] * o[2] - self[2] * o[1],
            self[2] * o[0] - self[0] * o[2],
            self[0] * o[1] - self[1] * o[0],
        )
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParseVec3Error {
    RegexMatchError,
    ParseFloatError(std::num::ParseFloatError),
}

impl From<std::num::ParseFloatError> for ParseVec3Error {
    fn from(e: std::num::ParseFloatError) -> ParseVec3Error {
        ParseVec3Error::ParseFloatError(e)
    }
}

impl std::str::FromStr for Vec3 {
    type Err = ParseVec3Error;

    fn from_str(s: &str) -> Result<Vec3, Self::Err> {
        use regex::Regex;
        let re = Regex::new(r"\s*([0-9.]+)\s+([0-9.]+)\s+([0-9.]+)").unwrap();
        let groups = re.captures(s).ok_or(ParseVec3Error::RegexMatchError)?;
        Ok(Vec3::new(
            groups[1].parse()?,
            groups[2].parse()?,
            groups[3].parse()?,
        ))
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{} {} {}", self[0], self[1], self[2])
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        self * -1.
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = Float;

    fn index(&self, i: usize) -> &Float {
        &self.0[i]
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut Float {
        &mut self.0[i]
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, o: Vec3) -> Vec3 {
        Vec3::new(self[0] + o[0], self[1] + o[1], self[2] + o[2])
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, o: Vec3) -> Vec3 {
        Vec3::new(self[0] - o[0], self[1] - o[1], self[2] - o[2])
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, o: Vec3) -> Vec3 {
        Vec3::new(self[0] * o[0], self[1] * o[1], self[2] * o[2])
    }
}

impl std::ops::Div for Vec3 {
    type Output = Vec3;

    fn div(self, o: Vec3) -> Vec3 {
        Vec3::new(self[0] / o[0], self[1] / o[1], self[2] / o[2])
    }
}

impl std::ops::Mul<Vec3> for Float {
    type Output = Vec3;

    fn mul(self, o: Vec3) -> Vec3 {
        Vec3::new(self * o[0], self * o[1], self * o[2])
    }
}

impl std::ops::Div<Float> for Vec3 {
    type Output = Vec3;

    fn div(self, o: Float) -> Vec3 {
        Vec3::new(self[0] / o, self[1] / o, self[2] / o)
    }
}

impl std::ops::Mul<Float> for Vec3 {
    type Output = Vec3;

    fn mul(self, o: Float) -> Vec3 {
        Vec3::new(self[0] * o, self[1] * o, self[2] * o)
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, i: Vec3) {
        for (s, i) in self.0.iter_mut().zip(i.0.iter()) {
            *s += i;
        }
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, i: Vec3) {
        for (s, i) in self.0.iter_mut().zip(i.0.iter()) {
            *s -= i;
        }
    }
}

impl std::ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, i: Vec3) {
        for (s, i) in self.0.iter_mut().zip(i.0.iter()) {
            *s *= i;
        }
    }
}

impl std::ops::DivAssign for Vec3 {
    fn div_assign(&mut self, i: Vec3) {
        for (s, i) in self.0.iter_mut().zip(i.0.iter()) {
            *s /= i;
        }
    }
}

impl std::ops::MulAssign<Float> for Vec3 {
    fn mul_assign(&mut self, t: Float) {
        for s in self.0.iter_mut() {
            *s *= t;
        }
    }
}

impl std::ops::DivAssign<Float> for Vec3 {
    fn div_assign(&mut self, t: Float) {
        for s in self.0.iter_mut() {
            *s /= t;
        }
    }
}
