use core::cmp::{Ordering, Ordering::*};
use std::ops::Range;
use indxvec::Mutops;
use crate::{Me,merror};

/// Scan a slice of f64s for NANs
pub fn nans(v: &[f64]) -> bool {
    for &f in v {
        if f.is_nan() {
            return true;
        };
    }
    false
}

/// kth item from rng (ascending or descending, depending on `c`)
pub fn best1_k<T,F>(s: &[T], k: usize, rng: Range<usize>, c: F) -> &T
   where
       F: Fn(&T, &T) -> Ordering,
   {
       let n = rng.len();
       assert!((k > 0) & (k <= n));
       let mut k_sorted: Vec<&T> = s.iter().skip(rng.start).take(k).collect();
       k_sorted.sort_unstable_by(|&a, &b| c(a, b));
       let mut k_max = k_sorted[k - 1];
       for si in s.iter() {
           if c(si, k_max) == Less {
               let insert_pos = match k_sorted.binary_search_by(|j| c(j, si)) {
                   Ok(ins) => ins + 1,
                   Err(ins) => ins,
               };
               k_sorted.insert(insert_pos, si);
               k_sorted.pop();
               k_max = k_sorted[k - 1];
           };
       }
       k_max
   }

/// Index of the middling value of four refs. Makes only three comparisons
fn middling(
    idx0: usize,
    idx1: usize,
    idx2: usize,
    idx3: usize,
    c: &mut impl FnMut(usize, usize) -> Ordering,
) -> usize {
    let max1 = if c(idx0, idx1) == Less { idx1 } else { idx0 };
    let max2 = if c(idx2, idx3) == Less { idx3 } else { idx2 };
    if c(max1, max2) == Less {
        max1
    } else {
        max2
    }
}

/// Minimum value within a range in a slice
/// Finds maximum, when arguments of c are swapped in the function call: `|a,b| c(b,a)`
pub fn min<'a, T>(s: &[&'a T], rng: Range<usize>, c: &mut impl FnMut(&T, &T) -> Ordering) -> &'a T {
    let mut min = s[rng.start];
    for si in s.iter().take(rng.end).skip(rng.start + 1) {
        if c(si, min) == Ordering::Less {
            min = si;
        };
    }
    min
}

/// Two minimum values within rng in slice s.  
/// Finds maxima, when invoked with arguments of c swapped: `|a,b| c(b,a)`.  
/// The first returned item refers to the primary minimum/maximum.
pub fn min2<'a, T>(
    s: &[&'a T],
    rng: Range<usize>,
    c: &mut impl FnMut(&T, &T) -> Ordering,
) -> (&'a T, &'a T) {
    let (mut min1, mut min2) = if c(s[rng.start + 1], s[rng.start]) == Ordering::Less {
        (s[rng.start + 1], s[rng.start])
    } else {
        (s[rng.start], s[rng.start + 1])
    };
    for si in s.iter().take(rng.end).skip(rng.start + 2) {
        if c(si, min1) == Ordering::Less {
            min2 = min1;
            min1 = si;
        } else if c(si, min2) == Ordering::Less {
            min2 = si;
        }
    }
    (min1, min2)
}

