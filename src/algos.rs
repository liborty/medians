use core::ops::Range;

/// measure errors in median
pub fn balance<T>(s: &[T], x: f64, quantify: &mut impl FnMut(&T) -> f64) -> i64 {
    let mut above = 0_i64;
    let mut below = 0_i64;
    for si in s {
        let sif = quantify(si);
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

/// Partial pivoting
/// Reorders mutable set within the given range so that all items
/// less than or equal to pivot come first, followed by items greater than or equal to pivot.
pub fn fpart(s: &mut [f64], rng: &Range<usize>, pivot: f64) -> usize {
    let mut ltsub = rng.start;
    let mut gtsub = rng.end - 1;
    loop {
        while s[ltsub] <= pivot {
            ltsub += 1;
            if ltsub > gtsub {
                return ltsub;
            };
        }
        while s[gtsub] >= pivot {
            gtsub -= 1;
            if gtsub <= ltsub {
                return ltsub;
            };
        }
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
            min2 = min1;
            min1 = si;
        } else if si < min2 {
            min2 = si;
        }
    }
    (min1 + min2) / 2.0
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

/// Iterative median, partitioning data range by mean as an estimated pivot.
/// on average this is faster than finding the midpoint between maximum and minimum values.
pub fn autof64(set: &mut [f64]) -> f64 {
    let n = set.len();
    let pivot = set.iter().sum::<f64>()/(n as f64); // using arithmetic mean as the pivot
    if (n & 1) == 1 {
        med_odd(set, 0..n, pivot)
    } else {
        med_even(set, 0..n, pivot)
    } 
}

/// Iterative median, partitioning data range by mean as an estimated pivot.
/// on average this is faster than finding the midpoint between maximum and minimum values.
pub fn auto_median<T>(set: &[T], quantify: &mut impl FnMut(&T) -> f64) -> f64 {
    let n = set.len();
    let mut pivot = 0_f64;
    let mut fset = set
        .iter()
        .map(|tval| {
            let fval = quantify(tval);
            pivot += fval;
            fval
        })
        .collect::<Vec<f64>>();
    pivot /= n as f64; // using arithmetic mean as the pivot
    if (n & 1) == 1 {
        med_odd(&mut fset, 0..n, pivot)
    } else {
        med_even(&mut fset, 0..n, pivot)
    }
}

/// Median of an odd sized set is the central value.
fn med_odd(set: &mut [f64], mut rng: Range<usize>, mut pivot: f64) -> f64 {
    let need = rng.len() / 2; // need as subscript (one less)
    loop {
        let gtsub = fpart(set, &rng, pivot);
        if need < gtsub {
            rng.end = gtsub;
            if need + 1 == gtsub {
                return fmax(set, rng.start..gtsub);
            };
        } else {
            rng.start = gtsub;
            if need == gtsub {
                return fmin(set, gtsub..rng.end);
            };
        };
        let newpivot = set.iter().take(rng.end).skip(rng.start).sum::<f64>() / rng.len() as f64;
        if newpivot == pivot {
            return pivot;
        }
        // in equals region
        else {
            pivot = newpivot;
        };
    }
}

/// Median of an even sized set is half of the sum of the two central values.
pub fn med_even(set: &mut [f64], mut rng: Range<usize>, mut pivot: f64) -> f64 {
    let need = rng.len() / 2 - 1;
    loop {
        let gtsub = fpart(set, &rng, pivot);
        if need < gtsub {
            if need + 1 == gtsub {
                return (fmax(set, rng.start..gtsub) + fmin(set, gtsub..rng.end)) / 2.;
            };
            rng.end = gtsub;
        } else {
            if need == gtsub {
                fmin2(set, gtsub..rng.end);
            }
            rng.start = gtsub;
        };
        let newpivot = set.iter().take(rng.end).skip(rng.start).sum::<f64>() / rng.len() as f64;
        if newpivot == pivot {
            return pivot;
        }
        // in equals region
        else {
            pivot = newpivot;
        };
    }
}
