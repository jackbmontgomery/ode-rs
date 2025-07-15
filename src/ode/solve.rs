use crate::ode::{ODE, ODESolution, ODESolver_};
use crate::state::{Real, State};
use rayon::prelude::*;

pub fn solve_ivp<'a, T, V, O, S>(y0: V, mut solver: S) -> ODESolution<T, V>
where
    T: Real,
    V: State<T>,
    O: ODE<T, V>,
    S: ODESolver_<'a, T, V, O>,
{
    let mut solution = ODESolution::new();

    let t0 = solver.get_t0();
    let tf = solver.get_tf();

    solution.push(&y0, &t0);

    let mut t = t0;
    let mut y = y0;

    let mut stopping_flag = false;

    while !stopping_flag {
        solver.step(&mut y, &mut t);
        solution.push(&y, &t);

        stopping_flag = !(t < tf)
    }

    solution
}

pub fn solve_ivp_batch<'a, T, V, O, S>(y0s: Vec<V>, solver: S) -> Vec<ODESolution<T, V>>
where
    T: Real,
    V: State<T>,
    O: ODE<T, V>,
    S: ODESolver_<'a, T, V, O> + Send + Sync,
{
    let solutions: Vec<ODESolution<T, V>> = y0s
        .into_par_iter()
        .map(move |y0| {
            let cloned_solver = solver.clone();
            solve_ivp(y0, cloned_solver)
        })
        .collect();

    solutions
}
