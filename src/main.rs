mod material;
mod vec3;

#[cfg(not(all(feature = "wincrypt_rand", target_os = "windows")))]
mod libc_rand;
#[cfg(all(feature = "wincrypt_rand", target_os = "windows"))]
mod win32_rand;

#[cfg(not(all(feature = "wincrypt_rand", target_os = "windows")))]
pub use libc_rand::RandState;
#[cfg(all(feature = "wincrypt_rand", target_os = "windows"))]
pub use win32_rand::RandState;

use material::{Dielectric, LambertianDiffuse, MaterialType, Metal};
use std::rc::Rc;
use vec3::Vec3;

pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}
impl Ray {
    fn new(orig: Vec3, dir: Vec3) -> Self {
        Self { orig, dir }
    }
    fn at(&self, t: f64) -> Vec3 {
        self.orig + t * self.dir
    }
}

const WHITE: Vec3 = Vec3::new(1.0, 1.0, 1.0);
const CYAN: Vec3 = Vec3::new(0.5, 0.7, 1.0);

fn lerp(t: f64, a: Vec3, b: Vec3) -> Vec3 {
    (1.0 - t) * a + t * b
}
#[derive(Clone)]
pub struct HitRecord {
    p: Vec3,
    normal: Vec3,
    t: f64,
    front_face: bool,
    material: MaterialType,
}
impl HitRecord {
    fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -outward_normal
        };
    }
}
impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Vec3::zero(),
            normal: Vec3::zero(),
            t: -1.0,
            front_face: false,
            material: LambertianDiffuse::new(Vec3::repeat(0.5)),
        }
    }
}
trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, out: &mut HitRecord) -> bool;
}
struct Sphere {
    center: Vec3,
    radius: f64,
    material: MaterialType,
}
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, out: &mut HitRecord) -> bool {
        let ray_to_sphere = ray.orig - self.center;
        let a = ray.dir.length_squared();
        let half_b = ray.dir.dot(&ray_to_sphere);
        let c = ray_to_sphere.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        out.t = root;
        out.p = ray.at(out.t);
        let outward_normal = (out.p - self.center) / self.radius;
        out.set_face_normal(ray, &outward_normal);
        out.material = self.material.clone();
        true
    }
}
#[cfg(feature = "dyn_hit")]
struct HittableList(Vec<Rc<dyn Hittable>>);
#[cfg(not(feature = "dyn_hit"))]
struct HittableList(Vec<Sphere>);
impl HittableList {
    #[allow(dead_code)]
    fn clear(&mut self) {
        self.0.clear();
    }
    #[cfg(feature = "dyn_hit")]
    fn add<T: Hittable + 'static>(&mut self, object: T) {
        self.0.push(Rc::new(object));
    }
    #[cfg(not(feature = "dyn_hit"))]
    fn add(&mut self, object: Sphere) {
        self.0.push(object);
    }
}
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, out: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut tmp_record = HitRecord::default();
        for obj in &self.0 {
            if obj.hit(r, t_min, closest_so_far, &mut tmp_record) {
                hit_anything = true;
                closest_so_far = tmp_record.t;
                *out = tmp_record.clone();
            }
        }
        hit_anything
    }
}
#[cfg(feature = "dyn_hit")]
type HitWorld<'a> = &'a dyn Hittable;
#[cfg(not(feature = "dyn_hit"))]
type HitWorld<'a> = &'a HittableList;
fn ray_color<'a>(rand: &mut RandState, world: HitWorld<'a>, r: &Ray, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::zero();
    }

    let mut hit_record = HitRecord::default();
    if world.hit(r, 0.001, f64::INFINITY, &mut hit_record) {
        //return hit_record.normal * 0.5 + Vec3::repeat(0.5);

        let mat = hit_record.material.clone();
        let mut attenuation = Vec3::zero();
        let mut scatter_ray = Ray::new(Vec3::zero(), Vec3::zero());
        let scatter = mat.scatter(rand, r, &hit_record, &mut attenuation, &mut scatter_ray);
        return if scatter {
            attenuation * ray_color(rand, world, &scatter_ray, depth - 1)
        } else {
            Vec3::zero()
        };
    }

    let unit_dir = r.dir.unit_vector();
    let t = 0.5 * (unit_dir.y() + 1.0);
    lerp(t, WHITE, CYAN)
    // For pastel background:
    //lerp(t, Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.86, 0.92, 1.0))
}

