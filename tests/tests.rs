#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]
use devtimer::{DevTime,SimpleTimer};
use medians::Median;
use medians::algos::{fpart,balance,auto_median};
use ran::{*,generators::*};
use indxvec::{ here, printing::*, Indices, Printing, Vecops, Mutops};
use ran::*;
use std::convert::From;
use times::{benchu8,benchu64,benchf64};

const NAMES:[&str;1] = [ "auto_median" ];

const CLOSURESF64:[fn(&[f64]);1] = [ 
    |v:&[_]| { auto_median(v,&mut |&x| x as f64); } ];

#[test]
fn text() {
    let song = "There was a jolly miller once who lived on the river Dee. \
        From morn till night all day he sang for a jolly old fellow was he; \
        and this forever the burden of his song seemed to be: \
        I care for nobody, no not I, and nobody cares for me. Tee hee heee.";
    let v = song.split(' ').collect::<Vec<_>>();
    println!("{}", v.gr()); // Display
    println!("Hash sorted by word lengths: {}",v.sorth(&mut |&s| s.len() as f64,true).gr());
    let median_word = v.median(&mut |&s| s.len() as f64)
        .expect("text(): Median failed\n");
    println!("Median word length in bytes is: {}",median_word.yl());
    }

#[test]
fn parting() {
    let mut v = [9.,8.,7.,6.,5.,5.,5.,5.,5.,5.,4.,3.,2.,1.,0.];
    let len = v.len();
    println!("Parting index {}\n{}",fpart(&mut v,&(0..len), 5.0),v.gr());
}

#[test]
fn comparison() {
    set_seeds(7777777777_u64);   // intialise random numbers generator
    // Rnum encapsulates the type of the data items
   benchf64(Rnum::newf64(),1..10000,500,10,&NAMES,&CLOSURESF64); 
}

#[test]
fn errors() { 
    let n = 10_usize; // number of vectors to test for each magnitude
    set_seeds(77777777_u64);   // intialise random numbers generator
    for d in [10,50,100,1000,10000,100000] { 
        let mut error = 0_i64; 
        for _ in 0..n { 
            let v = ranvu8(d).unwrap(); // random vector  
            let med = v.median(&mut |t:&u8| *t as f64).expect("even errors test");  
            error += balance(&v,med,&mut |f| *f as f64);
            // println!("{} balance: {}",med, balance(&v,med) );
        };
        println!("\nEven lengths: {GR}{}{UN}, repeats: {GR}{}{UN}, errors: {GR}{}{UN}",d,n,error); 
        error = 0_i64; 
        for _ in 0..n {
            let v = ranvu8(d+1).unwrap(); // random vector
            let med = v.median(&mut |t:&u8| *t as f64).expect("odd errors test");
            error += balance(&v,med,&mut |f| *f as f64);
        };
        println!("Odd lengths:  {GR}{}{UN}, repeats: {GR}{}{UN}, errors: {GR}{}{UN}",d+1,n,error);
    }
}
