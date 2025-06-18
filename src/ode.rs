use std::fs::File;
use std::io::{BufWriter, Write};
use std::marker::PhantomData;
use std::time::Instant;

use crate::state::{Real, State};

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

pub struct ODESolution<T, V>
where
    T: Real,
    V: State<T>,
{
    pub y: Vec<V>,
    pub t: Vec<f64>,
    duration_s: Option<f64>,
    _phantom: PhantomData<T>,
}

impl<T, V> ODESolution<T, V>
where
    T: Real,
    V: State<T>,
{
    fn new() -> Self {
        ODESolution {
            y: Vec::<V>::new(),
            t: Vec::<f64>::new(),
            _phantom: PhantomData,
            duration_s: None,
        }
    }

    fn push(&mut self, y: &V, t: &f64) {
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

    pub fn get_solution_duration(&self) -> Option<f64> {
        self.duration_s
    }
}

pub fn solve<T, V, O, S>(y0: V, t0: f64, final_t: f64, mut solver: S) -> ODESolution<T, V>
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