fn output_color(output_px: &mut [u8], pixel: Vec3, samples_per_px: u32) {
    let scale = 1.0 / (samples_per_px as f64); // Divide by samples_per_px using a multiplication

    let [r, g, b] = pixel.0;

    let [r, g, b] = [r * scale, g * scale, b * scale];

    // gamma correction
    let [r, g, b] = [r.sqrt(), g.sqrt(), b.sqrt()];

    let [r, g, b] = [r.clamp(0.0, 0.99), g.clamp(0.0, 0.99), b.clamp(0.0, 0.99)];

    let [r, g, b] = [(r * 256.0) as u8, (g * 256.0) as u8, (b * 256.0) as u8];

    output_px[0] = r;
    output_px[1] = g;
    output_px[2] = b;
}

struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f64,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}
impl Camera {
    fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        view_up: Vec3,
        vertical_fov_degrees: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vertical_fov_degrees.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (lookfrom - lookat).unit_vector();
        let u = view_up.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w,
            lens_radius: aperture / 2.0,
            u,
            v,
            w,
        }
    }

    fn get_ray(&self, rand: &mut RandState, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk(rand);
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray {
            orig: self.origin + offset,
            dir: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
        }
    }
}

#[allow(dead_code)]
fn random_scene(rand: &mut RandState) -> HittableList {
    let mut world = HittableList(Vec::with_capacity(22 * 22 + 5));

    let ground_material = LambertianDiffuse::new(Vec3::new(0.5, 0.5, 0.5));
    world.add(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material,
    });

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;
            let choose_mat = rand.random_double();
            let center = Vec3::new(
                a + 0.9 * rand.random_double(),
                0.2,
                b + 0.9 * rand.random_double(),
            );
            if choose_mat < 0.8 {
                // diffuse
                let albedo = Vec3::random(rand) * Vec3::random(rand);
                let material = LambertianDiffuse::new(albedo);
                world.add(Sphere {
                    center,
                    radius: 0.2,
                    material,
                });
            } else if choose_mat < 0.95 {
                // metal
                let material = Metal::new(
                    rand.random_double_range(0.5, 1.0),
                    rand.random_double_range(0.5, 1.0),
                    rand.random_double_range(0.5, 1.0),
                    rand.random_double_range(0.0, 0.5),
                );
                world.add(Sphere {
                    center,
                    radius: 0.2,
                    material,
                });
            } else {
                // glass
                let material = Dielectric::new(1.5);
                world.add(Sphere {
                    center,
                    radius: 0.2,
                    material,
                });
            }
        }
    }
    let material1 = Dielectric::new(1.5);
    world.add(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: material1,
    });
    let material2 = LambertianDiffuse::new(Vec3::new(0.4, 0.2, 0.1));
    world.add(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: material2,
    });
    let material3 = Metal::new(0.7, 0.6, 0.5, 0.0);
    world.add(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: material3,
    });

    world
}
#[allow(dead_code)]
fn pastel_scene(rand: &mut RandState) -> HittableList {
    let mut world = HittableList(Vec::with_capacity(22 * 22 + 5));

    let ground_material = LambertianDiffuse::new(Vec3::new(0.95, 0.95, 0.8));
    world.add(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material,
    });

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;
            let choose_mat = rand.random_double();
            let center = Vec3::new(
                a + 0.9 * rand.random_double(),
                0.2,
                b + 0.9 * rand.random_double(),
            );
            if choose_mat < 0.8 {
                // diffuse
                let albedo = Vec3::new(
                    rand.random_double_range(0.9, 1.0),
                    rand.random_double_range(0.9, 1.0),
                    rand.random_double_range(0.9, 1.0));
                let material = LambertianDiffuse::new(albedo);
                world.add(Sphere {
                    center,
                    radius: 0.2,
                    material,
                });
            } else if choose_mat < 0.95 {
                // metal
                let material = Metal::new(
                    rand.random_double_range(0.9, 1.0),
                    rand.random_double_range(0.9, 1.0),
                    rand.random_double_range(0.9, 1.0),
                    rand.random_double_range(0.0, 0.5),
                );
                world.add(Sphere {
                    center,
                    radius: 0.2,
                    material,
                });
            } else {
                // glass
                let material = Dielectric::new(1.5);
                world.add(Sphere {
                    center,
                    radius: 0.2,
                    material,
                });
            }
        }
    }
    let material1 = Dielectric::new(1.5);
    world.add(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: material1,
    });
    let material2 = LambertianDiffuse::new(Vec3::new(1.0, 0.9, 0.8));
    world.add(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: material2,
    });
    let material3 = Metal::new(0.6, 0.7, 0.8, 0.0);
    world.add(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: material3,
    });

    world
}

