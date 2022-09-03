use indxvec::{here, Vecops};

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
        if  sif > x { above += 1; }
        else if sif < x { below += 1; };
        };
    // println!("{}, {}",below,above); 
    if below == above { return 0; };
    let diff = (above-below).abs();
    if diff < (s.len() as i64 - above - below) { return 0; };
    1    
}

/// Median of a &[T] slice by sorting
/// Slow 'ground truth' for comparisons
/// Sorts its mutable slice argument as a side effect
/// # Example
/// ```
/// use medians::algos::naive_median;
/// let mut v = vec![1_u8,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
/// let res = naive_median(&mut v);
/// assert_eq!(res,8_f64);
/// ```
pub fn naive_median<T>(s: &[T]) -> f64
where
    T: Copy + PartialOrd,
    f64: From<T>,
{
    let n = s.len();
    if n == 0 {
        panic!("{} empty vector!", here!());
    };
    if n == 1 {
        return f64::from(s[0]);
    };
    if n == 2 {
        return (f64::from(s[0]) + f64::from(s[1])) / 2.0;
    };
    let mut sf = s.to_vec(); // copy to avoid mutating data
    // Using the fastest available Rust unstable mutable sort
    sf.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = s.len() / 2; // midpoint (floors odd sizes)
                           // Even/odd size test (for even size the median is in-between)
    if (n & 1) == 0 {
        (f64::from(sf[mid - 1]) + f64::from(sf[mid])) / 2.0
    } else {
        f64::from(sf[mid])
    } // s is odd
}

/// Used by r_median
fn part<T>(s: &[T], pivot: f64) -> (Vec<T>, Vec<T>)
where
    T: Copy,
    f64: From<T>,
{
    let mut ltset = Vec::new();
    let mut gtset = Vec::new();
    for &f in s {
        if f64::from(f) < pivot {
            ltset.push(f);
        } else {
            gtset.push(f);
        };
    }
    (ltset, gtset)
}

/// Recursive Reducing Median
pub fn r_median<T>(set: &[T]) -> f64
where
    T: Copy + PartialOrd,
    f64: From<T>,
{
    // let s = tof64(set); // makes an f64 copy
    let n = set.len();
    // starting pivot
    let (min, max) = set.minmaxt();
    let pivot = (f64::from(min) + f64::from(max)) / 2.;
    // passing min max just to stop recomputing it
    if (n & 1) == 0 {
        r_med_even(set, n / 2, pivot, f64::from(min), f64::from(max))
    } else {
        r_med_odd(set, n / 2 + 1, pivot, f64::from(min), f64::from(max))
    }
}

/// Reducing sets median using `minmax()` and secant
/// with proportionally subdivided data range as a pivot.
/// Need is a count of items from start of set to anticipated median position
fn r_med_odd<T>(set: &[T], need: usize, pivot: f64, setmin: f64, setmax: f64) -> f64
where
    T: PartialOrd + Copy,
    f64: From<T>,
{
    if need == 1 {
        return setmin;
    };
    let n = set.len();
    if need == n {
        return setmax;
    };

    let (ltset, gtset) = part(set, pivot);
    let ltlen = ltset.len();
    let gtlen = gtset.len();
    // println!("Need: {}, Pivot {:5.3}, partitions: {}, {}",need,pivot,ltlen,gtlen);

    match need {
        1 => f64::from(ltset.mint()),
        x if x < ltlen => {
            let max = f64::from(ltset.maxt());
            if setmin == max {
                return f64::from(ltset[0]);
            }; // all equal, done
            let newpivot = setmin + (need as f64) * (max - setmin) / (ltlen as f64);
            r_med_odd(&ltset, need, newpivot, setmin, max)
        }
        x if x == ltlen => f64::from(ltset.maxt()),
        x if x == ltlen + 1 => f64::from(gtset.mint()),
        x if x == n => f64::from(gtset.maxt()),
        _ => {
            // need > ltlen
            let newneed = need - ltlen;
            let min = f64::from(gtset.mint());
            if min == setmax {
                return f64::from(gtset[0]);
            }; // all equal, done
            let newpivot = min + (setmax - min) * (newneed as f64) / (gtlen as f64);
            r_med_odd(&gtset, newneed, newpivot, min, setmax)
        }
    }
}

/// Reducing sets median using `minmax()` and secant
/// with proportionally subdivided data range as a pivot.
/// Need is a count of items from start of set to anticipated median position
fn r_med_even<T>(set: &[T], need: usize, pivot: f64, setmin: f64, setmax: f64) -> f64
where
    T: PartialOrd + Copy,
    f64: From<T>,
{
    let n = set.len();
    let (ltset, gtset) = part(set, pivot);
    let ltlen = ltset.len();
    let gtlen = gtset.len();
    // println!("Need: {}, Pivot {}, partitions: {}, {}",need,pivot,ltlen,gtlen);
    match need {
        // 1 => ltset.mint(),
        x if x < ltlen => {
            let max = f64::from(ltset.maxt());
            if setmin == max {
                return f64::from(ltset[0]);
            }; // all equal, done
            let newpivot = setmin + (need as f64) * (max - setmin) / (ltlen as f64);
            r_med_even(&ltset, need, newpivot, setmin, max)
        }
        x if x == ltlen => (f64::from(ltset.maxt()) + f64::from(gtset.mint())) / 2., // at the boundary
        x if x == n => f64::from(gtset.maxt()),
        _ => {
            // need > ltlen
            let newneed = need - ltlen;
            let min = f64::from(gtset.mint());
            if min == setmax {
                return f64::from(gtset[0]);
            }; // all equal, done
            let newpivot = min + (newneed as f64) * (setmax - min) / (gtlen as f64);
            r_med_even(&gtset, newneed, newpivot, min, setmax)
        }
    }
}
