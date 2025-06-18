use ode_rs::numerical_methods::RungeKutta4;
use ode_rs::ode::{ODE, ODESolver, solve};

struct ExponentialDecay {
    lambda: f64,
}

impl ODE for ExponentialDecay {
    fn rhs(&self, y: &f64, _t: f64, dydt: &mut f64) {
        *dydt = -self.lambda * y;
    }
}

#[test]
fn test_runge_kutta_4() {
    let lambda = 0.5;
    let y0 = 2.0;
    let t0 = 0.0;
    let t_final = 4.0;
    let dt = 0.01;

    let ode = ExponentialDecay { lambda };

    let solver = RungeKutta4::new(ode, dt);

    let solution = solve(y0, t0, t_final, solver);

    let test_times = [1.0, 2.0, 3.0, 4.0];

    for &t in &test_times {
        let idx = solution
            .t
            .iter()
            .position(|&time| (time - t).abs() < dt / 2.0)
            .expect("Time point not found");

        let numerical = solution.y[idx];
        let analytical = y0 * (-lambda * t).exp();

        let relative_error = (numerical - analytical).abs() / analytical.abs();
        assert!(
            relative_error < 0.01,
            "Error too large at t={}: numerical={}, analytical={}, error={}",
            t,
            numerical,
            analytical,
            relative_error
        );
    }
}
