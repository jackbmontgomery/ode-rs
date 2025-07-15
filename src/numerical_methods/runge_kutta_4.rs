use crate::ode::{ODE, ODESolver_};
use crate::state::{Real, State};
use std::marker::PhantomData;

pub struct RungeKutta4<'a, T, V, O>
where
    T: Real,
    V: State<T>,
    O: ODE<T, V>,
{
    pub k1: V,
    pub k2: V,
    pub k3: V,
    pub k4: V,
    pub y_temp: V,
    pub t0: f64,
    pub tf: f64,
    pub dt: f64,
    pub ode: &'a O,
    _phantom: PhantomData<T>,
}

impl<'a, T, V, O> ODESolver_<'a, T, V, O> for RungeKutta4<'a, T, V, O>
where
    T: Real,
    V: State<T>,
    O: ODE<T, V>,
{
    fn init(ode: &'a O, t0: f64, tf: f64, dt: f64) -> Self {
        RungeKutta4 {
            k1: V::zeros(),
            k2: V::zeros(),
            k3: V::zeros(),
            k4: V::zeros(),
            y_temp: V::zeros(),
            dt,
            t0,
            tf,
            ode,
            _phantom: PhantomData,
        }
    }

    fn step(&mut self, y: &mut V, t: &mut f64) {
        let dt_t = T::from_f64(self.dt).unwrap();
        let half_t = T::from_f64(1. / 2.).unwrap();
        let sixth_t = T::from_f64(1. / 6.).unwrap();
        let two_t = T::from_f64(2.).unwrap();

        self.ode.rhs(y, *t, &mut self.k1);

        self.y_temp = *y + self.k1 * dt_t * half_t;
        self.ode.rhs(&self.y_temp, *t + self.dt / 2., &mut self.k2);

        self.y_temp = *y + self.k2 * dt_t * half_t;
        self.ode.rhs(&self.y_temp, *t + self.dt / 2., &mut self.k3);

        self.y_temp = *y + self.k3 * dt_t;
        self.ode.rhs(&self.y_temp, *t + self.dt, &mut self.k4);

        *y += (self.k1 + self.k2 * two_t + self.k3 * two_t + self.k4) * dt_t * sixth_t;
        *t += self.dt;
    }

    fn clone(&self) -> Self {
        RungeKutta4 {
            k1: V::zeros(),
            k2: V::zeros(),
            k3: V::zeros(),
            k4: V::zeros(),
            y_temp: V::zeros(),
            dt: self.dt,
            t0: self.t0,
            tf: self.tf,
            ode: self.ode,
            _phantom: PhantomData,
        }
    }

    fn get_t0(&self) -> f64 {
        self.t0
    }

    fn get_tf(&self) -> f64 {
        self.tf
    }
}
