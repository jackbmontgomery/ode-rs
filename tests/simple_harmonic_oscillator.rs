use nalgebra::SVector;
use ode_rs::numerical_methods::RungeKutta4;
use ode_rs::ode::{ODE, ODESolver, solve};

struct SimpleHarmonicOscillator;

impl ODE<f64, SVector<f64, 2>> for SimpleHarmonicOscillator {
    fn rhs(&self, y: &SVector<f64, 2>, _t: f64, dydt: &mut SVector<f64, 2>) {
        dydt[0] = y[1];
        dydt[1] = -y[0];
    }
}

#[test]
fn test_runge_kutta_4_vector() {
    let ode = SimpleHarmonicOscillator;
    let dt = 0.01;
    let t0 = 0.0;
    let t_final = 4.0;

    let y0 = SVector::<f64, 2>::new(1.0, 0.0);

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
        let analytical = SVector::<f64, 2>::new(t.cos(), -t.sin());

        let error = (numerical - analytical).norm() / analytical.norm();
        assert!(
            error < 0.01,
            "Error too large at t={}: numerical={:?}, analytical={:?}, error={}",
            t,
            numerical,
            analytical,
            error
        );
    }
}
