#![warn(missing_docs)]

//! Fast new algorithms for computing medians of
//! (one dimensional) vectors
//! 

/// Functions for finding medians
pub mod algos;
/// Functions specialized to f64
pub mod algosf64;
/// Custom errors
pub mod error;

//use core::cmp::{Ordering, Ordering::*};
//use core::ops::{Deref,DerefMut};
use core::fmt::Debug;
use crate::{algos::*, algosf64::{med_oddf64,med_evenf64}, error::{MedError,merror}};
use indxvec::{ printing::{GR, UN, YL}};

/// Shorthand type for medians errors with message payload specialized to String
pub type Me = MedError<String>;

/// The following defines Ord<T> struct which is a T that implements Ord.
/// This boilerplate makes any wrapped T:PartialOrd, such as f64, into Ord
#[derive(Clone,Copy,Debug)]

/*
pub struct Ordered<T>(pub T);

impl<T: std::fmt::Display > std::fmt::Display for Ordered<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let Ordered(x) = self;
        write!(f, "{x}" )
    }
}

impl<T> Deref for Ordered<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Ordered<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T:PartialOrd> PartialEq for Ordered<T> {
    fn eq(&self, other: &Self) -> bool {
        if **self < **other { return false; };
        if **self > **other { return false; };
        true
    }
}

impl<T:PartialOrd> PartialOrd for Ordered<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if **self < **other { return Some(Less) };
        if **self > **other { return Some(Greater) };
        None
    }
}

impl<T:PartialOrd> Ord for Ordered<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        if **self < **other { return Less };
        if **self > **other { return Greater };
        Equal
    }
}

impl<T:PartialOrd> Eq for Ordered<T> {}

impl<T> From<T> for Ordered<T> {
    fn from(f:T) -> Self {
        Ordered(f)
    }
}
*/

/// Holds measures of central tendency and spread.
/// Usually some kind of mean and its associated standard deviation, or median and its MAD
#[derive(Default)]
pub struct MStats {
    /// central tendency - (geometric, arithmetic, harmonic means or median)
    pub centre: f64,
    /// measure of data spread, typically standard deviation or MAD
    pub dispersion: f64,
}
impl std::fmt::Display for MStats {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{YL}centre: {GR}{:.5}{YL} ± spread: {GR}{:.5}{UN}",
            self.centre, self.dispersion
        )
    }
}

/// Fast 1D generic medians and associated information and tasks
pub trait Medianf64 {
    /// Finds the median of `&[T]`, fast. 
    fn median(self) -> Result<f64, Me>;  
     /// Zero median data produced by finding and subtracting the median. 
    fn zeromedian(self) -> Result<Vec<f64>, Me>;
    /// Median correlation = cosine of an angle between two zero median vecs
    fn mediancorr(self,v: &[f64]) -> Result<f64, Me>;
    /// Median of absolute differences (MAD).
    fn mad(self, med: f64) -> Result<f64, Me>;
    /// Median and MAD.
    fn medstats(self) -> Result<MStats, Me>;
}
impl Medianf64 for &[f64] {
    /// Returns single f64 number even for even medians
    fn median(self) -> Result<f64, Me> {
        let n = self.len();
        match n {
            0 => {
                return Err(merror("size","medianf64: zero length data")) 
            }
            1 => return Ok(self[0]),
            2 => return Ok((self[0] + self[1]) / 2.0),
            _ => ()
        };  
        let mut s = self.to_owned(); // quant_vec(self,quantify);
        if (n & 1) == 1 {
            Ok(med_oddf64(&mut s)) 
        } else { 
            let (med1,med2) = med_evenf64(&mut s); 
            Ok((med1+med2)/2.)
        }  
    }
    /// Zero median data produced by subtracting the median.
    /// Analogous to zero mean data when subtracting the mean.
    fn zeromedian(self) -> Result<Vec<f64>, Me> {
        let median = self.median()?;
        Ok(self.iter().map(|s| s - median).collect())
    }
    /// We define median based correlation as cosine of an angle between two
    /// zero median vectors (analogously to Pearson's zero mean vectors)
    /// # Example
    /// ```
    /// use medians::Medianf64;
    /// let v1 = vec![1_f64,2.,3.,4.,5.,6.,7.,8.,9.,10.,11.,12.,13.,14.];
    /// let v2 = vec![14_f64,1.,13.,2.,12.,3.,11.,4.,10.,5.,9.,6.,8.,7.];
    /// assert_eq!(v1.mediancorr(&v2).unwrap(),-0.1076923076923077);
    /// ```
    fn mediancorr(self, v: &[f64]) -> Result<f64, Me> {
        let mut sx2 = 0_f64;
        let mut sy2 = 0_f64;
        let smedian = self.median()?;
        let vmedian = v.median()?;
        let sxy: f64 = self
            .iter()
            .zip(v)
            .map(|(&xt, yt)| {
                let x = xt - smedian;
                let y = *yt - vmedian;
                sx2 += x * x;
                sy2 += y * y;
                x * y
            })
            .sum();
        Ok(sxy / (sx2 * sy2).sqrt())
    }

