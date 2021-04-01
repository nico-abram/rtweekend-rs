use super::{HitRecord, RandState, Ray, Vec3};

#[cfg(feature = "dyn_mat")]
use std::rc::Rc;

#[cfg(feature = "dyn_mat")]
pub type MaterialType = Rc<dyn Material>;
#[cfg(not(feature = "dyn_mat"))]
pub type MaterialType = EnumMat;

#[derive(Clone)]
pub enum EnumMat {
    Lamb(LambertianDiffuse),
    Diele(Dielectric),
    Met(Metal),
}
impl EnumMat {
    pub fn scatter(
        &self,
        rand: &mut RandState,
        incoming_ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scatter_ray: &mut Ray,
    ) -> bool {
        match self {
            EnumMat::Lamb(mat) => {
                mat.scatter(rand, incoming_ray, hit_record, attenuation, scatter_ray)
            }
            EnumMat::Diele(mat) => {
                mat.scatter(rand, incoming_ray, hit_record, attenuation, scatter_ray)
            }
            EnumMat::Met(mat) => {
                mat.scatter(rand, incoming_ray, hit_record, attenuation, scatter_ray)
            }
        }
    }
}
pub trait Material {
    fn scatter(
        &self,
        rand: &mut RandState,
        incoming_ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scatter_ray: &mut Ray,
    ) -> bool;
}
#[derive(Clone)]
pub struct LambertianDiffuse {
    albedo: Vec3,
}
impl LambertianDiffuse {
    #[cfg(feature = "dyn_mat")]
    pub fn new(albedo: Vec3) -> MaterialType {
        Rc::new(Self { albedo })
    }
    #[cfg(not(feature = "dyn_mat"))]
    pub fn new(albedo: Vec3) -> MaterialType {
        EnumMat::Lamb(Self { albedo })
    }
}
impl Material for LambertianDiffuse {
    fn scatter(
        &self,
        rand: &mut RandState,
        _incoming_ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scatter_ray: &mut Ray,
    ) -> bool {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector(rand);
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        *scatter_ray = Ray {
            orig: hit_record.p,
            dir: scatter_direction,
        };
        *attenuation = self.albedo;
        true
    }
}
#[derive(Clone)]
pub struct Metal {
    albedo: Vec3,
    fuzzyness: f64,
}
impl Metal {
    #[cfg(feature = "dyn_mat")]
    pub fn new(r: f64, g: f64, b: f64, fuzzyness: f64) -> MaterialType {
        Rc::new(Self {
            albedo: Vec3::new(r, g, b),
            fuzzyness: fuzzyness.min(1.0),
        })
    }
    #[cfg(not(feature = "dyn_mat"))]
    pub fn new(r: f64, g: f64, b: f64, fuzzyness: f64) -> MaterialType {
        EnumMat::Met(Self {
            albedo: Vec3::new(r, g, b),
            fuzzyness: fuzzyness.min(1.0),
        })
    }
}
impl Material for Metal {
    fn scatter(
        &self,
        rand: &mut RandState,
        incoming_ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scatter_ray: &mut Ray,
    ) -> bool {
        let reflected = incoming_ray.dir.unit_vector().reflect(hit_record.normal);

        *scatter_ray = Ray {
            orig: hit_record.p,
            dir: reflected + self.fuzzyness * Vec3::random_in_unit_sphere(rand),
        };
        *attenuation = self.albedo;

        scatter_ray.dir.dot(hit_record.normal) > 0.0
    }
}
#[derive(Clone)]
pub struct Dielectric {
    refraction_idx: f64,
}
impl Dielectric {
    #[cfg(feature = "dyn_mat")]
    pub fn new(refraction_idx: f64) -> MaterialType {
        Rc::new(Dielectric { refraction_idx })
    }
    #[cfg(not(feature = "dyn_mat"))]
    pub fn new(refraction_idx: f64) -> MaterialType {
        EnumMat::Diele(Dielectric { refraction_idx })
    }
}
fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
impl Material for Dielectric {
    fn scatter(
        &self,
        rand: &mut RandState,
        incoming_ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scatter_ray: &mut Ray,
    ) -> bool {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.refraction_idx
        } else {
            self.refraction_idx
        };

        let unit_dir = incoming_ray.dir.unit_vector();

        let cos_theta = (-unit_dir).dot(hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let dir =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > rand.random_double() {
                unit_dir.reflect(hit_record.normal)
            } else {
                Vec3::refract(unit_dir, hit_record.normal, refraction_ratio)
            };

        *scatter_ray = Ray {
            orig: hit_record.p,
            dir,
        };
        //*attenuation = Vec3::repeat(cannot_refract as u8 as f64);
        //*attenuation = Vec3::repeat(hit_record.front_face as u8 as f64);
        *attenuation = Vec3::repeat(1.0);

        true
    }
}
