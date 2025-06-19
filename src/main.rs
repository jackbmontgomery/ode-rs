use nalgebra::SVector;
use ode_rs::numerical_methods::RungeKutta4;
use ode_rs::ode::{ODE, ODESolver, solve_ivp_batch};

struct Lorenz {
    sigma: f64,
    beta: f64,
    rho: f64,
}

impl ODE<f64, SVector<f64, 3>> for Lorenz {
    fn rhs(&self, y: &SVector<f64, 3>, _t: f64, dydt: &mut SVector<f64, 3>) {
        dydt[0] = self.sigma * (y[1] - y[0]);
        dydt[1] = y[0] * (self.rho - y[2]) - y[1];
        dydt[2] = y[0] * y[1] - self.beta * y[2];
    }
}

fn main() {
    let y0s: Vec<SVector<f64, 3>> = vec![
        SVector::<f64, 3>::new(10.0, 10.0, 10.0),
        SVector::<f64, 3>::new(10.1, 10.0, 10.0),
        SVector::<f64, 3>::new(10.0, 10.1, 10.0),
    ];

    let t0 = 0.0;
    let tf = 100.0;
    let dt = 0.01;

    let ode = Lorenz {
        sigma: 10.,
        beta: 8. / 3.,
        rho: 28.,
    };

    let solver = RungeKutta4::new(&ode, t0, tf, dt);

    let solutions = solve_ivp_batch(y0s, solver);

    for (i, solution) in solutions.iter().enumerate() {
        let filename = format!("./plotting/solution_{}.csv", i);
        let _ = solution.write_to_csv(&filename);
    }
}
