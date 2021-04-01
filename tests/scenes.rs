use rtweekend::{
    //material::{Dielectric, LambertianDiffuse, MaterialType, Metal}, HittableList,
    render,
    scenes,
    Camera,
    RandState,
    RenderParams,
    Vec3,
};

#[test]
fn test() {
    let rand = &mut RandState::new();

    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 200i64;
    let image_height = (image_width as f64 / aspect_ratio) as i64;
    let samples_per_px = 20;
    let max_depth = 50;

    let render_params = RenderParams {
        image_width,
        image_height,
        samples_per_px,
        max_depth,
    };

    let world = scenes::normal_scene();

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

    let _output = render(camera, world, render_params, rand);
}
