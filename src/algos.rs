use crate::MedError;
use core::ops::{Range, RangeInclusive};
use indxvec::{Mutops, Vecops};

/// measure errors in median
pub fn balance<T>(s: &[T], x: f64) -> i64
where
    T: Copy,
    f64: From<T>,
{
    let mut above = 0_i64;
    let mut below = 0_i64;
    for &si in s {
        let sif = f64::from(si);
        if sif > x {
            above += 1;
        } else if sif < x {
            below += 1;
        };
    }
    // println!("{}, {}",below,above);
    if below == above {
        return 0;
    };
    let diff = (above - below).abs();
    if diff < (s.len() as i64 - above - below) {
        return 0;
    };
    1
}

/// Median of a &[T] slice by sorting
/// Slow 'ground truth' for comparisons
/// Sorts its mutable slice argument as a side effect
/// # Example
/// ```
/// use medians::algos::naive_median;
/// let mut v = vec![1_u8,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
/// let res = naive_median(&mut v,&mut |&t| t as f64).expect("example failed");
/// assert_eq!(res,8_f64);
/// ```
pub fn naive_median<T, Q>(s: &[T], quantify: &mut Q) -> Result<f64, MedError<String>>
where
    T: Copy + PartialOrd,
    Q: FnMut(&T) -> f64,
{
    let n = s.len();
    if n == 0 {
        return Err(MedError::SizeError(
            "naive_median: empty vector!".to_owned(),
        ));
    };
    if n == 1 {
        return Ok(quantify(&s[0]));
    };
    if n == 2 {
        return Ok((quantify(&s[0]) + quantify(&s[1])) / 2.0);
    };
    let mut sf = s.iter().map(quantify).collect::<Vec<f64>>(); // convert-copy to allow mutating data
                                                               // sf.sort_unstable_by(|a, b| a.partial_cmp(b).expect("sort_unstable ordering failed"));
                                                               // Apply much faster hashsort. Already f64s, so just pass no op
    sf.muthashsort(&mut |t: &f64| *t);
    let mid = sf.len() / 2; // midpoint (floors odd sizes)
    if (n & 1) == 0 {
        Ok((sf[mid - 1] + sf[mid]) / 2.0)
    } else {
        Ok(sf[mid])
    } // s is odd
}

/// Used by r_median
fn part<T, Q>(s: &[T], pivot: f64, quantify: &mut Q) -> (Vec<T>, Vec<T>)
where
    T: Copy,
    Q: FnMut(&T) -> f64,
{
    let mut ltset = Vec::new();
    let mut gtset = Vec::new();
    for &f in s {
        if quantify(&f) < pivot {
            ltset.push(f);
        } else {
            gtset.push(f);
        };
    }
    (ltset, gtset)
}

/// Recursive Reducing Median
pub fn r_median<T, Q>(set: &[T], quantify: &mut Q) -> f64
where
    T: Copy + PartialOrd,
    Q: FnMut(&T) -> f64,
{
    // let s = tof64(set); // makes an f64 copy
    let n = set.len();
    // starting pivot
    let (min, max) = set.minmaxt();
    let pivot = (quantify(&min) + quantify(&max)) / 2.;
    // passing min max to save recomputing it
    if (n & 1) == 0 {
        r_med_even(set, n / 2, pivot, quantify(&min), quantify(&max), quantify)
    } else {
        r_med_odd(
            set,
            n / 2 + 1,
            pivot,
            quantify(&min),
            quantify(&max),
            quantify,
        )
    }
}

/// This, fastest, median calculation is applicable when the closed (inclusive) interval (range) of the data
/// is known in advance, even as a conservative estimate.
pub fn rng_median<T>(
    set: &[T],
    rng: RangeInclusive<f64>,
    quantify: &mut impl FnMut(&T) -> f64,
) -> f64
where
    T: Copy + PartialOrd,
{
    let n = set.len();
    // initial pivot somewhere in the middle of the range
    let pivot = (rng.start() + rng.end()) / 2.0;
    let fset = set.iter().map(quantify).collect::<Vec<f64>>();
    if (n & 1) == 1 {
        b_med_odd(fset, 0..n, pivot)
    } else {
        b_med_even(fset, 0..n, pivot)
    } 
}

