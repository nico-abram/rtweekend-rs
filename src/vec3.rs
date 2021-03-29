use super::RandState;

#[derive(Debug, Clone, Copy)]
pub struct Vec3(pub [f64; 3]);

impl Vec3 {
    #[inline(always)]
    pub const fn new(a: f64, b: f64, c: f64) -> Self {
        Self([a, b, c])
    }
    #[inline(always)]
    pub const fn zero() -> Self {
        Self([0.0; 3])
    }
    #[inline(always)]
    pub const fn repeat(x: f64) -> Self {
        Self([x, x, x])
    }
    #[inline(always)]
    pub const fn x(&self) -> f64 {
        self.0[0]
    }
    #[inline(always)]
    pub const fn y(&self) -> f64 {
        self.0[1]
    }
    #[inline(always)]
    pub const fn z(&self) -> f64 {
        self.0[2]
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn sum(&self) -> f64 {
        self.0.iter().sum()
    }
    pub fn dot(&self, other: &Vec3) -> f64 {
        (self * other).sum()
    }
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        let a = &self.0;
        let b = &other.0;
        Vec3([
            a[1] * b[2] - a[2] * b[1],
            a[2] * b[0] - a[0] * b[2],
            a[0] * b[1] - a[1] * b[0],
        ])
    }
    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }
    pub fn length_squared(&self) -> f64 {
        self.0.iter().map(|x| x.powi(2)).sum()
    }
    pub fn random(rand: &mut RandState) -> Vec3 {
        Vec3::new(
            rand.random_double(),
            rand.random_double(),
            rand.random_double(),
        )
    }
    pub fn random_range(rand: &mut RandState, min: f64, max: f64) -> Vec3 {
        Vec3::new(
            rand.random_double_range(min, max),
            rand.random_double_range(min, max),
            rand.random_double_range(min, max),
        )
    }
    pub fn random_in_unit_sphere(rand: &mut RandState) -> Vec3 {
        loop {
            let x = Vec3::random_range(rand, -1.0, 1.0);
            if x.length_squared() < 1.0 {
                return x;
            }
        }
    }
    pub fn random_in_unit_disk(rand: &mut RandState) -> Vec3 {
        loop {
            let x = Vec3::new(
                rand.random_double_range(-1.0, 1.0),
                rand.random_double_range(-1.0, 1.0),
                0.0,
            );
            if x.length_squared() < 1.0 {
                return x;
            }
        }
    }
    pub fn random_unit_vector(rand: &mut RandState) -> Vec3 {
        Vec3::random_in_unit_sphere(rand).unit_vector()
    }
    pub fn near_zero(&self) -> bool {
        let delta = 1e-8;
        self.0.iter().all(|d| d.abs() < delta)
    }
    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        self - &(2.0 * self.dot(normal) * normal)
    }
    pub fn refract(uv: Vec3, normal: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-uv).dot(&normal).min(1.0);
        let r_out_perp = etai_over_etat * (uv + cos_theta * normal);
        let x = 1.0 - r_out_perp.length_squared();
        let x = -x.abs().sqrt();
        let r_out_parallel = x * normal;
        r_out_perp + r_out_parallel
    }
}

impl std::ops::Neg for &Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Vec3([-self.x(), -self.y(), -self.z()])
    }
}
impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        -(&self)
    }
}

impl std::ops::Add for &Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn add(self, other: Self) -> Vec3 {
        Vec3([
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        ])
    }
}
impl std::ops::AddAssign<&Vec3> for Vec3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: &Vec3) {
        let tmp = &*self + rhs;
        *self = tmp;
    }
}
impl std::ops::AddAssign for Vec3 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Vec3) {
        *self += &rhs;
    }
}
impl std::ops::Add for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn add(self, other: Self) -> Vec3 {
        &self + &other
    }
}
impl std::ops::Sub for &Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn sub(self, other: Self) -> Vec3 {
        Vec3([
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        ])
    }
}
impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn sub(self, other: Self) -> Vec3 {
        &self - &other
    }
}
impl std::ops::Mul<f64> for &Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3([self.x() * rhs, self.y() * rhs, self.z() * rhs])
    }
}
impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, rhs: f64) -> Vec3 {
        &self * rhs
    }
}
impl std::ops::Mul<&Vec3> for f64 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, rhs: &Vec3) -> Vec3 {
        rhs * self
    }
}
impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, rhs: Vec3) -> Vec3 {
        &rhs * self
    }
}
impl std::ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn div(self, rhs: f64) -> Self::Output {
        Vec3([self.x() / rhs, self.y() / rhs, self.z() / rhs])
    }
}
impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn div(self, rhs: f64) -> Self::Output {
        &self / rhs
    }
}
impl std::ops::Mul for &Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, rhs: &Vec3) -> Vec3 {
        Vec3([self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z()])
    }
}
impl std::ops::Mul for Vec3 {
    type Output = Vec3;

    #[inline(always)]
    fn mul(self, rhs: Vec3) -> Vec3 {
        &self * &rhs
    }
}
