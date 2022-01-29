// #![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]

use devtimer::DevTime;
use anyhow::Result;
use medians::{naive_median,fast_median,mederror};
use random_number::random_fill;

#[test]
fn naive() -> Result<()> {
   let v1 = vec![1_f64,2.,3.,4.,5.,6.,7.,8.,9.,10.,11.,12.,13.,14.,15.,16.,17.]; 
   println!("{:?}",v1);
   println!("Naive Median:\t {}",naive_median(&v1).unwrap());
   println!("Fast Median: \t {}",fast_median(&v1).unwrap());
   let d = 10000_usize;
   let n = 10_usize;
   println!("\nTesting on a set of {} random f64 vectors of length {} each",n,d); 
   let mut v = vec![0f64;333];
   let mut n_time = 0_u128;
   let mut f_time = 0_u128;
   let mut n_error = 0_f64;
   let mut f_error = 0_f64;     
   let mut n_timer = DevTime::new_simple();
   let mut f_timer = DevTime::new_simple();
   for _i in 0..n {
      random_fill!(v);
      f_timer.start();
      let f_med = fast_median(&v).unwrap();
      f_timer.stop();   
      f_time += f_timer.time_in_nanos().unwrap();
      f_error += mederror(&v,f_med);   
      n_timer.start();
      let n_med = naive_median(&v).unwrap();
      n_timer.stop();
      n_time += n_timer.time_in_nanos().unwrap();
      n_error += mederror(&v,n_med);    
      println!("Medians: {} {}",n_med,f_med);
   }
   let totaltime = (f_time + n_time) as f64;
   let tbal = 100_f64*(f_time as f64 - n_time as f64);
   println!("Total Time {} seconds, new time is {:6.2}%\nNaive errors: {}\nFast errors {}",
      totaltime/1e9,tbal/totaltime,n_error,f_error);
   Ok(())
}
 