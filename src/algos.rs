use core::ops::Range;

/// measure errors in median
pub fn balance<T>(s: &[T], x: f64, quantify: &mut impl FnMut(&T) -> f64) -> i64 {
    let mut bal = 0_i64;
    let mut eq = 0_i64;
    for si in s {
        let sif = quantify(si);
        if sif > x {
            bal += 1;
        } else if sif < x {
            bal -= 1;
        } else {
            eq += 1;
        };
    }
    if bal == 0 {
        return 0;
    };
    if bal.abs() <= eq {
        return 0;
    };
    1
}

/// Simple (partial) pivoting
/// Reorders mutable set within the given range so that all items
/// less than or equal to pivot come first, followed by items greater than or equal to pivot.
pub fn spart(s: &mut [f64], mut ltsub: usize, mut gtsub: usize, pivot: f64) -> usize {
    gtsub -= 1;
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

/*
/// Pivoting: reorders mutable set s within ltsub..gtsub so that all items
/// less than pivot come first, followed by items greater than or equal to pivot.
/// Also returns the count of equal items in the second part.
pub fn fpart(s: &mut [f64], mut ltsub: usize, mut gtsub: usize, pivot: f64) -> (usize, usize) {
    let mut eq = 0_usize;
    gtsub -= 1;
    loop {
        if s[ltsub] < pivot {
            ltsub += 1;
            if ltsub > gtsub {
                return (ltsub, eq);
            } else {
                continue;
            };
        };
        if s[ltsub] == pivot {
            eq += 1;
            if gtsub == ltsub {
                return (ltsub, eq);
            };
            s.swap(ltsub, gtsub);
            gtsub -= 1;
            continue;
        };
        'gtloop: loop {
            if s[gtsub] > pivot {
                if gtsub == ltsub {
                    return (ltsub, eq);
                };
                gtsub -= 1;
                continue 'gtloop;
            };
            if s[gtsub] == pivot {
                eq += 1;
                if gtsub == ltsub {
                    return (ltsub, eq);
                };
                gtsub -= 1;
                continue 'gtloop;
            };
            break 'gtloop;
        }
        s.swap(ltsub, gtsub);
        ltsub += 1;
        gtsub -= 1;
        if ltsub > gtsub {
            return (ltsub, eq);
        };
    }
}
*/

fn fmin(s: &[f64], rng: Range<usize>) -> f64 {
    let mut min = s[rng.start];
    for &si in s.iter().take(rng.end).skip(rng.start + 1) {
        if si < min {
            min = si;
        };
    }
    min
}

/// two minimum values, in order
pub fn fmin2(s: &[f64], rng: Range<usize>) -> (f64, f64) {
    let mut min1 = s[rng.start];
    let mut min2 = f64::MAX;
    for &si in s.iter().take(rng.end).skip(rng.start + 1) {
        if si < min1 {
            min2 = min1;
            min1 = si;
        } else if si < min2 {
            min2 = si;
        }
    }
    (min1, min2)
}

/// two maximum values, in order
pub fn fmax2(s: &[f64], rng: Range<usize>) -> (f64, f64) {
    let mut max1 = s[rng.start];
    let mut max2 = f64::MIN;
    for &si in s.iter().take(rng.end).skip(rng.start + 1) {
        if si > max1 {
            max2 = max1;
            max1 = si;
        } else if si > max2 {
            max2 = si;
        }
    }
    (max2, max1)
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

/// Median of an odd sized set is the central value.
pub fn med_odd(set: &mut [f64]) -> f64 {
    let mut rng = 0..set.len();
    let need = set.len() / 2; // need as subscript
    loop {
        let pivot = set.iter().take(rng.end).skip(rng.start).sum::<f64>() / rng.len() as f64;
        let gtsub = spart(set, rng.start, rng.end, pivot);
        if gtsub == rng.start || gtsub == rng.end {
            return pivot;
        };
        if need < gtsub {
            rng.end = gtsub;
            if need == gtsub - 2 {
                return fmax2(set, rng.start..gtsub).0;
            };
            if need == gtsub - 1 {
                return fmax(set, rng.start..gtsub);
            };
        } else {
            rng.start = gtsub;
            if need == gtsub {
                return fmin(set, gtsub..rng.end);
            };
            if need == gtsub + 1 {
                return fmin2(set, gtsub..rng.end).1;
            };
        };
    }
}

/// Median of an even sized set is half of the sum of the two central values.
pub fn med_even(set: &mut [f64]) -> f64 {
    let mut rng = 0..set.len();
    let need = set.len() / 2 - 1; // need as subscript - 1
    loop {
        let pivot = set.iter().take(rng.end).skip(rng.start).sum::<f64>() / rng.len() as f64;
        let gtsub = spart(set, rng.start, rng.end, pivot);
        if gtsub == rng.start || gtsub == rng.end {
            return pivot;
        };
        if need < gtsub {
            if need == gtsub - 2 {
                let (max1, max2) = fmax2(set, rng.start..gtsub);
                return (max1 + max2) / 2.;
            };
            if need == gtsub - 1 {
                return (fmax(set, rng.start..gtsub) + fmin(set, gtsub..rng.end)) / 2.;
            };
            rng.end = gtsub;
        } else {
            if need == gtsub {
                let (min1, min2) = fmin2(set, gtsub..rng.end);
                return (min1 + min2) / 2.;
            };
            rng.start = gtsub;
        };
    }
}