/// measure errors from centre (for testing)
/// requires quantising to f64 for accuracy
pub fn qbalance<T>(s: &[T], centre: &f64, q: impl Fn(&T) -> f64) -> i64 {
    let mut bal = 0_i64;
    let mut eq = 0_i64;
    for si in s {
        match &q(si).total_cmp(centre) {
            Less => bal -= 1,
            Greater => bal += 1,
            _ => eq += 1,
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

/// Odd median of `&[u8]`
fn oddmedianu8(s: &[u8]) -> f64 {
    let need = s.len() / 2; // median target position
    let mut histogram = [0_usize; 256];
    let mut cummulator = 0_usize;
    let mut res = 0_f64;
    for &u in s.iter() {
        histogram[u as usize] += 1;
    }
    for (i, &hist) in histogram.iter().enumerate() {
        if hist == 0 {
            continue;
        };
        cummulator += hist;
        if need < cummulator {
            res = i as f64;
            break;
        };
    }
    res
}

/// Even median of `&[u8]`
fn evenmedianu8(s: &[u8]) -> f64 {
    let need = s.len() / 2; // first median target position
    let mut histogram = [0_usize; 256];
    let mut cummulator = 0_usize;
    let mut firstres = true;
    let mut res = 0_f64;
    for &u in s.iter() {
        histogram[u as usize] += 1;
    }
    for (i, &hist) in histogram.iter().enumerate() {
        if hist == 0 {
            continue;
        };
        cummulator += hist;
        if firstres {       
            if need < cummulator {  res = i as f64; break; }; // cummulator exceeds need, found both items
            if need == cummulator { // found first item (last in this bucket)
                res = i as f64;       
                firstres = false;
                continue; // search for the second item
            };
        } else { // the second item is in the first following non-zero bucket
            res += i as f64;
            res /= 2.0;
            break;
        }; // found the second
    };
    res
}

/// Median of primitive type u8 by fast radix search
pub fn medianu8(s:&[u8]) -> Result<f64, Me> {
    let n = s.len();
    match n {
        0 => return merror("size", "median: zero length data")?,
        1 => return Ok(s[0] as f64),
        2 => return Ok((s[0] as f64 + s[1] as f64) / 2.0),
        _ => (),
    };
    if (n & 1) == 1 {
        Ok(oddmedianu8(s))
    } else {
        Ok(evenmedianu8(s))
    }
}

/// Median of odd sized generic data with Odering comparisons by custom closure
pub(super) fn oddmedian_by<'a, T>(s: &mut [&'a T], c: &mut impl FnMut(&T, &T) -> Ordering) -> &'a T {
    let mut rng = 0..s.len();
    let need = s.len() / 2; // median target position in fully partitioned set
    loop {
        let pivotsub = middling(
            rng.start,
            rng.start + 1,
            rng.end - 2,
            rng.end - 1,
            &mut |a, b| c(s[a], s[b]),
        );
        if pivotsub != rng.start {
            s.swap(rng.start, pivotsub);
        };
        let pivotref = s[rng.start];
        let (eqsub, gtsub) = <&mut [T]>::part(s, &rng, c);
        // well inside lt partition, iterate on it
        if need + 2 < eqsub {
            rng.end = eqsub;
            continue;
        };
        // penultimate place in lt partition, solution:
        if need + 2 == eqsub {
            // swapped comparator arguments to get penultimate maximum
            return min2(s, rng.start..eqsub, &mut |a, b| c(b, a)).1;
        };
        // last place in the lt partition, solution is its maximum:
        if need + 1 == eqsub {
            // swapped comparator arguments to get maximum
            return min(s, rng.start..eqsub, &mut |a, b| c(b, a));
        };
        if need < gtsub {
            // within equals partition, return the pivot
            return pivotref;
        };
        // first place in gt partition, the solution is its minimum
        if need == gtsub {
            return min(s, gtsub..rng.end, c);
        };
        // second place in gt partition, the solution is the next minimum
        if need == gtsub + 1 {
            return min2(s, gtsub..rng.end, c).1;
        };
        // well inside gt partition, iterate on it
        rng.start = gtsub;
    }
}

/// Median of even sized generic data with Odering comparisons by custom closure
pub(super) fn evenmedian_by<'a, T>(
    s: &mut [&'a T],
    c: &mut impl FnMut(&T, &T) -> Ordering,
) -> (&'a T, &'a T) {
    let mut rng = 0..s.len();
    let need = s.len() / 2 - 1; // median target position in fully partitioned set
    loop {
        let pivotsub = middling(
            rng.start,
            rng.start + 1,
            rng.end - 2,
            rng.end - 1,
            &mut |a, b| c(s[a], s[b]),
        );
        if pivotsub != rng.start {
            s.swap(rng.start, pivotsub);
        };
        let pivotref = s[rng.start];
        let (eqsub, gtsub) = <&mut [T]>::part(s, &rng, c);
        // well inside lt partition, iterate on it narrowing the range
        if need + 2 < eqsub {
            rng.end = eqsub;
            continue;
        };
        // penultimate place in lt partition, solution:
        if need + 2 == eqsub {
            // swapping comparison arguments to get two maxima
            let (m1, m2) = min2(s, rng.start..eqsub, &mut |a, b| c(b, a));
            return (m2, m1);
        };
        // last place in the lt partition, solution:
        if need + 1 == eqsub {
            // swapped comparison arguments to get maximum
            return (min(s, rng.start..eqsub, &mut |a, b| c(b, a)), pivotref);
        };
        // fully within equals partition
        if need + 1 < gtsub {
            return (pivotref, pivotref);
        };
        // last place in equals partition
        if need + 1 == gtsub {
            return (pivotref, min(s, gtsub..rng.end, c));
        };
        // first place in gt partition, the solution are its two minima
        if need == gtsub {
            return min2(s, gtsub..rng.end, c);
        };
        // inside gt partition, iterate on it, narrowing the range
        rng.start = gtsub;
    }
}
