mod col;
mod pos;
mod vec3;

use vec3::Float;

fn as_u8(f: Float) -> u8 {
    if f < 1. {
        (256. * f) as u8
    } else {
        255
    }
}

fn main() {
    use col::Col;

    let nx = 200u16;
    let ny = 100u16;
    print!("P3\n{} {}\n255\n", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let col = Col::new(
                Float::from(i) / Float::from(nx),
                Float::from(j) / Float::from(ny),
                0.2,
            );

            let ir = as_u8(col.r());
            let ig = as_u8(col.g());
            let ib = as_u8(col.b());

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
