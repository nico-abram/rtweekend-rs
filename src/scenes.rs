use super::*;

/// The random scene for the end of the bookSW
pub fn random_scene(rand: &mut RandState) -> HittableList {
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

/// The random scene for the end of the book with pastel colors
pub fn pastel_scene(rand: &mut RandState) -> HittableList {
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
                    rand.random_double_range(0.9, 1.0),
                );
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

/// A random scene with a couple big spheres around the main ground
pub fn moon_scene(rand: &mut RandState) -> HittableList {
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
                    rand.random_double() as u8 as f64,
                );
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
        center: Vec3::new(1000.0, 1000.0, -1700.0),
        radius: 700.0,
        material: moon1_material,
    });
    let moon2_material = LambertianDiffuse::new(Vec3::new(0.0, 0.3, 0.0));
    world.add(Sphere {
        center: Vec3::new(1000.0, 0.0, 0.0),
        radius: 700.0,
        material: moon2_material,
    });
    let moon3_material = LambertianDiffuse::new(Vec3::new(0.0, 0.0, 0.5));
    world.add(Sphere {
        center: Vec3::new(1000.0, 1300.0, 2000.0),
        radius: 700.0,
        material: moon3_material,
    });
    let moon_front_material = LambertianDiffuse::new(Vec3::new(0.1, 0.2, 0.3));
    world.add(Sphere {
        center: Vec3::new(-1000.0, 0.0, 0.0),
        radius: 700.0,
        material: moon_front_material,
    });

    world
}

/// The scene used to test field of view
pub fn red_blue_scene() -> HittableList {
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

/// The simple scene used through most of the book
pub fn normal_scene() -> HittableList {
    let mut world = HittableList(vec![]);

    let material_ground = LambertianDiffuse::new(Vec3::new(0.8, 0.8, 0.0));
    let material_center = LambertianDiffuse::new(Vec3::new(0.1, 0.2, 0.5));
    //let material_left = Metal::new(0.8, 0.8, 0.8, 0.3);
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

/// Deterministic scene used for performance tests
pub fn perf_scene() -> HittableList {
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
