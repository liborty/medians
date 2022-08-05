#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]
use devtimer::{DevTime,SimpleTimer};
use medians::Median;
use medians::algos::{naive_median,w_median,r_median};
use indxvec::printing::*;
use ran::{*,generators::*};
use indxvec::{ here, F64, printing::*, Indices, Printing, Vecops, Mutops};
use ran::*;
use std::convert::From;
use times::{benchu8,benchu64,benchf64};

const NAMES:[&str;3] = [ "w_median","r_median","median" ];

const CLOSURESU8:[fn(&[u8]);3] = [
    |v:&[_]| { w_median(v); }, 
    |v:&[_]| { r_median(v); }, 
    |v:&[_]| { v.median(); } ];

/// used to measure errors
fn balance<T>(s:&[T],x:f64) -> i64 where T: Copy,f64:From<T> {
    let mut bal = 0_i64;
    for &si in s { 
        let d = f64::from(si)-x;
        bal += d.signum() as i64;
    }
    bal
}
#[test]
fn comparison() {
    set_seeds(7777777777_u64);   // intialise random numbers generator
    // Rnum encapsulates the type of the data items
   benchu8(Rnum::newu8(),5,10,&NAMES,&CLOSURESU8); 
}

#[test]
fn magnitudes() { 
    let n = 10_usize; // number of vectors to test for each magnitude
    set_seeds(7777777777_u64);   // intialise random numbers generator
    for d in [10,100,1000,10000,100000] {
        let mut n_error = 0_i64;
        let mut w_error = 0_i64; 
        let mut r_error = 0_i64;
        let mut h_error = 0_i64;      
        
        println!("\nTesting even medians on a set of {GR}{}{UN} random vectors of length {GR}{}{UN} each",n,d);
        for _ in 0..n {
            let v = ranvf64_xoshi(d); // random vector 
            // println!("{}",v.medinfo());  
            let mut vm = vec![0f64;d];
            vm.clone_from(&v);

            let n_med = naive_median(&mut vm);
            n_error += balance(&v,n_med).abs();

            let w_med = w_median(&v);
            w_error += balance(&v,w_med).abs();          
      
            let r_med = r_median(&v);
            r_error += balance(&v,r_med).abs();

            let med = v.as_slice().median(); 
            h_error += balance(&v,med).abs();          
            // println!("Even Medians: {:9.6} {:9.6} {:9.6}",n_med,w_med,r_med);
        };
        println!("and odd medians on a set of {GR}{}{UN} random vectors of length {GR}{}{UN} each",n,d+1);
        for _ in 0..n {
            let v = ranvf64_xoshi(d+1); // random vector
            let mut vm = vec![0f64;d+1];
            vm.clone_from(&v);

            let n_med = naive_median(&mut vm); 
            n_error += balance(&v,n_med).abs();

            let w_med = w_median(&v); 
            w_error += balance(&v,w_med).abs();          
      
            let r_med = r_median(&v);
            r_error += balance(&v,r_med).abs();

            let med = v.as_slice().median();
            h_error += balance(&v,med).abs();
            
            // println!("Odd Medians: {:9.6} {:9.6} {:9.6}",n_med,w_med,r_med);
        }; 
        println!("{GR}errors: {:.10}{UN}",w_error-n_error);  
        println!("{GR}errors: {:.10}{UN}",r_error-n_error);
        println!("{GR}errors: {:.10}{UN}",h_error-n_error)
    }
}
