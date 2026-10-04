// small machines; lumen. the palette auditor. one file, no crates;
// it exists as itself.
//
//   rustc -O lumen.rs -o lumen
//   ./lumen                     audit the built-in calm palettes
//   ./lumen #0D1A15 #ECE7DB #C98E5F   audit your own; first hex is the background
//   ./lumen --selftest          the pinned checks
//
// every foreground and background pair is scored for the wcag contrast
// ratio, four times; once as-is, and once under each of protanopia,
// deuteranopia, and tritanopia, using the machado severity-one matrices.
// a pair passes at 4.5 or better in all four visions. when it fails,
// lumen walks the failing color toward white or black until every
// vision passes, and prints the fixed hex.

use std::env;
use std::process::ExitCode;

type Rgb = [f64; 3];

// machado, vieira, oliveira; a physiologically-based model, severity 1.0
const PROTANOPIA: [[f64; 3]; 3] = [
    [0.152286, 1.052583, -0.204868],
    [0.114503, 0.786281, 0.099216],
    [-0.003882, -0.048116, 1.051998],
];
const DEUTERANOPIA: [[f64; 3]; 3] = [
    [0.367322, 0.860646, -0.227968],
    [0.280085, 0.672501, 0.047413],
    [-0.011820, 0.042940, 0.968881],
];
const TRITANOPIA: [[f64; 3]; 3] = [
    [1.255528, -0.076749, -0.178779],
    [-0.078411, 0.930809, 0.147602],
    [0.004733, 0.691367, 0.303900],
];

fn hex_to_rgb(hex: &str) -> Option<Rgb> {
    let h = hex.trim_start_matches('#');
    if h.len() != 6 {
        return None;
    }
    let v = u32::from_str_radix(h, 16).ok()?;
    Some([
        ((v >> 16) & 255) as f64 / 255.0,
        ((v >> 8) & 255) as f64 / 255.0,
        (v & 255) as f64 / 255.0,
    ])
}

fn rgb_to_hex(c: Rgb) -> String {
    let q = |x: f64| (x.clamp(0.0, 1.0) * 255.0).round() as u8;
    format!("#{:02X}{:02X}{:02X}", q(c[0]), q(c[1]), q(c[2]))
}

fn lin(c: f64) -> f64 {
    if c <= 0.03928 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}

fn delin(c: f64) -> f64 {
    if c <= 0.0030353 {
        c * 12.92
    } else {
        1.055 * c.powf(1.0 / 2.4) - 0.055
    }
}

fn to_linear(c: Rgb) -> Rgb {
    [lin(c[0]), lin(c[1]), lin(c[2])]
}

fn luminance(l: Rgb) -> f64 {
    0.2126 * l[0] + 0.7152 * l[1] + 0.0722 * l[2]
}

fn simulate(l: Rgb, m: &[[f64; 3]; 3]) -> Rgb {
    [
        (m[0][0] * l[0] + m[0][1] * l[1] + m[0][2] * l[2]).clamp(0.0, 1.0),
        (m[1][0] * l[0] + m[1][1] * l[1] + m[1][2] * l[2]).clamp(0.0, 1.0),
        (m[2][0] * l[0] + m[2][1] * l[1] + m[2][2] * l[2]).clamp(0.0, 1.0),
    ]
}

fn contrast(a: Rgb, b: Rgb) -> f64 {
    let (la, lb) = (luminance(a), luminance(b));
    let (hi, lo) = if la > lb { (la, lb) } else { (lb, la) };
    (hi + 0.05) / (lo + 0.05)
}

// the four ratios a pair must survive
fn ratios(fg: Rgb, bg: Rgb) -> [f64; 4] {
    let (fl, bl) = (to_linear(fg), to_linear(bg));
    [
        contrast(fl, bl),
        contrast(simulate(fl, &PROTANOPIA), simulate(bl, &PROTANOPIA)),
        contrast(simulate(fl, &DEUTERANOPIA), simulate(bl, &DEUTERANOPIA)),
        contrast(simulate(fl, &TRITANOPIA), simulate(bl, &TRITANOPIA)),
    ]
}

const VISIONS: [&str; 4] = ["as-is", "protanopia", "deuteranopia", "tritanopia"];

fn fix(fg: Rgb, bg: Rgb) -> Option<Rgb> {
    let bg_lum = luminance(to_linear(bg));
    // move away from the background's side of the light
    let toward_white = bg_lum < 0.5;
    let target: Rgb = if toward_white { [1.0, 1.0, 1.0] } else { [0.0, 0.0, 0.0] };
    let start = to_linear(fg);
    for step in 1..=100 {
        let t = step as f64 / 100.0;
        let moved: Rgb = [
            start[0] + (target[0] - start[0]) * t,
            start[1] + (target[1] - start[1]) * t,
            start[2] + (target[2] - start[2]) * t,
        ];
        let candidate: Rgb = [delin(moved[0]), delin(moved[1]), delin(moved[2])];
        if ratios(candidate, bg).iter().all(|r| *r >= 4.5) {
            return Some(candidate);
        }
    }
    None
}

