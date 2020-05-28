use getopts::Options;
use image::{DynamicImage, GenericImageView, Pixel};
use std::env;

// http://paulbourke.net/dataformats/asciiart/
const RAMP: &[u8] = b" .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
// const RAMP: &[u8] = b" .:-=+*#%@";

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt(
        "s",
        "",
        "max size for the output. -1 will not scale the output at all.",
        "SIZE",
    );
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    let max_size: i32 = match matches.opt_str("s") {
        Some(x) => match x.parse() {
            Ok(x) => x,
            Err(e) => panic!(e.to_string()),
        },
        None => 250,
    };
    let path = matches.free.get(0).unwrap();

    let img = image::open(path).unwrap();
    let g = img.grayscale();
    let s = to_ascii(g, max_size);
    println!("{}", s)
}

fn to_ascii(img: DynamicImage, max_size: i32) -> String {
    let (w, h) = img.dimensions();
    let scale_factor = match max_size {
        -1 => 1,
        _ => {
            let hyp = ((w.pow(2) + h.pow(2)) as f64).sqrt();
            let sf = (hyp / max_size as f64).ceil() as u32;
            sf
        }
    };

    let mut s = String::new();
    for y in 0..h {
        if y % (scale_factor * 2) != 0 {
            continue;
        }
        for x in 0..w {
            if x % scale_factor != 0 {
                continue;
            }
            let px = img.get_pixel(x, y);
            let (r, _, _, _) = px.channels4();
            s.push(to_ascii_char(r))
        }
        s.push('\n');
    }

    return s;
}

fn to_ascii_char(i: u8) -> char {
    let x = i as f32 / 256.0;
    let c = (x * RAMP.len() as f32) as usize;
    RAMP[c] as char
}
