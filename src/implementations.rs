use std::error::Error;
use core::{fmt,cmp::Ordering};
use core::fmt::{Debug, Display};

use indxvec::{Vecops,printing::{GR, UN, YL}};
    use crate::{*,algos::*};

impl<T> Error for MedError<T> where T: Sized + Debug + Display {}

impl<T> Display for MedError<T>
where
    T: Sized + Debug + Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MedError::Size(s) => write!(f, "Size of data must be positive: {s}"),            
            MedError::Nan(s) => write!(f, "Floats must not include NaNs: {s}"), 
            MedError::Other(s) => write!(f, "Converted from: {s}"),
        }
    }
}

impl<T> std::fmt::Display for Medians<'_, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Medians::Odd(m) => {
                write!(f, "{YL}odd median: {GR}{}{UN}", *m)
            }
            Medians::Even((m1, m2)) => {
                write!(f, "{YL}even medians: {GR}{} {}{UN}", *m1, *m2)
            }
        }
    }
}

/// Medians of &mut [&f64].
impl Medianf64 for &[f64] {
    /// Returns `nan` error when any data item is a NaN, otherwise the median
    fn medf_checked(self) -> Result<f64, Me> {
        let n = self.len();
        match n {
            0 => return merror("size", "medf_checked: zero length data"),
            1 => return Ok(self[0]),
            2 => return Ok((self[0] + self[1]) / 2.0),
            _ => (),
        };
        let mut s = self
            .iter()
            .map(|x| {
                if x.is_nan() {
                    merror("Nan", "medf_checked: Nan in input!")
                } else {
                    Ok(x)
                }
            })
            .collect::<Result<Vec<&f64>, Me>>()?;        
        if (n & 1) == 1 {
            let oddm = oddmed_by(&mut s, 0..n, &mut <f64>::total_cmp);
            Ok(*oddm)
        } else {
            let (&med1, &med2) = evenmed_by(&mut s, 0..n, &mut <f64>::total_cmp);
            Ok((med1+med2) / 2.0)
        }
    }
    /// Use this when your data does not contain any NaNs.
    /// NaNs will not raise an error. However, they will affect the result
    /// because of their order positions beyond infinity.
    fn medf_unchecked(self) -> f64 {
        let n = self.len();
        match n {
            0 => return 0_f64,
            1 => return self[0],
            2 => return (self[0] + self[1]) / 2.0,
            _ => (),
        };
        let mut s = self.ref_vec(0..n);        
        if (n & 1) == 1 {
            let oddm = oddmed_by(&mut s, 0..n,&mut <f64>::total_cmp);
            *oddm
        } else {
            let (&med1, &med2) = evenmed_by(&mut s, 0..n,&mut <f64>::total_cmp);
            (med1 + med2) / 2.0
        }
    } 
    /// Iterative weighted median with accuracy eps
    fn medf_weighted(self, ws: Self, eps: f64) -> Result<f64, Me> { 
        if self.len() != ws.len() { 
            return merror("size","medf_weighted - data and weights lengths mismatch"); };
        if nans(self) {
            return merror("Nan","medf_weighted - detected Nan in input"); };
        let weights_sum: f64 = ws.iter().sum();
        let mut last_median  = 0_f64;
        for (g,w) in self.iter().zip(ws) { last_median += w*g; }; 
        last_median /= weights_sum; // start iterating from the weighted centre 
        let mut last_recsum = 0f64;
        loop { // iteration till accuracy eps is exceeded  
            let mut median = 0_f64;   
            let mut recsum = 0_f64;
            for (x,w) in self.iter().zip(ws) {   
                let mag = (x-last_median).abs(); 
                if mag.is_normal() { // only use this point if its distance from median is > 0.0
                    let rec = w/(mag.sqrt()); // weight/distance
                    median += rec*x; 
                    recsum += rec // add separately the reciprocals for final scaling   
                } 
            }
            if recsum-last_recsum < eps { return Ok(median/recsum); };  // termination test 
            last_median = median/recsum;
            last_recsum = recsum;            
        }
    }
    /// Zero mean/median data produced by subtracting the centre,
    /// typically the mean or the median.
    fn medf_zeroed(self, centre: f64) -> Vec<f64> {
        self.iter().map(|&s| s - centre).collect()
    }
    /// Median correlation = cosine of an angle between two zero median vectors,
    /// (where the two data samples are interpreted as n-dimensional vectors).
    fn medf_correlation(self, v: Self) -> Result<f64, Me> {
        let mut sx2 = 0_f64;
        let mut sy2 = 0_f64;
        let smedian = self.medf_checked()?;
        let vmedian = v.medf_checked()?;
        let sxy: f64 = self
            .iter()
            .zip(v)
            .map(|(&xt, &yt)| {
                let x = xt - smedian;
                let y = yt - vmedian;
                sx2 += x * x;
                sy2 += y * y;
                x * y
            })
            .sum();
        let res = sxy / (sx2 * sy2).sqrt();
        if res.is_nan() {
            merror("Nan", "medf_correlation: Nan result!")
        } else {
            Ok(res)
        }
    }
    /// Data dispersion estimator MAD (Median of Absolute Differences).
    /// MAD is more stable than standard deviation and more general than quartiles.
    /// When argument `centre` is the median, it is the most stable measure of data dispersion.
    fn madf(self, centre: f64) -> f64 {
        self.iter()
            .map(|&s| (s - centre).abs())
            .collect::<Vec<f64>>()
            .medf_unchecked()
    }
}