    /// Data dispersion estimator MAD (Median of Absolute Differences).
    /// MAD is more stable than standard deviation and more general than quartiles.
    /// When argument `med` is the median, it is the most stable measure of data dispersion. 
    fn mad(self, med: f64) -> Result<f64, Me> {
        self.iter()
            .map(|&s| (s - med).abs())
            .collect::<Vec<f64>>()
            .median()
    }

    /// Centre and dispersion defined by median
    fn medstats(self) -> Result<MStats, Me> {
        let centre = self.median()?;
        Ok(MStats {
            centre,
            dispersion: self.mad(centre)?,
        })
    }
}

/// Fast 1D generic medians and associated information and tasks.  
/// Using auto referencing to disambiguate conflicts 
/// with five more specific Medianf64 methods with the same names.  
/// To invoke specifically these generic versions, add a reference:  
/// `(&v[..]).method` or `v.as_slice().method`
pub trait Median<T> {
    /// Finds the median of `&[T]`, fast. 
    fn median(&self, quantify: &mut impl FnMut(&T) -> f64) -> Result<f64, Me>; 
    /// Odd median for any PartialOrd type T 
    fn generic_odd(&self) -> Result<&T, Me>;
    /// Even median for any PartialOrd type T 
    fn generic_even(&self) -> Result<(&T,&T), Me>;
    /// Finds the item at sort index k. For median, use k = self.len()/2 
    // fn strict_odd(&self, k:usize) -> Result<&T,Me>;
    /// Finds the two items from sort index k. For both even medians, use k = self.len()/2
    // fn strict_even(&self, k:usize) -> Result<(&T, &T),Me>;
    /// Zero median data produced by finding and subtracting the median. 
    fn zeromedian(&self, quantify: &mut impl FnMut(&T) -> f64) -> Result<Vec<f64>, Me>;
    /// Median correlation = cosine of an angle between two zero median vecs
    fn mediancorr(&self,v: &[T],quantify: &mut impl FnMut(&T) -> f64) -> Result<f64, Me>;
    /// Median of absolute differences (MAD).
    fn mad(&self, med: f64, quantify: &mut impl FnMut(&T) -> f64) -> Result<f64, Me>;
    /// Median and MAD.
    fn medstats(&self, quantify: &mut impl FnMut(&T) -> f64) -> Result<MStats, Me>;
}

