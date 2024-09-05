use std::io;
use plotters::prelude::*;
use rand::Rng;

fn main() {
    let mut input = String::new();
    println!("How many iterations?");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    let n: usize = input.trim().parse().expect("Please enter a valid number.");

    let f = |x: f64| x.powi(2); //Example function xˆ2
    // let f = |x: f64| (1.0 - x.powi(2)).sqrt();
    let a = 0.0;
    let b = 2.0;
    let h = 4.0;

    println!("Starting the calculation for {} iterations, to function ", n,);

    let (inside_points, outside_points, integral_approx) = monte_carlo_integration(f, a, b, h, n);
    plot_monte_carlo(
        &inside_points,
        &outside_points,
        a,
        b,
        h,
        n,
        integral_approx,
    );

    println!("Approximate integral value: {}", integral_approx);
}

fn monte_carlo_integration<F>(
    f: F,
    a: f64,
    b: f64,
    h: f64,
    n: usize,
) -> (Vec<(f64, f64)>, Vec<(f64, f64)>, f64)
where
    F: Fn(f64) -> f64,
{
    let mut rng = rand::thread_rng();
    let mut inside_points = Vec::new();
    let mut outside_points = Vec::new();
    let mut count_under_curve = 0;

    for _ in 0..n {
        let x = rng.gen_range(a..b);
        let y = rng.gen_range(0.0..h);
        if y <= f(x) {
            inside_points.push((x, y));
            count_under_curve += 1;
        } else {
            outside_points.push((x, y));
        }
    }

    let area = (b - a) * h;
    let integral_approx = (count_under_curve as f64 / n as f64) * area;

    (inside_points, outside_points, integral_approx)
}

fn plot_monte_carlo(
    inside_points: &[(f64, f64)],
    outside_points: &[(f64, f64)],
    a: f64,
    b: f64,
    h: f64,
    n: usize,
    integral_approx: f64,
) {
    // let name = format!("monte_carlo_integral_function:√(1-xˆ2)_{}.png", n);
    let name = format!("monte_carlo_integral_function:xˆ2_{}.png", n);
    let root = BitMapBackend::new(&name, (600, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let text = format!("Monte Carlo Integration: Integral ≈ {:.5}", integral_approx);
    let mut chart = ChartBuilder::on(&root)
        .margin(10)
        .caption(text, ("Arial", 20))
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(a..b, 0f64..h)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(
            inside_points
                .iter()
                .map(|&(x, y)| Circle::new((x, y), 2, Into::<ShapeStyle>::into(&GREEN).filled())),
        )
        .unwrap();

    chart
        .draw_series(
            outside_points
                .iter()
                .map(|&(x, y)| Circle::new((x, y), 2, Into::<ShapeStyle>::into(&BLUE).filled())),
        )
        .unwrap();

    root.present().unwrap();
}
