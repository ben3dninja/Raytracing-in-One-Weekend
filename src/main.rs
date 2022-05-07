use std::io::Write;
use std::time;

use rt_weekend::ray::Ray;
use rt_weekend::vec3::{Color, Point3, Vec3};

fn main() {
    // Image settings
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    // Camera settings

    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    const ORIGIN: Point3 = Point3::zero();
    const HORIZONTAL: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    const VERTICAL: Vec3 = Vec3::new(0.0, VIEWPORT_WIDTH, 0.0);

    let lower_left_corner: Point3 =
        &ORIGIN - &HORIZONTAL / 2.0 - &VERTICAL / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    // Rendering image

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let instant = time::Instant::now();

    for j in (0..IMAGE_HEIGHT).rev() {
        flush_progress(j, IMAGE_HEIGHT);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let ray = Ray::new(
                ORIGIN.clone(),
                lower_left_corner.clone() + u * &HORIZONTAL + v * &VERTICAL - &ORIGIN,
            );
            let pixel_color = ray_color(ray);
            pixel_color.write_color();
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

fn ray_color(ray: Ray) -> Color {
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