/// Reducing sets median using `minmax()` and secant
/// with proportionally subdivided data range as a pivot.
/// Need is a count of items from start of set to anticipated median position
fn r_med_odd<T, Q>(
    set: &[T],
    need: usize,
    pivot: f64,
    setmin: f64,
    setmax: f64,
    quantify: &mut Q,
) -> f64
where
    T: PartialOrd + Copy,
    Q: FnMut(&T) -> f64,
{
    if need == 1 {
        return setmin;
    };
    let n = set.len();
    if need == n {
        return setmax;
    };

    let (ltset, gtset) = part(set, pivot, quantify);
    let ltlen = ltset.len();
    let gtlen = gtset.len();
    // println!("Need: {}, Pivot {:5.3}, partitions: {}, {}",need,pivot,ltlen,gtlen);

    match need {
        1 => quantify(&ltset.mint()),
        x if x < ltlen => {
            let max = quantify(&ltset.maxt());
            if setmin == max {
                return quantify(&ltset[0]);
            }; // all equal, done
            let newpivot = setmin + (need as f64) * (max - setmin) / (ltlen as f64);
            r_med_odd(&ltset, need, newpivot, setmin, max, quantify)
        }
        x if x == ltlen => quantify(&ltset.maxt()),
        x if x == ltlen + 1 => quantify(&gtset.mint()),
        x if x == n => quantify(&gtset.maxt()),
        _ => {
            // need > ltlen
            let newneed = need - ltlen;
            let min = quantify(&gtset.mint());
            if min == setmax {
                return quantify(&gtset[0]);
            }; // all equal, done
            let newpivot = min + (setmax - min) * (newneed as f64) / (gtlen as f64);
            r_med_odd(&gtset, newneed, newpivot, min, setmax, quantify)
        }
    }
}

/// Reducing sets median using `minmax()` and secant
/// with proportionally subdivided data range as a pivot.
/// Need is a count of items from start of set to anticipated median position
fn r_med_even<T, Q>(
    set: &[T],
    need: usize,
    pivot: f64,
    setmin: f64,
    setmax: f64,
    quantify: &mut Q,
) -> f64
where
    T: PartialOrd + Copy,
    Q: FnMut(&T) -> f64,
{
    // let n = set.len();
    let (ltset, gtset) = part(set, pivot, quantify);
    let ltlen = ltset.len();
    let gtlen = gtset.len();
    // println!("Need: {}, Pivot {}, partitions: {}, {}",need,pivot,ltlen,gtlen);
    match need {
        x if x < ltlen => {
            let max = quantify(&ltset.maxt());
            if setmin == max {
                return quantify(&ltset[0]);
            }; // all equal, done
            let newpivot = setmin + (need as f64) * (max - setmin) / (ltlen as f64);
            r_med_even(&ltset, need, newpivot, setmin, max, quantify)
        }
        x if x == ltlen => (quantify(&ltset.maxt()) + quantify(&gtset.mint())) / 2., // at the boundary
        // x if x == n => quantify(&gtset.maxt()),
        _ => {
            let newneed = need - ltlen;
            let min = quantify(&gtset.mint());
            if min == setmax {
                return quantify(&gtset[0]);
            }; // all equal, done
            let newpivot = min + (newneed as f64) * (setmax - min) / (gtlen as f64);
            r_med_even(&gtset, newneed, newpivot, min, setmax, quantify)
        }
    }
}

/// Partial pivoting
/// Reorders mutable set within the given range so that all items
/// less than or equal to pivot come first, followed by items greater than or equal to pivot.
pub fn fpart(s: &mut [f64], rng: &Range<usize>, pivot: f64) -> usize {
    let mut ltsub = rng.start;
    let mut gtsub = rng.end-1;
    loop {
        while s[ltsub] <= pivot { 
            ltsub += 1;
            if ltsub > gtsub { return ltsub; }; 
        }; 
        while s[gtsub] >= pivot { 
            gtsub -= 1; 
            if gtsub <= ltsub { return ltsub; };
        }; 
        s.swap(ltsub, gtsub);
    } 
}

