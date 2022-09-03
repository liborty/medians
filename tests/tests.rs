#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]
use devtimer::{DevTime,SimpleTimer};
use medians::Median;
use medians::algos::{balance,naive_median,r_median};
use indxvec::printing::*;
use ran::{*,generators::*};
use indxvec::{ here, printing::*, Indices, Printing, Vecops, Mutops};
use ran::*;
use std::convert::From;
use times::{benchu8,benchu64,benchf64};

const NAMES:[&str;2] = [ "naive_median","median" ];

const CLOSURESF64:[fn(&[f64]);2] = [
    |v:&[_]| { naive_median(v); },
    |v:&[_]| { v.median(); } ];

#[test]
fn comparison() {
    set_seeds(7777777777_u64);   // intialise random numbers generator
    // Rnum encapsulates the type of the data items
   benchf64(Rnum::newf64(),5,20,&NAMES,&CLOSURESF64); 
}

#[test]
fn errors() { 
    let n = 10_usize; // number of vectors to test for each magnitude
    set_seeds(777777777_u64);   // intialise random numbers generator
    for d in [10,50,100,1000,10000,100000] { 
        let mut error = 0_i64;        
        print!("\nEven lengths: {GR}{}{UN}, repeats: {GR}{}{UN}, ",d,n);
        for _ in 0..n { 
            let v = ranvf64_xoshi(d); // random vector  
            let med = v.median(); 
            error += balance(&v,med);
        };
        println!("errors: {GR}{}{UN}",error);
        error = 0_i64;
        print!("Odd lengths:  {GR}{}{UN}, repeats: {GR}{}{UN}, ",d+1,n);
        for _ in 0..n {
            let v = ranvf64_xoshi(d+1); // random vector
            let med = v.median();
            error += balance(&v,med);
        }; 
        println!("errors: {GR}{}{UN}",error)
    }
}
