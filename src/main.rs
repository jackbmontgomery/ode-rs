use nalgebra::SVector;
use ode_rs::numerical_methods::RungeKutta4;
use ode_rs::ode::{ODE, ODESolution, ODESolver, solve_ode};
use rayon::prelude::*;

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
    let initial_conditions: Vec<SVector<f64, 3>> = vec![
        SVector::<f64, 3>::new(10.0, 10.0, 10.0),
        SVector::<f64, 3>::new(10.1, 10.0, 10.0),
        SVector::<f64, 3>::new(10.0, 10.1, 10.0),
    ];
    // let y0 = SVector::<f64, 3>::new(10.0, 10.0, 10.0);
    // let t0 = 0.0;
    // let t_final = 100.0;
    // let dt = 0.01;
    //
    // let ode = Lorenz {
    //     sigma: 10.,
    //     beta: 8. / 3.,
    //     rho: 28.,
    // };
    //
    // let solver = RungeKutta4::new(ode, dt);
    //
    // let solution = solve(y0, t0, t_final, solver);
    //
    // println!("{:?}", solution.get_solution_duration());
    //

    let result: Vec<ODESolution<f64, SVector<f64, 3>>> = initial_conditions
        .into_par_iter()
        .map(|y0| {
            println!("{:?}", y0);
            let t0 = 0.0;
            let t_final = 1.0;
            let dt = 0.01;

            let ode = Lorenz {
                sigma: 10.,
                beta: 8. / 3.,
                rho: 28.,
            };

            let solver = RungeKutta4::new(ode, dt);

            solve_ode(y0, t0, t_final, solver)
        })
        .collect();

    println!("{:?}", result);
    // println!("{result:?}");
}
