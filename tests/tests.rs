// #![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]

use devtimer::DevTime;
use anyhow::Result;
use medians::naive_median;
use rstats::{Stats,wv,wi};
use random_number::random_fill;

#[test]
fn naive() -> Result<()> {
   let v1 = vec![1_f64,2.,3.,4.,5.,6.,7.,8.,9.,10.,11.,12.,13.,14.,15.]; 
   println!("{}",wv(&v1));    
   println!("{}",v1.median().unwrap());
   println!("Mad:\t\t {}",wi(&v1.mad().unwrap())); 
   println!("Naive Median:\t {}",wi(&naive_median(&v1).unwrap()));
   let d = 500_usize;
   let n = 10_usize;
   println!("\nTesting on a set of {} random u8 vectors of length {} each",wi(&n),wi(&d)); 
   let mut v = vec![0u8;333];
   let mut sumtime = 0_u128;
   let mut timer = DevTime::new_simple();
      for _i in 0..n {
      random_fill!(v);
      timer.start();
      let m = naive_median(&v).unwrap();
      timer.stop();
      sumtime += timer.time_in_nanos().unwrap(); 
      println!("Median: {}",wi(&m));
   }
   println!("Total Time: {:<12} seconds",wi(&(sumtime as f64/1e9)));
   Ok(())
}
 