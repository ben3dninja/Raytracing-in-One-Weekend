use std::f64::INFINITY;
use std::io::Write;
use std::rc::Rc;
use std::time;

use rand::{thread_rng, Rng};
use rt_weekend::{camera::Camera, ray::Ray, sphere::Sphere, vec3::Color, world::World};

fn main() {
    // Image and camera settings
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 50;

    const FOCAL_LENGTH: f64 = 1.0;

    let cam = Camera::new(ASPECT_RATIO, FOCAL_LENGTH);

    // World settings

    let mut world = World::empty();
    world.add(Rc::new(Sphere::ni(0, 0, -1, 0.5)));
    world.add(Rc::new(Sphere::n(0.0, -100.5, -1.0, 100.0)));

    // Rendering image

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let instant = time::Instant::now();

    // The origin of the image is the bottom left corner
    for pixel_y in (0..IMAGE_HEIGHT).rev() {
        flush_progress(pixel_y, IMAGE_HEIGHT);
        for pixel_x in 0..IMAGE_WIDTH {
            let mut color_sum = Color::zero();
            for _ in 0..SAMPLES_PER_PIXEL {
                // u and v are the coordinates of the "holes" the rays punch through the viewport, in the
                // (bottom_left_corner, horizontal, vertical) base
                let u =
                    (pixel_x as f64 + thread_rng().gen_range(0.0..1.0)) / (IMAGE_WIDTH - 1) as f64;
                let v =
                    (pixel_y as f64 + thread_rng().gen_range(0.0..1.0)) / (IMAGE_HEIGHT - 1) as f64;
                let ray = cam.shoot_ray(u, v);
                color_sum += compute_ray_color(&ray, &world);
            }
            color_sum.average_and_write_color(SAMPLES_PER_PIXEL);
        }
    }

    eprintln!();
    eprintln!("Done.");
    eprintln!("Time elapsed: {}s", instant.elapsed().as_secs())
}

fn flush_progress(current_col: u32, total_cols: u32) {
    let stderr = std::io::stderr();
    let mut handle = stderr.lock();
    handle
        .write_all(format!("\rProgress: {}%", 100 - current_col * 100 / total_cols).as_bytes())
        .expect("Failed to write progress");
    handle.flush().expect("Failed to flush progress");
}

fn compute_ray_color(ray: &Ray, world: &World) -> Color {
    if let Some(record) = world.trace_ray(ray, 0.0, INFINITY).as_ref() {
        // This uses the hit record's normal vector as a base for the color
        0.5 * (&record.normal + Color::new(1.0, 1.0, 1.0))
    } else {
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        // This creates a gradient from blue (top) to white (bottom)
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
