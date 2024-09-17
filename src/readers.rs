use polars::prelude::*;
use pyo3::{pyclass, pyfunction};
use pyo3_polars::export::polars_core::datatypes::DataType;
use pyo3_polars::export::polars_core::prelude::Series;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::sync::Mutex;

#[pyclass]
#[derive(Clone)]
pub struct PyReader(pub Arc<Mutex<Box<dyn FileReader>>>);

pub trait FileReader: Send {
    fn name(&self) -> &str;

    fn dtype(&self) -> DataType;

    fn next_n(&mut self, n: usize) -> Series;
}

struct LineReader {
    name: String,
    file_path: String,
}

fn new_line_reader_impl(name: String, file_path: String) -> LineReader {
    LineReader {
        name,
        file_path: file_path,
    }
}

impl FileReader for LineReader {
    fn name(&self) -> &str {
        &self.name
    }

    fn dtype(&self) -> DataType {
        return DataType::String;
    }

    fn next_n(&mut self, n: usize) -> Series {
        // Output/result vector
        let mut out: Vec<String> = Vec::with_capacity(n);

        let path = Path::new(&self.file_path);

        let mut cnt = 0usize;

        match File::open(path) {
            Ok(file) => {
                let reader = io::BufReader::new(file);
                for line in reader.lines() {
                    if cnt == n {
                        break
                    }
                    match line {
                        Ok(line) => {
                            let line = line.trim().to_string();
                            out.push(line);
                            cnt += 1;
                        }
                        Err(e) => {
                            eprintln!("Error reading line: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error opening file: {}", e);
            }
        }

        Series::new(self.name().into(), out)
    }
}

#[pyfunction]
pub fn new_reader(name: String, filename: String) -> PyReader {
    let res = Box::new(new_line_reader_impl(name, filename)) as Box<dyn FileReader>;
    PyReader(Arc::new(Mutex::new(res)))
}
