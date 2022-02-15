#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]
use devtimer::DevTime;
use anyhow::Result;
use medians::{balance,naive_median,w_median,hash_median};
use indxvec::{GR,UN};
use ran::*;

#[test]
fn naive() {
   let d = 10001_usize;
   let n = 12_usize;
   println!("\nTesting on a set of {} random f64 vectors of length {} each\n",n,d);
   let mut n_error = 0_i64;
   let mut w_error = 0_i64;
   let mut i_error = 0_i64;
   let mut n_timer = DevTime::new_simple();
   let mut w_timer = DevTime::new_simple();
   let mut i_timer = DevTime::new_simple();
   let (mut n_time, mut w_time, mut i_time) = (0_u128, 0_u128, 0_u128); 
   set_seeds(7777777777_u64);

   for _i in 0..n {
      let min = 0_f64;
      let max = 255_f64;
      let v = ranvu8(d);
      let mut vm = vec![0u8;d];
      vm.clone_from(&v);
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
      let i_med = hash_median(&v,min,max);
      i_timer.stop();
      i_time += i_timer.time_in_nanos().unwrap();
      i_error += balance(&v,i_med).abs();

      println!("Medians: {:9.6} {:9.6} {:9.6}",n_med,w_med,i_med);
   }
   //let n_time = n_timer.time_in_nanos().unwrap();
   //let w_time = w_timer.time_in_nanos().unwrap();
   //let i_time = i_timer.time_in_nanos().unwrap();

   let mut tbal = 100_f64*(w_time as f64 - n_time as f64);
   println!("\nw_median versus naive_median {GR}time: {:6.2}% errors: {:.10}{UN}",
      tbal/n_time as f64,w_error-n_error);
   tbal = 100_f64*(i_time as f64 - n_time as f64);
   println!("hashsort_median versus naive_median {GR}time: {:6.2}% errors: {:.10}{UN}",
      tbal/n_time as f64,i_error-n_error); 
}