fn audit(name: &str, palette: &[(String, Rgb)]) {
    let (bg_name, bg) = &palette[0];
    println!();
    println!("  {name}; background {bg_name} {}", rgb_to_hex(*bg));
    for (fg_name, fg) in &palette[1..] {
        let rs = ratios(*fg, *bg);
        let worst = rs.iter().cloned().fold(f64::INFINITY, f64::min);
        if worst >= 4.5 {
            println!(
                "    {:14} {}  {:.2}; ok in all four visions",
                fg_name,
                rgb_to_hex(*fg),
                worst
            );
        } else {
            let failing: Vec<&str> = rs
                .iter()
                .zip(VISIONS.iter())
                .filter(|(r, _)| **r < 4.5)
                .map(|(_, v)| *v)
                .collect();
            match fix(*fg, *bg) {
                Some(fixed) => println!(
                    "    {:14} {}  {:.2}; fails {}; fix {}",
                    fg_name,
                    rgb_to_hex(*fg),
                    worst,
                    failing.join(", "),
                    rgb_to_hex(fixed)
                ),
                None => println!(
                    "    {:14} {}  {:.2}; fails {}; no luminance fix, the hue needs help",
                    fg_name,
                    rgb_to_hex(*fg),
                    worst,
                    failing.join(", ")
                ),
            }
        }
    }
}

fn selftest() -> bool {
    let black = [0.0, 0.0, 0.0];
    let white = [1.0, 1.0, 1.0];
    let ok = (contrast(to_linear(black), to_linear(white)) - 21.0).abs() < 0.01
        && (contrast(to_linear(white), to_linear(white)) - 1.0).abs() < 1e-9
        && luminance(simulate(to_linear(white), &PROTANOPIA)) > 0.99
        && luminance(simulate(to_linear(white), &TRITANOPIA)) > 0.99;

    // a deliberately weak pair must fail, then pass after the fix
    let bg = hex_to_rgb("0D1A15").unwrap();
    let weak = hex_to_rgb("24382E").unwrap();
    let failed = ratios(weak, bg).iter().any(|r| *r < 4.5);
    let fixed_ok = fix(weak, bg).map(|f| ratios(f, bg).iter().all(|r| *r >= 4.5)) == Some(true);

    ok && failed && fixed_ok
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.iter().any(|a| a == "--selftest") {
        if selftest() {
            println!("  lumen; the checks hold");
            return ExitCode::SUCCESS;
        }
        println!("  lumen; a check broke");
        return ExitCode::FAILURE;
    }

    println!();
    println!("  small machines · lumen; the palette auditor");
    println!("  pairs pass at 4.5 or better, in all four visions");

    if args.is_empty() {
        let builtins: Vec<(&str, Vec<(&str, &str)>)> = vec![
            (
                "spruce and copper",
                vec![
                    ("spruce", "0D1A15"),
                    ("bone", "ECE7DB"),
                    ("moss", "9FB6A7"),
                    ("copper", "C98E5F"),
                ],
            ),
            (
                "ink and verdigris",
                vec![
                    ("ink", "101418"),
                    ("fog", "E8E6E1"),
                    ("verdigris", "7FA8A0"),
                ],
            ),
            (
                "night and periwinkle",
                vec![
                    ("night", "141821"),
                    ("paper", "E6E4EA"),
                    ("periwinkle", "8C93B8"),
                ],
            ),
        ];
        for (name, colors) in builtins {
            let palette: Vec<(String, Rgb)> = colors
                .into_iter()
                .map(|(n, h)| (n.to_string(), hex_to_rgb(h).unwrap()))
                .collect();
            audit(name, &palette);
        }
    } else {
        let mut palette: Vec<(String, Rgb)> = Vec::new();
        for a in &args {
            match hex_to_rgb(a) {
                Some(c) => palette.push((a.to_lowercase(), c)),
                None => {
                    eprintln!("  {} is not a hex color; lumen wants #RRGGBB", a);
                    return ExitCode::FAILURE;
                }
            }
        }
        if palette.len() < 2 {
            eprintln!("  give lumen a background and at least one foreground");
            return ExitCode::FAILURE;
        }
        audit("your palette", &palette);
    }

    ExitCode::SUCCESS
}