/// Medians of &[T]
impl<'a, T> Median<'a, T> for &'a [T] {
    /// Median of `&[T]` by comparison `c`, quantified to a single f64 by `q`.
    /// When T is a primitive type directly convertible to f64, pass in `as f64` for `q`.
    /// When f64:From<T> is implemented, pass in `|x| x.into()` for `q`.
    /// When T is Ord, use `|a,b| a.cmp(b)` as the comparator closure.
    /// In all other cases, use custom closures `c` and `q`.
    /// When T is not quantifiable at all, use `median_by` method.
    fn qmedian_by(
        self,
        c: &mut impl FnMut(&T, &T) -> Ordering,
        q: impl Fn(&T) -> f64,
    ) -> Result<f64, Me> {
        let n = self.len();
        match n {
            0 => return merror("size", "qmedian_by: zero length data"),
            1 => return Ok(q(&self[0])),
            2 => return Ok((q(&self[0]) + q(&self[1])) / 2.0),
            _ => (),
        };
        let mut s = self.ref_vec(0..self.len());
        if (n & 1) == 1 {
            Ok(q(oddmed_by(&mut s, 0..n,c)))
        } else {
            let (med1, med2) = evenmed_by(&mut s, 0..n, c);
            Ok((q(med1) + q(med2)) / 2.0)
        }
    }

    /// Median(s) of unquantifiable type by general comparison closure
    fn median_by(self, c: &mut impl FnMut(&T, &T) -> Ordering) -> Result<Medians<'a, T>, Me> {
        let n = self.len();
        match n {
            0 => return merror("size", "median_ord: zero length data"),
            1 => return Ok(Medians::Odd(&self[0])),
            2 => return Ok(Medians::Even((&self[0], &self[1]))),
            _ => (),
        };
        let mut s = self.ref_vec(0..self.len());
        if (n & 1) == 1 {
            Ok(Medians::Odd(oddmed_by(&mut s, 0..n, c)))
        } else {
            Ok(Medians::Even(evenmed_by(&mut s, 0..n, c)))
        }
    }

    /// Zero mean/median data produced by subtracting the centre
    fn zeroed(self, centre: f64, q: impl Fn(&T) -> f64) -> Result<Vec<f64>, Me> {
        Ok(self.iter().map(|s| q(s) - centre).collect())
    }
    /// We define median based correlation as cosine of an angle between two
    /// zero median vectors (analogously to Pearson's zero mean vectors) 
    fn med_correlation(
        self,
        v: Self,
        c: &mut impl FnMut(&T, &T) -> Ordering,
        q: impl Fn(&T) -> f64,
    ) -> Result<f64, Me> {
        let mut sx2 = 0_f64;
        let mut sy2 = 0_f64;
        let smedian = self.qmedian_by(c, &q)?;
        let vmedian = v.qmedian_by(c, &q)?;
        let sxy: f64 = self
            .iter()
            .zip(v)
            .map(|(xt, yt)| {
                let x = q(xt) - smedian;
                let y = q(yt) - vmedian;
                sx2 += x * x;
                sy2 += y * y;
                x * y
            })
            .sum();
        let res = sxy / (sx2 * sy2).sqrt();
        if res.is_nan() {
            merror("Nan", "correlation: Nan result!")
        } else {
            Ok(res)
        }
    }
    /// Data dispersion estimator MAD (Median of Absolute Differences).
    /// MAD is more stable than standard deviation and more general than quartiles.
    /// When argument `centre` is the median, it is the most stable measure of data dispersion.
    fn mad(self, centre: f64, q: impl Fn(&T) -> f64) -> f64 {
        self.iter()
            .map(|s| (q(s) - centre).abs())
            .collect::<Vec<f64>>()
            .medf_unchecked()
    }
}
