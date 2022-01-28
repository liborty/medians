// #![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]

use devtimer::DevTime;
use anyhow::Result;
use medians::{naive_median,median};
use rstats::{Stats,wv,wi};
use random_number::random_fill;

#[test]
fn naive() -> Result<()> {
   let v1 = vec![1_f64,2.,3.,4.,5.,6.,7.,8.,9.,10.,11.,12.,13.,14.,15.,16.]; 
   println!("{}",wv(&v1));    
   println!("{}",v1.median().unwrap());
   println!("Mad:\t\t {}",wi(&v1.mad().unwrap())); 
   println!("Naive Median:\t {}",wi(&naive_median(&v1).unwrap()));
   println!("Fast Median: \t {}",wi(&median(&v1).unwrap()));
   let d = 10000_usize;
   let n = 10_usize;
   println!("\nTesting on a set of {} random u8 vectors of length {} each",wi(&n),wi(&d)); 
   let mut v = vec![0u8;333];
   let mut n_time = 0_u128;
   let mut f_time = 0_u128;
   let mut n_timer = DevTime::new_simple();
   let mut f_timer = DevTime::new_simple();
   for _i in 0..n {
      random_fill!(v);
      n_timer.start();
      let n_med = naive_median(&v).unwrap();
      n_timer.stop();
      n_time += n_timer.time_in_nanos().unwrap();
      f_timer.start();
      let f_med = median(&v).unwrap();
      f_timer.stop();
      f_time += f_timer.time_in_nanos().unwrap();       
      println!("Medians: {} {}",wi(&n_med),wi(&f_med));
   }
   let totaltime = f_time + n_time;
   let tbal = f_time as i128 - n_time as i128;
   println!("Total Time {} saved: {} nanoseconds",wi(&totaltime),wi(&tbal));
   Ok(())
}
 