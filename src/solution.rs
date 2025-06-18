use crate::ode::ODE;
use crate::state::{Real, State};
use crate::solvers::ODESolver;

pub fn solve_ode<S, O, V, T>(solver: &mut S, system: &O) -> Vec<V>
where
    T: Real,
    V: State<T>,
    O: ODE<T, V>,
    S: ODESolver<O, T, V>
{
    for 
}
