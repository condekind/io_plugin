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

    fn next_n(&mut self, n: usize) -> Vec<Series>;
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

    fn next_n(&mut self, n: usize) -> Vec<Series> {
        // Output/result vector
        let mut out: Vec<Vec<String>> = Vec::new();

        let path = Path::new(&self.file_path);

        let mut cnt = 0usize;

        match File::open(path) {
            Ok(file) => {
                let reader = io::BufReader::new(file);
                for line in reader.lines() {

                    // On zero we initialize `out`, on `n` we break
                    if cnt == 0usize {
                        // Split first line to decide how many Vecs `out` will contain
                        let split_line = match &line {
                            Ok(l) => l.split(',').collect::<Vec<&str>>(),
                            Err(e) => {
                                println!("Error reading line #0: {}", e);
                                Vec::new()
                            },
                        };
                        for _ in 0usize..split_line.len() {
                            out.push(Vec::new());
                        };
                    } else if cnt == n {
                        // If we already read n lines, stop
                        break;
                    }

                    match line {
                        Ok(line) => {
                            let fields = line.split(',').collect::<Vec<&str>>();
                            if fields.len() != out.len() {
                                println!("(WW): Line #{cnt} doesn't have the same num. of fields than line #0");
                                let mut fill = fields.len();
                                while fill < out.len() {
                                    out[fill].push(String::from("MISSING"));
                                    fill += 1;
                                }
                            }

                            for idx in 0usize..fields.len() {

                                if idx >= out.len() {
                                    println!("(WW): Line #{cnt} has more fields than line #0");
                                    break;
                                }

                                // Push idx-th line to idx-th Vec in `out`
                                out[idx].push(fields[idx].parse().unwrap());
                            }

                            cnt += 1;
                        }
                        Err(e) => {
                            eprintln!("Error reading line: {}", e);
                            cnt += 1;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error opening file: {}", e);
            }
        }

        let mut res: Vec<Series> = Vec::new();
        for (idx, v) in out.iter().enumerate() {
            res.push(Series::new(idx.to_string().as_str().into(), v));
        }
        res
    }
}

#[pyfunction]
pub fn new_reader(name: String, filename: String) -> PyReader {
    let res = Box::new(new_line_reader_impl(name, filename)) as Box<dyn FileReader>;
    PyReader(Arc::new(Mutex::new(res)))
}
