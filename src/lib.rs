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

#[derive(Default)]
/// Median, quartiles, mad (median of absolute diffs)
pub struct Med {
    /// the median value
    pub median: f64,
    /// lower quartile, as MND (median of negative differences)
    pub lq: f64,
    /// upper quartile, as MPD (median of positive differences)
    pub uq: f64,
    /// median of absolute differences (from median)
    pub mad: f64,
    /// standard error - mad divided by median
    pub ste: f64,
}
impl std::fmt::Display for Med {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "median:
    \tLower Q: {GR}{:>.16}{UN}
    \tMedian:  {GR}{:>.16}{UN}
    \tUpper Q: {GR}{:>.16}{UN}
    \tMad:     {GR}{:>.16}{UN}
    \tStd Err: {GR}{:>.16}{UN}",
            self.lq, self.median, self.uq, self.mad, self.ste
        )
    }
}

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

/// Fast 1D medians and associated information and tasks
pub trait Median<T> {
    /// Finds the median of `&[T]`, fast
    fn median(self, quantify: &mut impl FnMut(&T) -> f64) -> Result<f64, ME>;
    /// Finds the median of odd sized nonquantifiable Ord data
    fn odd_strict_median(self) -> T
    where
        T: Ord + Clone;
    /// Finds the two mid values of even sized nonquantifiable Ord data
    fn even_strict_median(self) -> (T,T)
    where
        T: Ord + Clone;
    /// Zero median f64 data produced by finding and subtracting the median.
    fn zeromedian(self, quantify: &mut impl FnMut(&T) -> f64) -> Result<Vec<f64>, ME>;
    /// Median correlation = cosine of an angle between two zero median vecs
    fn mediancorr(
        self,
        v: &[T],
        quantify: &'static mut impl FnMut(&T) -> f64,
    ) -> Result<f64, MedError<String>>;
    /// Median of absolute differences (MAD).
    fn mad(self, med: f64, quantify: &mut impl FnMut(&T) -> f64) -> Result<f64, ME>;
    /// Median and MAD.
    fn medstats(self, quantify: &mut impl FnMut(&T) -> f64) -> Result<MStats, ME>;
    /// Median, quartiles, MAD, Stderr
    fn medinfo(self, quantify: &mut impl FnMut(&T) -> f64) -> Result<Med, ME>;
}

impl<T> Median<T> for &[T] {
    /// Median using user defined quantification, allowing T->f64 conversion and
    /// then very efficient pivoting using the mean
    fn median(self, quantify: &mut impl FnMut(&T) -> f64) -> Result<f64, ME> {
        let n = self.len();
        match n {
            0 => Err(MedError::SizeError("median: zero length data".to_owned())),
            1 => Ok(quantify(&self[0])),
            2 => Ok((quantify(&self[0]) + quantify(&self[1])) / 2.0),
            _ => Ok(auto_median(self, quantify)),
        }
    }

    /// Finds the median of odd sized data, which is not quantifiable
    fn odd_strict_median(self) -> T
    where
        T: Ord + Clone,
    {
        self.max_1_min_k(self.len() / 2 + 1)
    }

    /// Finds the two mid values of even sized data, which is not quantifiable
    fn even_strict_median(self) -> (T,T)
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
    /// assert_eq!(v1.mediancorr(&v2,&mut |f:&f64| *f).unwrap(),-0.1076923076923077);
    /// ```
    fn mediancorr(
        self,
        v: &[T],
        quantify: &'static mut impl FnMut(&T) -> f64,
    ) -> Result<f64, MedError<String>> {
        let mut sx2 = 0_f64;
        let mut sy2 = 0_f64;
        let selfmedian = self.median(quantify)?;
        let vmedian = v.median(quantify)?;
        let sxy: f64 = self
            .iter()
            .zip(v)
            .map(|(xt,yt)| {
                let x = quantify(xt)-selfmedian;
                let y = quantify(yt)-vmedian;
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

    /// Full median information: central tendency, quartiles and MAD spread
    fn medinfo(self, quantify: &mut impl FnMut(&T) -> f64) -> Result<Med, ME> {
        let mut deref = |t: &f64| *t;
        let mut equals = 0_usize;
        let mut posdifs: Vec<f64> = Vec::new();
        let mut negdifs: Vec<f64> = Vec::new();
        let med = self.median(quantify)?;
        for s in self {
            let sf = quantify(s);
            if sf > med {
                posdifs.push(sf - med)
            } else if sf < med {
                negdifs.push(med - sf)
            } else {
                equals += 1
            };
        }
        if equals > 1 {
            let eqhalf = vec![0.; equals / 2];
            let eqslice = vec![0.; equals];
            let lq = med - negdifs.unite_unsorted(&eqhalf).median(&mut deref)?;
            let uq = med + eqhalf.unite_unsorted(&posdifs).median(&mut deref)?;
            let mad = [negdifs, eqslice, posdifs].concat().median(&mut deref)?;
            Ok(Med {
                median: med,
                lq,
                uq,
                mad,
                ste: mad / med,
            })
        } else {
            let lq = med - negdifs.median(&mut deref)?;
            let uq = med + posdifs.median(&mut deref)?;
            let mad = [negdifs, posdifs].concat().median(&mut deref)?;
            Ok(Med {
                median: med,
                lq,
                uq,
                mad,
                ste: mad / med,
            })
        }
    }
}
