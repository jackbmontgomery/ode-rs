use crate::ode::ODE;
use crate::state::{Real, State};
use std::marker::PhantomData;

pub trait ODESolver<'a, O, T, S>
where
    T: Real,
    S: State<T>,
    O: ODE<T, S>,
{
    fn new(ode: &'a O, dt: f64) -> Self;
    fn step(&mut self, t: f64, y: &mut S);
}

pub struct RK4<'a, O, T, S>
where
    T: Real,
    S: State<T>,
    O: ODE<T, S>,
{
    pub k1: S,
    pub k2: S,
    pub k3: S,
    pub k4: S,
    pub y_temp: S,
    pub system: &'a O,
    pub dt: f64,
    _phantom: PhantomData<T>,
}

impl<'a, O, T, S> ODESolver<'a, O, T, S> for RK4<'a, O, T, S>
where
    T: Real,
    S: State<T>,
    O: ODE<T, S>,
{
    fn new(ode: &'a O, dt: f64) -> Self {
        RK4 {
            k1: S::zeros(),
            k2: S::zeros(),
            k3: S::zeros(),
            k4: S::zeros(),
            y_temp: S::zeros(),
            system: ode,
            dt,
            _phantom: std::marker::PhantomData,
        }
    }

    fn step(&mut self, t: f64, y: &mut S) {
        self.system.rhs(t, y, &mut self.k1);

        let dt_t = T::from_f64(self.dt).unwrap();
        let half_t = T::from_f64(1. / 2.).unwrap();
        let sixth_t = T::from_f64(1. / 6.).unwrap();

        self.y_temp = *y + self.k1 * dt_t * half_t;
        self.system
            .rhs(t + self.dt / 2., &self.y_temp, &mut self.k2);

        self.y_temp = *y + self.k2 * dt_t * half_t;
        self.system
            .rhs(t + self.dt / 2., &self.y_temp, &mut self.k3);

        self.y_temp = *y + self.k3 * dt_t;
        self.system.rhs(t + self.dt, &self.y_temp, &mut self.k3);

        *y += (self.k1 + self.k2 + self.k3 + self.k4) * dt_t * sixth_t;
    }
}
