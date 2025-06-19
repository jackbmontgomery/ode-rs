use crate::ode::{ODE, ODESolution, ODESolver};
use crate::state::{Real, State};
use std::time::Instant;

pub fn solve_ode<T, V, O, S>(y0: V, t0: f64, final_t: f64, mut solver: S) -> ODESolution<T, V>
where
    T: Real,
    V: State<T>,
    O: ODE<T, V>,
    S: ODESolver<T, V, O>,
{
    let mut solution = ODESolution::new();
    solution.push(&y0, &t0);

    let mut t = t0;
    let mut y = y0;

    let mut stopping_flag = false;

    let start = Instant::now();

    while !stopping_flag {
        solver.step(&mut y, &mut t);
        solution.push(&y, &t);

        stopping_flag = !(t < final_t)
    }

    let end = Instant::now();
    let duration = end.duration_since(start).as_secs_f64();
    solution.duration_s = Some(duration);

    solution
}
