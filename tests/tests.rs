#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]
use devtimer::{DevTime,SimpleTimer};
use medians::{Medianf64,Median};
use medians::algos::{balance,auto_median};
use ran::{*,generators::*};
use indxvec::{ here, printing::*, Indices, Printing, Vecops, Mutops};
use ran::*;
use std::convert::From;
use times::{benchu8,benchu64,benchf64,mutbenchf64};

const NAMES:[&str;2] = [ "medianf64","quantized median" ]; //, "strict meadian" ];

const CLOSURESF64:[fn(&[f64]);2] = [ 
    |v:&[_]| { v.medianf64().unwrap(); },
    |v:&[_]| { auto_median(v,&mut |&x| x); } ];  // use x.into() when not f64
    // |v:&[_]| { v.odd_strict_median(); } ];

#[test]
fn text() {
    let song = "There was a *jolly* miller once who lived on the river Dee. \
        From morn till night all day he sang for a jolly old fellow was he; \
        and this forever the burden of his song seemed to be: \
        I care for nobody, no not I, and nobody cares for me. \
        Tee hee heee, quoth he.";
    let v = song.split(' ').collect::<Vec<_>>();
    println!("{}", v.gr()); // Display
    println!("Hash sorted by word lengths: {}",v.sorth(&mut |&s| s.len() as f64,true).gr());
    let median_word = v.median(&mut |&s| s.len() as f64)
        .expect("text(): Median failed\n");
    println!("Median word length in bytes is: {}",median_word.yl());
    println!("Merge sorted by lexicon: {}",v.sortm(true).gr());
    println!("Even median lexographic words: {}",v.even_strict_median().yl());
    }

/*
#[test]
fn parting() {
    let v = [5.,9.,1.,2.,8.,7.,6.,5.,5.,7.,5.,5.,4.,3.,2.,1.,6.];
    let len = v.len();
    let mut idx = Vec::from_iter(0..len);
    let pivot = 5.0;
    println!("Pivot {}, Set:{}\nIndex {}\n",pivot.yl(),v.gr(),idx.gr());
    let (ltset,gtset) = partition(&v,&mut idx,&pivot);
    println!("ltset: {}\ngtset: {}\nequal items: {}\n",     
        ltset.gr(),
        gtset.gr(),
        (idx.len()-ltset.len()-gtset.len()).gr()
    );
}

#[test]
fn minmax() {
    let v = [5.,9.,8.,7.,6.,5.,5.,5.,5.,5.,4.,3.,2.,1.,6.];
    let len = v.len();
    let mut idx = Vec::from_iter(0..len);
    println!("{}\nParting index {}\n{}", 
    v.gr(),   
    minmaxpt(&v,&mut idx,&(0..len)).yl(),
    idx.gr()
    );
}
*/

#[test]
fn medf64() {
    set_seeds(7777777777_u64);   // intialise random numbers generator
    let v = ranvf64(10).unwrap();
    println!("{}\nmedian: {}",v.gr(),v.medianf64().unwrap());
}


#[test]
fn comparison() {
    set_seeds(7777777777_u64);   // intialise random numbers generator
    // Rnum encapsulates the type of the data items
   benchf64(Rnum::newf64(),4..10000,500,10,&NAMES,&CLOSURESF64); 
}


#[test]
fn errors() { 
    let n = 10_usize; // number of vectors to test for each magnitude
    set_seeds(77777777_u64);   // intialise random numbers generator
        for d in [10,50,100,1000,10000,100000] {  
        let mut error = 0_i64;
        trait Eq: PartialEq<Self> {}
        impl Eq for f64 {} 
        
        for _ in 0..n { 
            let v = ranvu8(d).unwrap(); // random vector  
            // let med = v.median(&mut |t:&u8| *t as f64).expect("even errors test");
            let (med1,med2) = v.even_strict_median();  
            error += balance(&v,(med1 as f64 + med2 as f64)/2.,&mut |f| *f as f64);
            // println!("{} balance: {}",med, balance(&v,med) );
        };
        println!("\nEven lengths: {GR}{d}{UN}, repeats: {GR}{n}{UN}, errors: {GR}{error}{UN}"); 
        error = 0_i64; 
        
        for _ in 0..n {
            let v = ranvu8(d+1).unwrap(); // random vector
            //let med = v.median(&mut |t:&u8| *t as f64).expect("odd errors test");
            //let OneTwo::One(med) = partial_median(&v) else { panic!("Odd test failed"); }; 
            let medix = v.odd_strict_median();
            error += balance(&v,medix as f64,&mut |f| *f as f64);
        };
        println!("Odd lengths:  {GR}{}{UN}, repeats: {GR}{}{UN}, errors: {GR}{}{UN}",d+1,n,error);
    }
}
