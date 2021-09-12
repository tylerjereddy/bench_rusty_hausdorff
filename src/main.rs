#[macro_use]
extern crate timeit;
use ndarray::Array2;
use ndarray_npy::NpzReader;
use rusty_hausdorff::directed_hausdorff;
use std::fs::File;
use std::sync::Arc;

fn main() {
    // load the same `.npz` data as used for the
    // SciPy directed_hausdorff() benchmarks
    let mut npz =
		NpzReader::new(File::open("/Users/treddy/rough_work/scipy/data_files_for_rust_hausdorff_compare/compare_hausdorff_data.npz").expect("unable to open data file")).expect("unable to read from file");
    let arr_1_vlarge: Array2<f64> = npz
        .by_name("arr_1_vlarge.npy")
        .expect("unable to retrieve arr_1_vlarge field");
    let arr_1_large: Array2<f64> = npz
        .by_name("arr_1_large.npy")
        .expect("unable to retrieve arr_1_large field");
    let arr_1_medium: Array2<f64> = npz
        .by_name("arr_1_medium.npy")
        .expect("unable to retrieve arr_1_medium field");
    let arr_1_small: Array2<f64> = npz
        .by_name("arr_1_small.npy")
        .expect("unable to retrieve arr_1_small field");
    let arr_2_small: Array2<f64> = npz
        .by_name("arr_2_small.npy")
        .expect("unable to retrieve arr_2_small field");
    let workers: Vec<usize> = vec![1, 2, 4, 8, 16];
    let arr_1_options = vec![arr_1_vlarge, arr_1_large, arr_1_medium, arr_1_small];
    let arr_2_small = Arc::new(arr_2_small);
    for arr_1 in arr_1_options {
        let arr_1_val = Arc::new(arr_1);
        for worker_count in &workers {
            println!("worker_count: {}", worker_count);
            timeit!({
                directed_hausdorff(arr_1_val.clone(), arr_2_small.clone(), *worker_count);
            });
        }
    }
}
