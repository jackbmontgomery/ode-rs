use crate::ode::{ODE, ODESolver};
use crate::state::{Real, State};
use std::marker::PhantomData;

pub struct RungeKutta4<T, V, O>
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
    pub dt: f64,
    pub ode: O,
    _phantom: PhantomData<T>,
}

impl<T, V, O> ODESolver<T, V, O> for RungeKutta4<T, V, O>
where
    T: Real,
    V: State<T>,
    O: ODE<T, V>,
{
    fn new(ode: O, dt: f64) -> Self {
        RungeKutta4 {
            k1: V::zeros(),
            k2: V::zeros(),
            k3: V::zeros(),
            k4: V::zeros(),
            y_temp: V::zeros(),
            dt,
            ode,
            _phantom: PhantomData,
        }
    }
    fn step(&mut self, y: &mut V, t: &mut f64) {
        let dt_t = T::from_f64(self.dt).unwrap();
        let half_t = T::from_f64(1. / 2.).unwrap();
        let sixth_t = T::from_f64(1. / 6.).unwrap();
        let two = T::from_f64(2.).unwrap();

        self.ode.rhs(y, *t, &mut self.k1);
        println!("{:?}", self.k1);

        self.y_temp = *y + self.k1 * dt_t * half_t;
        self.ode.rhs(&self.y_temp, *t + self.dt / 2., &mut self.k2);
        println!("{:?}", self.k2);

        self.y_temp = *y + self.k2 * dt_t * half_t;
        self.ode.rhs(&self.y_temp, *t + self.dt / 2., &mut self.k3);
        println!("{:?}", self.k3);

        self.y_temp = *y + self.k3 * dt_t;
        self.ode.rhs(&self.y_temp, *t + self.dt, &mut self.k4);
        println!("{:?}", self.k4);

        *y += (self.k1 + self.k2 * two + self.k3 * two + self.k4) * dt_t * sixth_t;
        *t += self.dt;
    }
}
