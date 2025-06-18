// use ode_rs::numerical_methods::RungeKutta4;
// use ode_rs::ode::{ODE, ODESolver, solve};
//
// struct ExponentialDecay {
//     lambda: f64,
// }
//
// impl ODE<f64, f64> for ExponentialDecay {
//     fn rhs(&self, y: &f64, _t: f64, dydt: &mut f64) {
//         *dydt = -self.lambda * y;
//     }
// }
fn main() {
    // let lambda = 0.5;
    // let mut y0 = 2.0;
    // let mut t0 = 0.0;
    // let _t_final = 4.0;
    // let dt = 0.01;
    //
    // let ode = ExponentialDecay { lambda };
    //
    // let mut solver = RungeKutta4::new(ode, dt);
    //
    // solver.step(&mut y0, &mut t0);
    //
    // println!("{:?}", y0);
    // let solution = solve(y0, t0, t_final, solver);
    // let _ = solution.write_to_csv("./plotting/decay_trajectory.csv");
}
