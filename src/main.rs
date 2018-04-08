mod fileio;
mod vector;

fn main() {

    let nx = 200;
    let ny = 100;

    let mut pixels = String::from(format!("P3\n{} {}\n255\n", nx, ny));

    for j in (0..ny).rev() {
        for i in 0..nx {
            let col = vector::Vec3::new(i as f32 / nx as f32, j as f32 / ny as f32, 0.2 as f32);

            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;
            pixels.push_str(&format!("{} {} {} ", ir, ig, ib));
        }
        pixels.push_str("\n");
    }

    fileio::write_ppm("target/radium_output.ppm", &pixels);
}
