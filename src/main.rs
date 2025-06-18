use nalgebra::{SVector, vector};
use ode_rs::ode::ODE;
use ode_rs::solution::solve_ode;
use ode_rs::solvers::{ODESolver, RK4};

struct LorenzSystem {
    sigma: f64,
    beta: f64,
    rho: f64,
}

impl ODE<f64, SVector<f64, 3>> for LorenzSystem {
    fn rhs(&self, _t: f64, y: &SVector<f64, 3>, dydt: &mut SVector<f64, 3>) {
        dydt[0] = self.sigma * (y[1] - y[0]);
        dydt[1] = y[0] * (self.rho - y[2]) - y[1];
        dydt[2] = y[0] * y[1] - self.beta * y[2];
    }
}

fn main() {
    let ode = LorenzSystem {
        sigma: 10.,
        beta: 8. / 3.,
        rho: 28.,
    };

    let t = 0.01;
    let dt = 0.1;

    let mut solver = RK4::new(&ode, dt);
    let mut y = vector![10., 10., 10.];

    solver.step(t, &mut y);

    println!("{y:?}");
    solve_ode();

    // trajectory.push(y);
    //
    // let y = solver.step(0.1, y, &ode);
    // trajectory.push(y);
    //
    // println!("{trajectory:?}");
}