#[allow(dead_code)]
fn moon_scene(rand: &mut RandState) -> HittableList {
    let mut world = HittableList(Vec::with_capacity(22 * 22 + 5));

    let ground_material = LambertianDiffuse::new(Vec3::new(0.95, 0.95, 0.8));
    world.add(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material,
    });

    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;
            let choose_mat = rand.random_double();
            let center = Vec3::new(
                a + 0.9 * rand.random_double(),
                0.2,
                b + 0.9 * rand.random_double(),
            );
            if choose_mat < 0.8 {
                // diffuse
                let albedo = Vec3::new(
                    rand.random_double() as u8 as f64,
                    rand.random_double_range(0.9, 1.0),
                    rand.random_double() as u8 as f64,);
                let material = LambertianDiffuse::new(albedo);
                world.add(Sphere {
                    center,
                    radius: 0.2,
                    material,
                });
            } else if choose_mat < 0.95 {
                // metal
                let material = Metal::new(
                    rand.random_double() as u8 as f64,
                    rand.random_double() as u8 as f64,
                    rand.random_double_range(0.9, 1.0),
                    rand.random_double_range(0.0, 0.5),
                );
                world.add(Sphere {
                    center,
                    radius: 0.2,
                    material,
                });
            } else {
                // glass
                let material = Dielectric::new(1.5);
                world.add(Sphere {
                    center,
                    radius: 0.2,
                    material,
                });
            }
        }
    }
    let material1 = Dielectric::new(1.5);
    world.add(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: material1,
    });
    let material2 = LambertianDiffuse::new(Vec3::new(1.0, 0.9, 0.8));
    world.add(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: material2,
    });
    let material3 = Metal::new(0.6, 0.7, 0.8, 0.0);
    world.add(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: material3,
    });
    let moon1_material = LambertianDiffuse::new(Vec3::new(0.4, 0.0, 0.0));
    world.add(Sphere {
        center: Vec3::new(1000.0,1000.0 , -1700.0),
        radius: 700.0,
        material: moon1_material,
    });
    let moon2_material = LambertianDiffuse::new(Vec3::new(0.0, 0.3, 0.0));
    world.add(Sphere {
        center: Vec3::new(1000.0,0.0 , 0.0),
        radius: 700.0,
        material: moon2_material,
    });
    let moon3_material = LambertianDiffuse::new(Vec3::new(0.0, 0.0, 0.5));
    world.add(Sphere {
        center: Vec3::new(1000.0,1300.0 , 2000.0),
        radius: 700.0,
        material: moon3_material,
    });
    let moon_front_material = LambertianDiffuse::new(Vec3::new(0.1, 0.2, 0.3));
    world.add(Sphere {
        center: Vec3::new(-1000.0,0.0 , 0.0),
        radius: 700.0,
        material: moon_front_material,
    });

    world
}
#[allow(dead_code)]
fn red_blue_scene() -> HittableList {
    let mut world = HittableList(vec![]);

    let radius = (std::f64::consts::PI / 4.0).cos();
    let material_right = LambertianDiffuse::new(Vec3::new(0.0, 0.0, 1.0));
    let material_left = LambertianDiffuse::new(Vec3::new(1.0, 0.0, 0.0));
    world.add(Sphere {
        center: Vec3::new(-radius, 0.0, -1.0),
        radius: radius,
        material: material_left,
    });
    world.add(Sphere {
        center: Vec3::new(radius, 0.0, -1.0),
        radius: radius,
        material: material_right,
    });

    world
}
#[allow(dead_code)]
fn normal_scene() -> HittableList {
    let mut world = HittableList(vec![]);

    let material_ground = LambertianDiffuse::new(Vec3::new(0.8, 0.8, 0.0));
    let material_center = LambertianDiffuse::new(Vec3::new(0.1, 0.2, 0.5));
    let material_left = Metal::new(0.8, 0.8, 0.8, 0.3);
    let material_right = Metal::new(0.8, 0.6, 0.2, 0.0);

    let material_left = Dielectric::new(1.5);
    world.add(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: material_ground,
    });
    world.add(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: material_center,
    });
    world.add(Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: material_left.clone(),
    });
    world.add(Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: -0.4,
        material: material_left,
    });
    world.add(Sphere {
        center: Vec3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: material_right,
    });

    world
}
fn perf_scene() -> HittableList {
    let mut world = HittableList(Vec::with_capacity(22 * 22 + 5));

    let ground_material = LambertianDiffuse::new(Vec3::new(0.5, 0.5, 0.5));
    world.add(Sphere {
        center: Vec3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: ground_material,
    });

    let mut gen_double = {
        let mut tmp1 = 0;
        let mut tmp2 = 0;
        let mut tmp3 = 0;
        move || {
            tmp1 += 1;
            tmp2 += 3;
            tmp3 += 7;
            tmp1 = tmp1 % 100;
            tmp2 = tmp2 % 100;
            tmp3 = tmp3 % 100;
            ((tmp1 as f64) + (tmp2 as f64) + (tmp3 as f64)) / 300.0
        }
    };
    for a in -11..11 {
        for b in -11..11 {
            let a = a as f64;
            let b = b as f64;
            let choose_mat = gen_double();
            let center = Vec3::new(a + 0.9 * gen_double(), 0.2, b + 0.9 * gen_double());
            if choose_mat < 0.8 {
                // diffuse
                let albedo = Vec3::new(gen_double(), gen_double(), gen_double())
                    * Vec3::new(gen_double(), gen_double(), gen_double());
                let material = LambertianDiffuse::new(albedo);
                world.add(Sphere {
                    center,
                    radius: 0.2,
                    material,
                });
            } else if choose_mat < 0.95 {
                // metal
                let material = Metal::new(
                    0.5 + 0.5 * gen_double(),
                    0.5 + 0.5 * gen_double(),
                    0.5 + 0.5 * gen_double(),
                    0.5 * gen_double(),
                );
                world.add(Sphere {
                    center,
                    radius: 0.2,
                    material,
                });
            } else {
                // glass
                let material = Dielectric::new(1.5);
                world.add(Sphere {
                    center,
                    radius: 0.2,
                    material,
                });
            }
        }
    }
    let material1 = Dielectric::new(1.5);
    world.add(Sphere {
        center: Vec3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: material1,
    });
    let material2 = LambertianDiffuse::new(Vec3::new(0.4, 0.2, 0.1));
    world.add(Sphere {
        center: Vec3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: material2,
    });
    let material3 = Metal::new(0.7, 0.6, 0.5, 0.0);
    world.add(Sphere {
        center: Vec3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: material3,
    });

    world
}
fn main() {
    let mut rand = RandState::new();

    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 1200i64;
    let image_height = (image_width as f64 / aspect_ratio) as i64;
    let samples_per_px = 200;
    let max_depth = 50;

    // World
    //let world = normal_scene(&mut rand);
    //let world = red_blue_scene(&mut rand);
    //let world = moon_scene(&mut rand);
    //let world = random_scene(&mut rand);
    let world = pastel_scene(&mut rand);
    //let world = perf_scene();

    // Camera

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = 1.0;
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.0,
        dist_to_focus,
    );

    // Render
    println!("P3\n{} {}\n255", image_width, image_height);

    let mut output = vec![0u8; 3* (image_width*image_height) as usize];

    let scanline_iter = (0..image_height).rev().zip(output.chunks_mut(3*image_width as usize));
    #[cfg(feature = "parallel")]
    use rayon::prelude::*;
    #[cfg(feature = "parallel")]
    let scanline_iter = scanline_iter.collect::<Vec<_>>().into_par_iter();
    scanline_iter.for_each(|(i, output_scanline)| {
        #[cfg(not(feature = "parallel"))]
        write!(&mut std::io::stderr(), "\rScanlines remaining: {:04}", i).unwrap();

        let mut work =  |rand: &mut RandState| {
            for (j, output_px) in (0..image_width).zip(output_scanline.chunks_mut(3)) {
                let mut color = Vec3::zero();
                for _ in 0..samples_per_px {
                    let (u, v) = (
                        (j as f64 + rand.random_double()) / (image_width as f64 - 1.0),
                        (i as f64 + rand.random_double()) / (image_height as f64 - 1.0),
                    );
                    //let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
                    let r = camera.get_ray( rand, u, v);
                    color += ray_color(rand, &world, &r, max_depth);
                }

                output_color(output_px, color, samples_per_px); 
            }
        };
        
        #[cfg(feature = "parallel")]
        thread_local!(static RAND: std::cell::RefCell<RandState> = std::cell::RefCell::new(RandState::new()));
         RAND.with(|rand| {
            let mut rand = rand.borrow_mut();
            work(&mut*rand);
        });

        #[cfg(not(feature = "parallel"))]
        work(&mut rand);
    });

    let stdout = std::io::stdout();
    let lock = stdout.lock();
    let mut buf = std::io::BufWriter::new(lock);
    for x in output.chunks(3) {
        use std::io::Write;
        write!(&mut buf, "{} {} {}\n", x[0], x[1], x[2]).unwrap();
    }
}
