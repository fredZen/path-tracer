fn main() {
    let nx = 200;
    let ny = 100;
    print!("P3\n{} {}\n255\n", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = i as f64 / nx as f64;
            let g = j as f64 / ny as f64;
            let b = 0.2;

            let ir = (255.99 * r) as u8;
            let ig = (255.99 * g) as u8;
            let ib = (255.99 * b) as u8;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
