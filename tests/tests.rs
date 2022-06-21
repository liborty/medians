#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]
use devtimer::DevTime;
use anyhow::Result;
use medians::{naive_median,w_median,hash_median,r_median};
use indxvec::printing::*;
use ran::{*,generators::*};

/// used to measure errors
pub fn balance<T>(s:&[T],x:f64) -> i64 where T: Copy,f64:From<T> {
    let mut bal = 0_i64;
    for &si in s { 
        let d = f64::from(si)-x;
        bal += d.signum() as i64;
    }
    bal
}

#[test]
fn naive() {
   let d = 1001_usize;
   let n = 12_usize;
   println!("\nTesting on a set of {GR}{}{UN} random vectors of length {GR}{}{UN} each\n",n,d);
   let mut n_error = 0_i64;
   let mut w_error = 0_i64;
   let mut i_error = 0_i64;
   let mut r_error = 0_i64;
   let mut n_timer = DevTime::new_simple();
   let mut w_timer = DevTime::new_simple();
   let mut i_timer = DevTime::new_simple();
   let mut r_timer = DevTime::new_simple();
   let (mut n_time, mut w_time, mut i_time, mut r_time) = (0_u128, 0_u128, 0_u128, 0_u128); 
   set_seeds(7777777777_u64);

   for _i in 0..n {
      let min = 0_f64;
      let max = 255_f64;
      let v = ranvu8(d);
      let mut vm = vec![0u8;d];
      vm.clone_from(&v);
      let mut vhash = vec![0u8;d];
      vhash.clone_from(&v);
      //println!("{}",v.gr());
      //println!("{}",hashsort(&v,0.0,1.0).unindex(&v,true).gr());
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
    
      i_timer.start();
      let i_med = hash_median(&mut vhash,min,max);
      i_timer.stop();
      i_time += i_timer.time_in_nanos().unwrap();
      i_error += balance(&v,i_med).abs();

      r_timer.start();
      let r_med = r_median(&vhash,min,max);
      r_timer.stop();
      r_time += r_timer.time_in_nanos().unwrap();
      r_error += balance(&v,r_med).abs();

      println!("Medians: {:9.6} {:9.6} {:9.6} {:9.6}",n_med,w_med,i_med,r_med);
   }
   //let n_time = n_timer.time_in_nanos().unwrap();
   //let w_time = w_timer.time_in_nanos().unwrap();
   //let i_time = i_timer.time_in_nanos().unwrap();

   let mut tbal = 100_f64*(w_time as f64 - n_time as f64);
   println!("\n{GR}w_median time:   {:6.2}% errors: {:.10}{UN}",
      tbal/n_time as f64,w_error-n_error);
   tbal = 100_f64*(i_time as f64 - n_time as f64);
   println!("{GR}hashsort_median: {:6.2}% errors: {:.10}{UN}",
      tbal/n_time as f64,i_error-n_error); 
    tbal = 100_f64*(r_time as f64 - n_time as f64);
    println!("{GR}r_median time:   {:6.2}% errors: {:.10}{UN}",
      tbal/n_time as f64,r_error-n_error) 
}
