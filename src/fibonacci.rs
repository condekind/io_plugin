use polars::prelude::*;
use pyo3::{pyclass, pyfunction};
use pyo3_polars::export::polars_core::datatypes::DataType;
use pyo3_polars::export::polars_core::prelude::Series;
use std::sync::Mutex;

#[pyclass]
#[derive(Clone)]
pub struct PySeq(pub Arc<Mutex<Box<dyn SeqProducer>>>);

pub trait SeqProducer: Send {
    fn name(&self) -> &str;

    fn dtype(&self) -> DataType;

    fn next_n(&mut self, n: usize) -> Series;
}

struct FibonacciProducer {
    name: String,
    last_two: [i64; 2],
}

fn new_fib_impl(name: String, first: i64, second: i64) -> FibonacciProducer {
    FibonacciProducer {
        name,
        last_two: [first, second],
    }
}

impl SeqProducer for FibonacciProducer {
    fn name(&self) -> &str {
        &self.name
    }

    fn dtype(&self) -> DataType {
        return DataType::Int64;
    }

    fn next_n(&mut self, n: usize) -> Series {
        // Output/result vector
        let mut out = Vec::with_capacity(n);

        // Push the last two numbers stored to the sequence. Without this, the
        // first two numbers supplied (e.g., [1, 1]) wouldn't show in the result
        out.push(self.last_two[0]);
        out.push(self.last_two[1]);

        // Calculate and push the next n-2, since we already pushed 2 above
        for _ in 0..n - 2 {
            let next = self.last_two[0]
                .checked_add(self.last_two[1])
                .unwrap_or(self.last_two[1]);
            out.push(next);
            self.last_two = [self.last_two[1], next];
        }

        // Calculate next two numbers that occupy last_two, for future purposes
        for _ in 0..2 {
            let next = self.last_two[0]
                .checked_add(self.last_two[1])
                .unwrap_or(self.last_two[1]);
            self.last_two = [self.last_two[1], next];
        }

        Series::from_vec(self.name(), out)
    }
}

#[pyfunction]
pub fn new_fibonacci(name: String, first: i64, second: i64) -> PySeq {
    let res = Box::new(new_fib_impl(name, first, second)) as Box<dyn SeqProducer>;
    PySeq(Arc::new(Mutex::new(res)))
}
