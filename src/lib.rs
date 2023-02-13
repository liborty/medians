#![warn(missing_docs)]

//! Fast new algorithms for computing medians of
//! (one dimensional) vectors

/// Functions for finding medians
pub mod algos;
/// Custom error
pub mod error;

pub use crate::{algos::*, error::MedError};
use indxvec::{
    printing::{GR, UN},
    Vecops,
};

/// Shorthand type for returned errors with message payload
pub type ME = MedError<String>;

/*
/// The following defines Ordf64 struct which is a wrapped f64 that implements Ord.
/// This would enable the use of strict medians, which require their T to be Ord.
/// It is here for instruction only, as to how to make other user types T implement Ord.
/// Normally, on f64s, it is more efficient to use Median64 trait.
/// Any quantifiable types should use Median trait.
/// Only non numeric unquantizable types will need the strict medians.
use core::cmp::{Ordering, Ordering::*};
use core::ops::{Deref,DerefMut};

//#[derive(PartialOrd)]
pub struct Ordf64 {
    pub myf64: f64
}

impl Deref for Ordf64 {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.myf64
    }
}

impl DerefMut for Ordf64 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.myf64
    }
}

impl PartialEq for Ordf64 {
    fn eq(&self, other: &Self) -> bool {
        if *self < *other { return false; };
        if *self > *other { return false; };
        true
    }
}

impl PartialOrd for Ordf64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if *self < *other { return Some(Less) };
        if *self > *other { return Some(Greater) };
        None
    }
}

impl Ord for Ordf64 {
    fn cmp(&self, other: &Self) -> Ordering {
        if *self < *other { return Less };
        if *self > *other { return Greater };
        Equal
    }
}
impl Eq for Ordf64 {}
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
            "centre: {GR}{:<10e}{UN}\tdispersion: {GR}{:<10e}{UN}",
            self.centre, self.dispersion
        )
    }
}

///Fast 1D f64 medians and associated information and tasks
pub trait Medianf64 {
    /// Finds the median of `&[f64]`, fast
    fn medianf64(self) -> Result<f64, ME>;
    /// Median correlation = cosine of an angle between two zero median vecs
    fn mediancorrf64(self, v: &[f64]) -> Result<f64, MedError<String>>;
    /// Median of absolute differences (MAD).
    fn madf64(self, med: f64) -> Result<f64, ME>;
    /// Median and MAD.
    fn medstatsf64(self) -> Result<MStats, ME>;
}

impl Medianf64 for &[f64] {
    /// Iterative median, partitioning data by weighted mean as an estimated pivot.
    /// On average, this is faster than finding the midpoint between maximum and minimum values.
    /// 0. <= ratio <= 1. ratio = 0.5 finds median, 0.25 lower quartile, 0.75 upper quartile, etc.
    fn medianf64(self) -> Result<f64, ME> {
        let n = self.len();
        match n {
            0 => {
                return Err(MedError::SizeError(
                    "medianf64: zero length data".to_owned(),
                ))
            }
            1 => return Ok(self[0]),
            2 => return Ok((self[0] + self[1]) / 2.0),
            _ => (),
        };
        let mut fset = self.to_owned();
        if (n & 1) == 1 {
            Ok(med_odd(&mut fset))
        } else {
            Ok(med_even(&mut fset))
        }
    }

    /// We define median based correlation as cosine of an angle between two
    /// zero median vectors (analogously to Pearson's zero mean vectors)
    /// # Example
    /// ```
    /// use medians::Medianf64;
    /// let v1 = vec![1_f64,2.,3.,4.,5.,6.,7.,8.,9.,10.,11.,12.,13.,14.];
    /// let v2 = vec![14_f64,1.,13.,2.,12.,3.,11.,4.,10.,5.,9.,6.,8.,7.];
    /// assert_eq!(v1.mediancorrf64(&v2).unwrap(),-0.1076923076923077);
    /// ```
    fn mediancorrf64(self, v: &[f64]) -> Result<f64, ME> {
        let mut sx2 = 0_f64;
        let mut sy2 = 0_f64;
        let smedian = self.medianf64()?;
        let vmedian = v.medianf64()?;
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
    /// However, any central tendency can be used.
    fn madf64(self, med: f64) -> Result<f64, ME> {
        self.iter()
            .map(|s| (s - med).abs())
            .collect::<Vec<f64>>()
            .medianf64()
    }

    /// Centre and dispersion defined by median
    fn medstatsf64(self) -> Result<MStats, ME> {
        let centre = self.medianf64()?;
        Ok(MStats {
            centre,
            dispersion: self.madf64(centre)?,
        })
    }
}

/// Fast 1D generic medians and associated information and tasks
pub trait Median<T> {
    /// Finds the median of `&[T]`, fast
    fn median(self, quantify: &mut impl FnMut(&T) -> f64) -> Result<f64, ME>;
    /// Finds the median of odd sized nonquantifiable Ord data
    fn odd_strict_median(self) -> T
    where
        T: Ord + Clone;
    /// Finds the two mid values of even sized nonquantifiable Ord data
    fn even_strict_median(self) -> (T, T)
    where
        T: Ord + Clone;
    /// Zero median f64 data produced by finding and subtracting the median.
    fn zeromedian(self, quantify: &mut impl FnMut(&T) -> f64) -> Result<Vec<f64>, ME>;
    /// Median correlation = cosine of an angle between two zero median vecs
    fn mediancorr(self, v: &[T], quantify: &mut impl FnMut(&T) -> f64)
        -> Result<f64, MedError<String>>;
    /// Median of absolute differences (MAD).
    fn mad(self, med: f64, quantify: &mut impl FnMut(&T) -> f64) -> Result<f64, ME>;
    /// Median and MAD.
    fn medstats(self, quantify: &mut impl FnMut(&T) -> f64) -> Result<MStats, ME>;
}

impl<T> Median<T> for &[T] {
    /// Median using user defined quantification, allowing T->f64 conversion and
    /// then very efficient pivoting using the mean
    fn median(self, quantify: &mut impl FnMut(&T) -> f64) -> Result<f64, ME> {
        let fset = self.iter().map(quantify).collect::<Vec<f64>>();
        fset.medianf64()
    }

    /// Finds the median of odd sized data, which is not quantifiable
    fn odd_strict_median(self) -> T
    where
        T: Ord + Clone,
    {
        self.max_1_min_k(self.len() / 2 + 1)
    }

    /// Finds the two mid values of even sized data, which is not quantifiable
    fn even_strict_median(self) -> (T, T)
    where
        T: Ord + Clone,
    {
        self.max_2_min_k(self.len() / 2 + 1)
    }

    /// Zero median data produced by subtracting the median.
    /// Analogous to zero mean data when subtracting the mean.
    fn zeromedian(self, quantify: &mut impl FnMut(&T) -> f64) -> Result<Vec<f64>, ME> {
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
    /// assert_eq!(v1.mediancorr(&v2, &mut |f:&f64| *f).unwrap(),-0.1076923076923077);
    /// ```
    fn mediancorr(self, v: &[T], quantify: &mut impl FnMut(&T) -> f64) -> Result<f64, ME> {
        let mut sx2 = 0_f64;
        let mut sy2 = 0_f64;
        let selfmedian = self.median(quantify)?;
        let vmedian = v.median(quantify)?;
        let sxy: f64 = self
            .iter()
            .zip(v)
            .map(|(xt, yt)| {
                let x = quantify(xt) - selfmedian;
                let y = quantify(yt) - vmedian;
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
    /// However, any central tendency can be used.
    fn mad(self, med: f64, quantify: &mut impl FnMut(&T) -> f64) -> Result<f64, ME> {
        self.iter()
            .map(|s| ((quantify(s) - med).abs()))
            .collect::<Vec<f64>>()
            .median(&mut |f: &f64| *f)
    }

    /// Centre and dispersion defined by median
    fn medstats(self, quantify: &mut impl FnMut(&T) -> f64) -> Result<MStats, ME> {
        let centre = self.median(quantify)?;
        Ok(MStats {
            centre,
            dispersion: self.mad(centre, quantify)?,
        })
    }
}