impl<T> Median<T> for &[T]
where T:PartialOrd
{ 
    /// Median using user defined quantification for `T->U` conversion, where U:Ord 
    fn median(&self, quantify: &mut impl FnMut(&T) -> f64) -> Result<f64, Me>
    {
        let n = self.len(); 
        match n {
            0 => { 
                return Err(merror("size",format!("median: zero length data {n}")))
            }
            1 => return Ok(quantify(&self[0])),
            2 => return Ok((quantify(&self[0]) + quantify(&self[1])) / 2.),
            _ => ()
        }; 
        let mut s = quant_vec(self,quantify);
        if (n & 1) == 1 {
            Ok(med_oddf64(&mut s)) 
        } else { 
            let (m1,m2) = med_evenf64(&mut s);
            Ok((m1+m2)/2.0) 
        } 
    }

    /// Odd median for any PartialOrd type T 
    fn generic_odd(&self) -> Result<&T, Me>
    {
        let n = self.len(); 
        match n {
            0 => { 
                return Err(merror("size",format!("generic_odd_median: zero length data {n}")))
            }
            1 => return Ok(&self[0]), 
            _ => ()
        }; 
        if (n & 1) == 1 {
            Ok(med_odd(self)) 
        } else { 
            Err(merror("size",format!("generic_odd: even length data {n}"))) 
        } 
    }    

    /// Even median for any PartialOrd type T 
    fn generic_even(&self) -> Result<(&T,&T), Me>
        {
            let n = self.len(); 
            match n {
                0 => { 
                    return Err(merror("size",format!("generic_odd_median: zero length data {n}")))
                }
                2 => return Ok((&self[0],&self[1])), 
                _ => ()
            }; 
            if (n & 1) == 0 {
                Ok(med_even(self)) 
            } else { 
                Err(merror("size",format!("generic_even: odd length data {n}"))) 
            } 
        }    
/*
    /// Finds the item at sort index k using the heap method
    /// To find the median, use k = self.len()/2
    fn strict_odd(&self, k:usize) -> Result<&T,Me>
{
    let os = ord_vec(self);
    let s = os.as_slice();
    if let Some(&m) = s.smallest_k(k+1).peek() { Ok(m) }
        else { Err(merror("other","strict_odd: failed to peek smallest_k heap")) }
}    
    /// Finds the two items from sort index k, using the heap method.  
    /// To find both even medians, use k = self.len()/2
    fn strict_even(&self, k:usize) -> Result<(&T, &T),Me>
{
    let os = ord_vec(self);
    let s = os.as_slice();
        let mut heap = s.smallest_k(k+1); 
        let Some(m1) = heap.pop() else { 
            return Err(merror("other","strict_even: failed to pop smallest_k heap")); };
        let Some(&m2) = heap.peek() else { 
            return Err(merror("other","strict_even: failed to peek smallest_k heap")); };
    Ok((m2,m1))
}
*/
    /// Zero median data produced by subtracting the median.
    /// Analogous to zero mean data when subtracting the mean.
    fn zeromedian(&self, quantify: &mut impl FnMut(&T) -> f64) -> Result<Vec<f64>, Me>     
    {
        let median = self.median(quantify)?;
        Ok(self.iter().map(|s| quantify(s) - median).collect())
    }

    /// We define median based correlation as cosine of an angle between two
    /// zero median vectors (analogously to Pearson's zero mean vectors)
    /// # Example
    /// ```
    /// use medians::Median;
    /// let v1 = vec![1_f64,2.,3.,4.,5.,6.,7.,8.,9.,10.,11.,12.,13.,14.];
    /// let v2 = vec![14_f64,1.,13.,2.,12.,3.,11.,4.,10.,5.,9.,6.,8.,7.];
    /// assert_eq!((&v1[..]).mediancorr(&v2, &mut |&f| f).unwrap(),-0.1076923076923077);
    /// ```
    fn mediancorr(&self, v: &[T], quantify: &mut impl FnMut(&T) -> f64) -> Result<f64, Me> {
        let fself = quant_vec(self,quantify);
        let fv = quant_vec(v,quantify);
        fself.mediancorr(&fv)
    }

    /// Data dispersion estimator MAD (Median of Absolute Differences).
    /// MAD is more stable than standard deviation and more general than quartiles.
    /// When argument `med` is the median, it is the most stable measure of data dispersion.
    /// However, any central tendency can be used.
    fn mad(&self, med: f64, quantify: &mut impl FnMut(&T) -> f64) -> Result<f64, Me> {
        self.iter()
            .map(|s| ((quantify(s) - med).abs()))
            .collect::<Vec<f64>>()
            .median()
    }

    /// Centre and dispersion defined by median
    fn medstats(&self, quantify: &mut impl FnMut(&T) -> f64) -> Result<MStats, Me> {
        let centre = self.median(quantify)?;
        Ok(MStats {
            centre,
            dispersion: self.mad(centre, quantify)?,
        })
    }
}
