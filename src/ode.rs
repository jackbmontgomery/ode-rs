use crate::state::State;

pub trait ODE<T, S: State<T>> {
    fn rhs(&self, t: f64, y: &S, dydt: &mut S);
}
