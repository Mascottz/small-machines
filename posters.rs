// small machines; the poster press. one file, no cargo, no crates;
// it exists as itself. compile and point it at a seed;
//
//   rustc posters.rs -o posters && ./posters 42 > poster-42.svg
//
// every seed is a calm poster from the same family; hairline frame,
// one loud shape, a field of quiet marks, and the type sitting low.

use std::env;
use std::process::ExitCode;

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        // splitmix64; small, fast, and honest about being a toy
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    fn f(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64
    }
    fn between(&mut self, a: f64, b: f64) -> f64 {
        a + self.f() * (b - a)
    }
    fn pick<'a, T>(&mut self, xs: &'a [T]) -> &'a T {
        &xs[self.next() as usize % xs.len()]
    }
}

fn main() -> ExitCode {
    let seed: u64 = env::args()
        .nth(1)
        .and_then(|a| a.parse().ok())
        .unwrap_or_else(|| {
            eprintln!("usage; ./posters <seed>");
            std::process::exit(1)
        });

    let mut r = Rng(seed.wrapping_mul(0x9E3779B97F4A7C15) ^ 0xA5A5A5A5);

    // four calm families; the seed picks the room
    let palettes = [
        ("#101418", "#E8E6E1", "#7FA8A0", "ink and verdigris"),
        ("#13221C", "#ECE7DB", "#C98E5F", "spruce and copper"),
        ("#1A1512", "#EDE4D7", "#B08D57", "umber and brass"),
        ("#141821", "#E6E4EA", "#8C93B8", "night and periwinkle"),
    ];
    let (bg, ink, accent, family) = r.pick(&palettes);

    let (w, h) = (1200.0, 1600.0);
    let mut s = String::new();
    s.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {} {}\" width=\"{}\" height=\"{}\">\n",
        w, h, w, h
    ));
    s.push_str(&format!("<rect width=\"{}\" height=\"{}\" fill=\"{}\"/>\n", w, h, bg));
    s.push_str(&format!(
        "<rect x=\"40\" y=\"40\" width=\"{}\" height=\"{}\" fill=\"none\" stroke=\"{}\" stroke-opacity=\"0.35\" stroke-width=\"2\"/>\n",
        w - 80.0,
        h - 80.0,
        ink
    ));

    // one loud shape, placed by the seed
    let cx = r.between(300.0, 900.0);
    let cy = r.between(350.0, 850.0);
    let kind = r.next() % 3;
    match kind {
        0 => {
            let rad = r.between(140.0, 260.0);
            s.push_str(&format!(
                "<circle cx=\"{cx}\" cy=\"{cy}\" r=\"{rad}\" fill=\"none\" stroke=\"{accent}\" stroke-width=\"{sw}\"/>\n",
                sw = r.between(3.0, 7.0)
            ));
        }
        1 => {
            let rad = r.between(160.0, 300.0);
            let a0 = r.f() * 360.0;
            let a1 = a0 + r.between(90.0, 240.0);
            s.push_str(&format!(
                "<path d=\"M {} {} A {rad} {rad} 0 0 1 {} {}\" fill=\"none\" stroke=\"{accent}\" stroke-width=\"{sw}\" stroke-linecap=\"round\"/>\n",
                cx + rad * (a0.to_radians()).cos(),
                cy + rad * (a0.to_radians()).sin(),
                cx + rad * (a1.to_radians()).cos(),
                cy + rad * (a1.to_radians()).sin(),
                sw = r.between(3.0, 7.0)
            ));
        }
        _ => {
            let len = r.between(240.0, 480.0);
            let ang = r.pick(&[45.0_f64, -45.0, 0.0, 90.0]).to_radians();
            s.push_str(&format!(
                "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{accent}\" stroke-width=\"{sw}\" stroke-linecap=\"round\"/>\n",
                cx - len / 2.0 * ang.cos(),
                cy - len / 2.0 * ang.sin(),
                cx + len / 2.0 * ang.cos(),
                cy + len / 2.0 * ang.sin(),
                sw = r.between(3.0, 7.0)
            ));
        }
    }

    // a field of quiet marks, lower third
    let cols = 9;
    let rows = 5;
    let gx0 = 140.0;
    let gy0 = r.between(980.0, 1080.0);
    let step = (w - 280.0) / (cols - 1) as f64;
    for row in 0..rows {
        for col in 0..cols {
            if r.f() < 0.25 {
                continue;
            }
            let x = gx0 + col as f64 * step;
            let y = gy0 + row as f64 * 44.0;
            let mark = r.next() % 3;
            let op = r.between(0.25, 0.6);
            match mark {
                0 => s.push_str(&format!(
                    "<line x1=\"{x}\" y1=\"{y}\" x2=\"{x}\" y2=\"{}\" stroke=\"{ink}\" stroke-opacity=\"{op}\" stroke-width=\"2\"/>\n",
                    y + 14.0
                )),
                1 => s.push_str(&format!(
                    "<circle cx=\"{x}\" cy=\"{y}\" r=\"3\" fill=\"{ink}\" fill-opacity=\"{op}\"/>\n"
                )),
                _ => s.push_str(&format!(
                    "<line x1=\"{}\" y1=\"{y}\" x2=\"{}\" y2=\"{y}\" stroke=\"{ink}\" stroke-opacity=\"{op}\" stroke-width=\"2\"/>\n",
                    x - 7.0,
                    x + 7.0
                )),
            }
        }
    }

    // type sits low, like a caption in a gallery
    s.push_str(&format!(
        "<text x=\"140\" y=\"{}\" font-family=\"Georgia, 'DejaVu Serif', serif\" font-size=\"84\" fill=\"{ink}\">small machines</text>\n",
        h - 210.0
    ));
    s.push_str(&format!(
        "<text x=\"140\" y=\"{}\" font-family=\"'DejaVu Sans Mono', monospace\" font-size=\"30\" fill=\"{accent}\">poster {seed:04} · {family}</text>\n",
        h - 150.0
    ));
    s.push_str("</svg>\n");

    print!("{s}");
    ExitCode::SUCCESS
}
