use plotters::prelude::*;
use rand::Rng;
use std::io;

fn main() {
    loop {
        let mut input = String::new();
        println!("Choose the function (\n
        1 -> for x^2, \n
        2 -> for sqrt(1 - x^2))\n
        3 -> x^2 + 1:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        let choice: i32 = input.trim().parse().expect("Please enter a valid option.");

        let f: Box<dyn Fn(f64) -> f64> = match choice {
            1 => Box::new(|x: f64| x.powi(2)), // Function x^2
            2 => Box::new(|x: f64| (1.0 - x.powi(2)).sqrt()), // Function sqrt(1 - x^2)
            3 => Box::new(|x:f64| x.powi(2) + 1.0),
            _ => panic!("Invalid choice"),
        };

        input.clear();
        println!("How many iterations?");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        let n: usize = input.trim().parse().expect("Please enter a valid number.");

        // Definindo o intervalo e a altura do gráfico
        let (a, b, h) = match choice {
            1 => (0.0, 2.0, 4.0),          // Para f(x) = x^2 no intervalo [0, 2], h = 4
            2 => (0.0, 1.0, 1.0),          // Para f(x) = sqrt(1 - x^2) no intervalo [0, 1], h = 1
            3 => (0.0, 1.0, 2.0),          // Para f(x) = x^2 + 1 no intervalo [0, 1], h = 2
            _ => panic!("Invalid choice"),
        };
        println!(
            "Starting the calculation for {} iterations, for function {}",
            n,
            match choice {
                1 => "x^2",
                2 => "sqrt(1 - x^2)",
                3 => "x^2 + 1",
                _ => "",
            }
        );

        println!("Starting the calculation for {} iterations, to function ", n);
        // Calculando a integral
        let (inside_points, outside_points, integral_approx) = monte_carlo_integration(&*f, a, b, h, n);
        plot_monte_carlo(
            &inside_points,
            &outside_points,
            a,
            b,
            h,
            n,
            integral_approx,
            choice,
        );

        println!("Approximate integral value: {}", integral_approx);

        println!("Do you want to perform another calculation? (yes/no)");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
        if input.trim().eq_ignore_ascii_case("no") {
            break;
        }
    }
}

// Função para calcular a integral de uma função no intervalo [a, b] usando o método de Monte Carlo
fn monte_carlo_integration<F>(
    f: &F,
    a: f64,
    b: f64,
    h: f64,
    n: usize,
) -> (Vec<(f64, f64)>, Vec<(f64, f64)>, f64)
where
    F: Fn(f64) -> f64 + ?Sized,
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


// Função para plotar o gráfico
fn plot_monte_carlo(
    inside_points: &[(f64, f64)],
    outside_points: &[(f64, f64)],
    a: f64,
    b: f64,
    h: f64,
    n: usize,
    integral_approx: f64,
    choice: i32,
) {
    let name = format!(
        "monte_carlo_integral_function_{}_{}.png",
        match choice {
            1 => "x²",
            2 => "sqrt(1 - x²)",
            3 => "x² + 1",
            _ => "unknown",
        },
        n
    );
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
