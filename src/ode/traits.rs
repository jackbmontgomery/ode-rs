use crate::state::{Real, State};
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::marker::PhantomData;

pub trait ODE<T = f64, V = f64>
where
    T: Real,
    V: State<T>,
{
    fn rhs(&self, y: &V, t: f64, dydt: &mut V);
}

pub trait ODESolver<T, V, O>
where
    T: Real,
    V: State<T>,
    O: ODE<T, V>,
{
    fn new(ode: O, dt: f64) -> Self;
    fn step(&mut self, y: &mut V, t: &mut f64);
}

#[derive(Debug)]
pub struct ODESolution<T, V>
where
    T: Real,
    V: State<T> + Debug,
{
    pub y: Vec<V>,
    pub t: Vec<f64>,
    pub duration_s: Option<f64>,
    _phantom: PhantomData<T>,
}

impl<T, V> ODESolution<T, V>
where
    T: Real,
    V: State<T>,
{
    pub fn new() -> Self {
        ODESolution {
            y: Vec::<V>::new(),
            t: Vec::<f64>::new(),
            _phantom: PhantomData,
            duration_s: None,
        }
    }

    pub fn push(&mut self, y: &V, t: &f64) {
        self.y.push(*y);
        self.t.push(*t);
    }

    pub fn write_to_csv(&self, path: &str) -> std::io::Result<()> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        for (t, y) in self.t.iter().zip(self.y.iter()) {
            writeln!(writer, "{},{}", t, y.flat())?;
        }

        Ok(())
    }
}
