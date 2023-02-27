#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]
use devtimer::{DevTime,SimpleTimer};
use medians::{Medianf64,Median};
use medians::algos::{balance,spart};
use ran::{*,generators::*};
use indxvec::{ here, printing::*, Indices, Printing, Vecops, Mutops};
use ran::*;
// use std::io::{stdout,Write};
use std::convert::From;
use times::{benchu8,benchu64,benchf64,mutbenchf64};

const NAMES:[&str;2] = [ "medianf64","quantized median" ]; //, "strict meadian" ];

const CLOSURESF64:[fn(&[f64]);2] = [ 
    |v:&[_]| { v.medianf64().unwrap(); },
    |v:&[_]| { v.median(&mut |&x| x).unwrap(); } ];  // use x.into() when not f64
    // |v:&[_]| { v.odd_strict_median(); } ];

/*
#[test]
fn sparting() {
    let len:usize = 20;
    set_seeds(55557777_u64);   // intialise random numbers generator
    let mut v = ranvf64_xoshi(20)
        .expect("Random numbers generation error"); // random vector 
    println!("Input set:\n{}",v.gr()); 
    let mut input:String = Default::default();
    print!("Type your float pivot between 0. and 1.: "); 
    std::io::stdout().flush().expect("Failed to flush stdout");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");
    let pivot = input.trim().parse::<f64>()
        .expect("Failed to parse f64");
    println!("Pivot {}",pivot.yl()); 
    let gpart = spart(&mut v,0,len,pivot);
    println!("[ltset, geset]:\n{GR}[{},\n {}]{UN}",
        &v[0..gpart].to_plainstr(),&v[gpart..len].to_plainstr()); 
}
*/

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
    println!("Even median lexographic words: {}",v.as_slice().even_strict_median().yl());
    }

#[test]
fn medf64() {
    let v = [1.,2.,3.,4.,5.,6.,7.,8.,9.,10.,11.,12.,13.,14.,15.,16.,17.];
    let med = v.medianf64().unwrap();
    println!("{}\nMedian: {}",v.gr(),med.gr());
    println!("Medstats: {}",v.medstatsf64().unwrap());
}

#[test]
fn comparison() {
    set_seeds(7777777777_u64);   // intialise random numbers generator
    // Rnum encapsulates the type of the data items
   benchf64(Rnum::newf64(),7..10000,500,5,&NAMES,&CLOSURESF64); 
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
            let med = v.median(&mut |t:&u8| *t as f64).expect("even errors test");
            error += balance(&v,med,&mut |f| *f as f64);
            // let (med1,med2) = v.even_strict_median();  
            //error += balance(&v,(med1 as f64 + med2 as f64)/2.,&mut |f| *f as f64);
            // println!("{} balance: {}",med, balance(&v,med) );
        };
        println!("\nEven lengths: {GR}{d}{UN}, repeats: {GR}{n}{UN}, errors: {GR}{error}{UN}"); 
        error = 0_i64; 
        
        for _ in 0..n {
            let v = ranvu8(d+1).unwrap(); // random vector
            let med = v.median(&mut |t:&u8| *t as f64).expect("odd errors test");
            //let OneTwo::One(med) = partial_median(&v) else { panic!("Odd test failed"); }; 
            // let medix = v.odd_strict_median();
            error += balance(&v,med,&mut |f| *f as f64);
        };
        println!("Odd lengths:  {GR}{}{UN}, repeats: {GR}{}{UN}, errors: {GR}{}{UN}",d+1,n,error);
    }
}
