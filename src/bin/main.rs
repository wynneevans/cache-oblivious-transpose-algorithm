extern crate rand;

use std::io::Write;
use rand::Rng;
use std::env;
use std::time::Instant;
use std::fs::File;

fn transpose(matrix_a: &mut Vec<f32>) -> Vec<f32> {
    let matrix_n = (matrix_a.len() as f32).sqrt() as usize;
    let mut matrix_b: Vec<f32> = vec![0.0;matrix_n*matrix_n];
    for i in 0..matrix_n {
        for j in 0..matrix_n {
            matrix_b[i+j*matrix_n] = matrix_a[j+i*matrix_n];
        }
    }
    matrix_b
}

fn print_matrix(matrix: &mut Vec<f32>) {
    let n = (matrix.len() as f32).sqrt() as usize;
    for i in 0..n {
        for j in 0..n {
            print!("{:.3} ",matrix[i+j*n]);
        }
        println!("");
    }
}

pub fn cache_oblivious_transpose(n: usize, rb: usize, re: usize, cb: usize, ce: usize, mut a: &mut Vec<f32>, mut b: &mut Vec<f32>) {
    let r = re - rb;
    let c = ce - cb;
    if r <= 16 && c <= 16 {
        for j in rb..re{
            for i in cb..ce {
                b[j * n + i] = a[i * n + j];
            }
        }
    } else if r >= c {
        cache_oblivious_transpose(n, rb, rb + (r / 2), cb, ce, &mut a, &mut b);
        cache_oblivious_transpose(n, rb + (r / 2), re, cb, ce, &mut a, &mut b);
    } else {
        cache_oblivious_transpose(n, rb, re, cb, cb + (c / 2), &mut a, &mut b);
        cache_oblivious_transpose(n, rb, re, cb + (c / 2), ce, &mut a, &mut b);
    }
}

fn main() {
    let mut file = File::create("transpose_comparison.txt").unwrap();
    //let mut writer = BufWriter::new(&write_file);
    
    //let n = env::args().nth(1).unwrap().parse::<usize>().unwrap();

    for k in 1..10 {
        let n = k*100;
        
        let mut rng = rand::thread_rng();
        let mut a: Vec<f32> = vec![0.0;n*n]; //let mut a: [f32; 20] = [1.0;20];
        let mut b: Vec<f32> = vec![0.0;n*n];
        
        
        
        for i in a.iter_mut() {
            *i = rng.gen();
        }
        
        let start = Instant::now();
        let c = transpose(&mut a);
        let finish = start.elapsed().subsec_nanos();
        
        let begin = Instant::now();
        cache_oblivious_transpose(n, 0, n, 0, n, &mut a, &mut b);
        let mut end = begin.elapsed().subsec_nanos();
        write!(&mut file , "{}  {:.3}  {:.3}\n", n, finish, end);
    }
    
    
    
}
