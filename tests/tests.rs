#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]
use devtimer::{DevTime,SimpleTimer};
use medians::Median;
use medians::algos::{balance,naive_median,w_median,r_median};
use indxvec::printing::*;
use ran::{*,generators::*};
use indxvec::{ here, printing::*, Indices, Printing, Vecops, Mutops};
use ran::*;
use std::convert::From;
use times::{benchu8,benchu64,benchf64};

const NAMES:[&str;4] = [ "naive_median","w_median","r_median","median" ];

const CLOSURESF64:[fn(&[f64]);4] = [
    |v:&[_]| { naive_median(v); },
    |v:&[_]| { w_median(v); }, 
    |v:&[_]| { r_median(v); }, 
    |v:&[_]| { v.median(); } ];

#[test]
fn comparison() {
    set_seeds(7777777777_u64);   // intialise random numbers generator
    // Rnum encapsulates the type of the data items
   benchf64(Rnum::newf64(),5,10,&NAMES,&CLOSURESF64); 
}

#[test]
fn errors() { 
    let n = 10_usize; // number of vectors to test for each magnitude
    set_seeds(777777777_u64);   // intialise random numbers generator
    for d in [10,100,1000,10000,100000] { 
        let mut error = 0_i64;        
        println!("\nTesting even medians on a set of {GR}{}{UN} random vectors of length {GR}{}{UN} each",n,d);
        for _ in 0..n { 
            let v = ranvf64_xoshi(d); // random vector 
            // println!("{}",v.medinfo());
            let med = v.median(); 
            error += balance(&v,med);
        };
        println!("Testing odd  medians on a set of {GR}{}{UN} random vectors of length {GR}{}{UN} each",n,d+1);
        for _ in 0..n {
            let v = ranvf64_xoshi(d+1); // random vector
            let med = v.median();
            error += balance(&v,med);
        }; 
        println!("{GR}errors: {}{UN}",error)
    }
}
