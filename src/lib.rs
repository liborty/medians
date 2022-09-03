#![warn(missing_docs)]

//! Fast new algorithms for computing medians of
//! (one dimensional) vectors

/// Functions for finding medians
pub mod algos;
use crate::algos::{naive_median, r_median};
use indxvec::{
    printing::{GR, UN},
    Vecops,
};

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

/// Finding 1D medians, quartiles, and MAD (median of absolute differences)
pub trait Median {
    /// Finds the median of `&[T]`, fast
    fn median(self) -> f64;
    /// Median of absolute differences (MAD).
    fn mad(self, median: f64) -> f64;
    /// Median and MAD.
    fn medstats(self) -> MStats;
    /// Median, quartiles, MAD, Stderr
    fn medinfo(self) -> Med;
}

impl<T> Median for &[T]
where
    T: Copy + PartialOrd,
    f64: From<T>,
{
    /// median 'big switch' chooses the best algorithm for a given length of input
    fn median(self) -> f64 {
        let n = self.len();
        if n == 0 {
            return 0_f64;
        };
        if n < 50 { 
            naive_median(self)
        } else {
            r_median(self)
        }
    }

    /// Data dispersion estimator MAD (Median of Absolute Differences).
    /// MAD is more stable than standard deviation and more general than quartiles.
    /// When argument `med` is the median, it is the most stable measure of data dispersion.
    /// However, any central tendency can be used.
    fn mad(self, med: f64) -> f64 {
        self.iter()
            .map(|&s| ((f64::from(s) - med).abs()))
            .collect::<Vec<f64>>()
            .median()
    }

    /// Centre and dispersion defined by median
    fn medstats(self) -> MStats {
        let centre = self.median();
        MStats {
            centre,
            dispersion: self.mad(centre),
        }
    }

    /// Full median information: central tendency, quartiles and MAD spread
    fn medinfo(self) -> Med {
        let mut equals = 0_usize;
        let mut posdifs: Vec<f64> = Vec::new();
        let mut negdifs: Vec<f64> = Vec::new();
        let med = self.median();
        for &s in self {
            let sf = f64::from(s);
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
            let lq = med - negdifs.unite_unsorted(&eqhalf).median();
            let uq = med + eqhalf.unite_unsorted(&posdifs).median();
            let mad = [negdifs, eqslice, posdifs].concat().median();
            Med {
                median: med,
                lq,
                uq,
                mad,
                ste: mad / med,
            }
        } else {
            let lq = med - negdifs.median();
            let uq = med + posdifs.median();
            let mad = [negdifs, posdifs].concat().median();
            Med {
                median: med,
                lq,
                uq,
                mad,
                ste: mad / med,
            }
        }
    }
}