fn fmin(s: &[f64], rng: Range<usize>) -> f64 {
    let mut min = s[rng.start];
    for &si in s.iter().take(rng.end).skip(rng.start + 1) {
        if si < min {
            min = si;
        };
    }
    min
}

fn fmin2(s: &[f64], rng: Range<usize>) -> f64 {
    let mut min1 = s[rng.start];
    let mut min2 = min1;
    for &si in s.iter().take(rng.end).skip(rng.start + 1) {
        if si < min1 {
            min2 = min1; min1 = si;
        } else if si < min2 { min2 = si; }
    }
    (min1+min2)/2.0
}

fn fmax(s: &[f64], rng: Range<usize>) -> f64 {
    let mut max = s[rng.start];
    for &si in s.iter().take(rng.end).skip(rng.start + 1) {
        if si > max {
            max = si;
        };
    }
    max
}

/// Slower than `range_median`. To be used when data range is unknown.
/// Guesstimates the initial pivot from the first and the last data items.
/// Performance shows greater variability due to luck of this initial guess
/// but on average it is faster than finding the real maximum and minimum.
/// Those are now found during the first data splitting, which saves some comparisons per data item.
pub fn auto_median<T>(set: &[T], quantify: &mut impl FnMut(&T) -> f64) -> f64
where
    T: Copy + PartialOrd,
{
    let n = set.len();
    let mut pivot = 0_f64;
    let fset = set
        .iter()
        .map(|tval| {
            let fval = quantify(tval);
            pivot += fval;
            fval
        })
        .collect::<Vec<f64>>();
    pivot /= n as f64; // using arithmetic mean as the pivot
    if (n & 1) == 1 {
        b_med_odd(fset, 0..n, pivot)
    } else {
        b_med_even(fset, 0..n, pivot)
    }
}

/// Reducing sets iterative median using secant
/// with proportionally subdivided data range as a pivot.
/// Need is a count of items from start of set to expected median position
fn b_med_odd(mut set: Vec<f64>, mut rng: Range<usize>, mut pivot: f64) -> f64 {
    let need = rng.len() / 2; // need as subscript (one less)
    loop {
        let gtsub = fpart(&mut set, &rng, pivot); 
        if need < gtsub {
            rng.end = gtsub;
            if need+1 == gtsub  {
                return fmax(&set, rng.start..gtsub);
            };  
        } else {
            rng.start = gtsub;
            if need == gtsub { 
                return fmin(&set, gtsub..rng.end);
            };
        };
        let newpivot = set.iter().take(rng.end).skip(rng.start).sum::<f64>() / rng.len() as f64;
        if newpivot == pivot { return pivot; } // in equals region
        else { pivot = newpivot; }; 
        // println!("gtpivot {}",pivot);
    }
}

/// Reducing sets median using secant
/// with proportionally subdivided data range as a pivot.
/// Need is a count of items from start of set to anticipated median position
fn b_med_even(mut set: Vec<f64>, mut rng: Range<usize>, mut pivot: f64) -> f64 {
    let need = rng.len() / 2 - 1; 
    loop { 
        let gtsub = fpart(&mut set, &rng, pivot); 
        if need < gtsub { 
            if need+1 == gtsub {
                return (fmax(&set, rng.start..gtsub) +
                fmin(&set, gtsub..rng.end))/2.; 
            };
            rng.end = gtsub;
        } else {  
            if need == gtsub { 
                fmin2(&set,gtsub..rng.end);
            }
            rng.start = gtsub;
        };
        let newpivot = set.iter().take(rng.end).skip(rng.start).sum::<f64>() / rng.len() as f64;
        if newpivot == pivot { return pivot; } // in equals region
        else { pivot = newpivot; };         
    }
}
