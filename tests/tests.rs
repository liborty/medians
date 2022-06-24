#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]
use devtimer::{DevTime,SimpleTimer};
use medians::{naive_median,w_median,r_median,median};
use indxvec::printing::*;
use ran::{*,generators::*};

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
fn magnitudes() { 
    let n = 10_usize; // number of vectors to test for each magnitude
    set_seeds(7777777777_u64);   // intialise random numbers generator
    for d in [10,100,1000,10000,100000] {
        let mut n_error = 0_i64;
        let mut w_error = 0_i64; 
        let mut r_error = 0_i64;
        let mut h_error = 0_i64;
        let mut n_timer = DevTime::new_simple();
        let mut w_timer = DevTime::new_simple();  
        let mut r_timer = DevTime::new_simple();
        let mut h_timer = DevTime::new_simple();
        let (mut n_time, mut w_time, mut r_time, mut h_time) = (0_u128, 0_u128, 0_u128, 0_u128);        
        
        println!("\nTesting medians on a set of {GR}{}{UN} random vectors of length {GR}{}{UN} each",n,d);
        for _ in 0..n {
            let v = ranvf64_xoshi(d); // random vector
            let mut vm = vec![0f64;d];
            vm.clone_from(&v);

            n_timer.start();
            let n_med = naive_median(&mut vm);
            n_timer.stop();
            n_time += n_timer.time_in_nanos().unwrap();
            n_error += balance(&v,n_med).abs();

            w_timer.start();
            let w_med = w_median(&v);
            w_timer.stop();
            w_time += w_timer.time_in_nanos().unwrap();
            w_error += balance(&v,w_med).abs();          
      
            r_timer.start();
            let r_med = r_median(&v);
            r_timer.stop();
            r_time += r_timer.time_in_nanos().unwrap();
            r_error += balance(&v,r_med).abs();

            h_timer.start();
            let med = median(&v);
            h_timer.stop();
            h_time += h_timer.time_in_nanos().unwrap();
            h_error += balance(&v,med).abs();
            
            // println!("Even Medians: {:9.6} {:9.6} {:9.6}",n_med,w_med,r_med);
        };
        println!("Testing odd medians on a set of {GR}{}{UN} random vectors of length {GR}{}{UN} each",n,d+1);
        for _ in 0..n {
            let v = ranvf64_xoshi(d+1); // random vector
            let mut vm = vec![0f64;d+1];
            vm.clone_from(&v);

            n_timer.start();
            let n_med = naive_median(&mut vm);
            n_timer.stop();
            n_time += n_timer.time_in_nanos().unwrap();
            n_error += balance(&v,n_med).abs();

            w_timer.start();
            let w_med = w_median(&v);
            w_timer.stop();
            w_time += w_timer.time_in_nanos().unwrap();
            w_error += balance(&v,w_med).abs();          
      
            r_timer.start();
            let r_med = r_median(&v);
            r_timer.stop();
            r_time += r_timer.time_in_nanos().unwrap();
            r_error += balance(&v,r_med).abs();

            h_timer.start();
            let med = median(&v);
            h_timer.stop();
            h_time += h_timer.time_in_nanos().unwrap();
            h_error += balance(&v,med).abs();
            
            // println!("Odd Medians: {:9.6} {:9.6} {:9.6}",n_med,w_med,r_med);
        }; 
        println!("\n{GR}Naive m. time: 100%"); 
        println!("{GR}w_median time: {:6.2}% errors: {:.10}{UN}",100.*w_time as f64/n_time as f64,w_error-n_error);  
        println!("{GR}r_median time: {:6.2}% errors: {:.10}{UN}",100.*r_time as f64/n_time as f64,r_error-n_error);
        println!("{GR}median time:   {:6.2}% errors: {:.10}{UN}",100.*h_time as f64/n_time as f64,h_error-n_error)
    }
}
