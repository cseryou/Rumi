use colored::Colorize;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
//#d8a3a8 #d47e86 #f57867 #bed6e2 #e6ebea #d5d7d6 #e1e0e4
fn main() {
    colorize_re();
    print!("{}", "Rumi".bold());
    colorize_wh();
    println!("ㅣscientific calculator");

    colorize_re();
    print!("{}", "한 근".bold());
    colorize_wh();

    print_cubic(1.0, 0.0, 0.0, -27.0)
}

fn print_cubic (a:f32, b:f32, c:f32, d:f32) {
    let ans = cubic(a, b, c, d);
    println!(": {}, {}", ans[0], ans[1])
}

fn colorize_re() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color:: Rgb(0xf5, 0x78, 0x67)))).unwrap();
}

fn colorize_wh() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(0xd5, 0xd7, 0xd6)))).unwrap();
}

fn cubic(a:f32, b:f32, c:f32, d:f32) -> [String;2] {
    let p = -2.0 * f32::powf(b, 3.0) + 9.0*a*b*c - 27.0 * f32::powf(a, 2.0) * d;
    let q = f32::powf(-1.0 * b*b + 3.0*a*c, 3.0);
    let m_q = -1.0*b*b + 3.0*a*c; 

    let f_numerator = f32::powf(p + f32::powf(4.0*q + p*p, 0.5), 1.0/3.0);
    let f_denominator = 3.0 * f32::powf(2.0, 1.0/3.0) * a;

    let f2_numerator = f32::powf(2.0, 1.0 / 3.0) * m_q;
    let f2_denominator = 3.0*a * f32::powf(p + f32::powf(4.0*q + p*p, 0.5), 1.0 / 3.0);

    let first = f_numerator / f_denominator - f2_numerator / f2_denominator - b/3.0*a;

    let A = a;
    let B = b + 3.0 * first * A;
    let C = c + 3.0 * a * first * first;

    let mut sec_thr: String = String::from("asfd");
    if ((-pretty_number(B) + f32::powf(f32::powf(B, 2.0) - 3.0*A*C, 0.5)) / 3.0*A).fract() == 0.0 {
        sec_thr = format!("{}", (-pretty_number(B) + f32::powf(f32::powf(B, 2.0) - 3.0*A*C, 0.5)) / 3.0*A);
    }
    else if f32::powf(f32::powf(B, 2.0) - 3.0*A*C,0.5).fract() == 0.0 {
        sec_thr = format!("{} / {}", -pretty_number(B) + f32::powf(f32::powf(B, 2.0) - 3.0*A*C, 0.5), 3.0*A);
    } else {
        sec_thr = format!("({} + √{}) / {}", -pretty_number(B), (f32::powf(B, 2.0) - 3.0*A*C), 3.0*A);
    };

    return [format!("{}", pretty_number(first)), format!("{}", sec_thr)]; 
}

fn pretty_number(p: f32) -> f32 {
    let prettier = (p * 100.0).round() / 100.0;
    prettier
}