use std::{
    env,
    f32::consts::TAU,
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

const N: usize = 256;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("trig_lut.rs");
    let file = File::create(path).unwrap();
    let mut fb = BufWriter::new(file);

    writeln!(fb, "pub(crate) static TRIG_LUT: [(f32, f32); {N}] = [").unwrap();

    for i in 0..N {
        let a = (i as f32 / N as f32) * TAU;
        writeln!(fb, "\t{:?},", a.sin_cos()).unwrap();
    }

    writeln!(fb, "];").unwrap();
}
