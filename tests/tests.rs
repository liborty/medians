// #![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]

use devtimer::DevTime;
use anyhow::Result;
use medians::{naive_median,w_median,i_median,mad};
use random_number::random_fill;

const GR:&str = "\x1B[01;92m";
const UNGR:&str = "\x1B[0m";

#[test]
fn naive() -> Result<()> { 
   let d = 10000_usize;
   let n = 12_usize;
   println!("\nTesting on a set of {} random f64 vectors of length {} each\n",n,d); 
   let mut v = vec![0f64;d];
   let mut n_time = 0_u128;
   let mut w_time = 0_u128;
   let mut i_time = 0_u128;
   let mut n_error = 0_f64;
   let mut w_error = 0_f64;  
   let mut i_error = 0_f64;     
   let mut n_timer = DevTime::new_simple();
   let mut w_timer = DevTime::new_simple();
   let mut i_timer = DevTime::new_simple();
   for _i in 0..n {
      random_fill!(v);

      n_timer.start();
      let n_med = naive_median(&v).unwrap();
      n_timer.stop();
      n_time += n_timer.time_in_nanos().unwrap();
      n_error += mad(&v,n_med);

      w_timer.start();
      let w_med = w_median(&v).unwrap();
      w_timer.stop();   
      w_time += w_timer.time_in_nanos().unwrap();
      w_error += mad(&v,w_med);

      i_timer.start();
      let i_med = i_median(&v).unwrap();
      i_timer.stop();
      i_time += i_timer.time_in_nanos().unwrap();
      i_error += mad(&v,i_med); 

      println!("Medians: {:9.6} {:9.6} {:9.6}",n_med,w_med,i_med);
   } 

   let mut tbal = 100_f64*(w_time as f64 - n_time as f64);
   println!("\nw_median versus naive_median {}time: {:6.2}% errors: {:.10}{}",
      GR,tbal/n_time as f64,w_error-n_error,UNGR);
   tbal = 100_f64*(i_time as f64 - n_time as f64);
   println!("i_median versus naive_median {}time: {:6.2}% errors: {:.10}{}",
      GR,tbal/n_time as f64,i_error-n_error,UNGR);
   Ok(())
}
 