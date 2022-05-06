use std::io::Write;
use std::time;

use rt_weekend::vec3::Color;

fn main() {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    let instant = time::Instant::now();

    for j in (0..IMAGE_HEIGHT).rev() {
        flush_progress(j, IMAGE_HEIGHT);

        for i in 0..IMAGE_WIDTH {
            let pixel_color = Color::new(
                i as f64 / (IMAGE_WIDTH - 1) as f64,
                j as f64 / (IMAGE_WIDTH - 1) as f64,
                0.25,
            );
            pixel_color.write_color();
        }
    }

    eprintln!();
    eprintln!("Done.");
    eprintln!("Time elapsed: {}s", instant.elapsed().as_secs())
}

fn flush_progress(current_row: u32, number_of_rows: u32) {
    let stderr = std::io::stderr();
    let mut handle = stderr.lock();
    handle
        .write_all(format!("\rProgress: {}%", 100 - current_row * 100 / number_of_rows).as_bytes())
        .expect("Failed to write progress");
    handle.flush().expect("Failed to flush progress");
}
