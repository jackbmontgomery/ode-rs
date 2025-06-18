use nalgebra::{SVector, vector};
use ode_rs::numerical_methods::RungeKutta4;
use ode_rs::ode::{ODE, ODESolver, solve};

struct LorenzSystem {
    sigma: f64,
    beta: f64,
    rho: f64,
}

impl ODE<f64, SVector<f64, 3>> for LorenzSystem {
    fn rhs(&self, y: &SVector<f64, 3>, _t: f64, dydt: &mut SVector<f64, 3>) {
        dydt[0] = self.sigma * (y[1] - y[0]);
        dydt[1] = y[0] * (self.rho - y[2]) - y[1];
        dydt[2] = y[0] * y[1] - self.beta * y[2];
    }
}

struct CoolingSystem {
    k: f64,
    t_ambient: f64,
}

impl ODE<f64, f64> for CoolingSystem {
    fn rhs(&self, y: &f64, _t: f64, dydt: &mut f64) {
        *dydt = -self.k * (y - self.t_ambient);
    }
}

fn main() {
    // let ode = LorenzSystem {
    //     sigma: 10.0,
    //     beta: 8.0 / 3.0,
    //     rho: 28.0,
    // };
    // let y0 = vector![10.0, 10.0, 10.0];

    let ode = CoolingSystem {
        k: 0.1,
        t_ambient: 20.0,
    };
    let y0 = 100.0;

    let dt = 0.01;
    let t0 = 0.;
    let t_final = 100.;

    let solver = RungeKutta4::new(ode, dt);

    let solution = solve(y0, t0, t_final, solver);

    let _result = solution.write_to_csv("./plotting/trajectory_cooling.csv");
}
