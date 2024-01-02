use core::cmp::{Ordering, Ordering::*};
use std::ops::Range;

/// Constructs ref wrapped `Vec<&T>` from `&[T] in rng`
pub fn ref_vec<T>(v: &[T], rng: Range<usize>) -> Vec<&T> {
    v.iter().take(rng.end).skip(rng.start).collect()
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

/// Partitions `s: &mut [&T]` within range `rng`, using comparator `c`.  
/// The first item `s[rng.start]` is assumed to be the pivot.   
/// The three rearranged partitions are demarcated by eqstart,gtstart, where:  
/// `rng.start..eqstart` (may be empty) contains refs to items lesser than the pivot,  
/// `gtstart-eqstart` is the number (>= 1) of items equal to the pivot (values within this subrange are undefined)  
/// `gtstart..rng.end` (may be empty) contains refs to items greater than the pivot.
pub fn part<T>(
    s: &mut [&T],
    rng: &Range<usize>,
    c: &mut impl FnMut(&T, &T) -> Ordering,
) -> (usize, usize) {
    // get pivot from the first location
    let pivot = s[rng.start];
    let mut eqstart = rng.start;
    let mut gtstart = eqstart + 1;
    for t in rng.start + 1..rng.end {
        match c(s[t], pivot) {
            Less => {
                s[eqstart] = s[t];
                eqstart += 1;
                s[t] = s[gtstart];
                gtstart += 1;
            }
            Equal => {
                s[t] = s[gtstart];
                gtstart += 1;
            }
            Greater => (),
        }
    }
    (eqstart, gtstart)
}

/// Odd median of `&[u8]`
pub fn oddmedianu8(s: &[u8]) -> f64 {
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
pub fn evenmedianu8(s: &[u8]) -> f64 {
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


/// Median of odd sized generic data with Odering comparisons by custom closure
pub fn oddmedian_by<'a, T>(s: &mut [&'a T], c: &mut impl FnMut(&T, &T) -> Ordering) -> &'a T {
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
        let (eqsub, gtsub) = part(s, &rng, c);
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
pub fn evenmedian_by<'a, T>(
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
        let (eqsub, gtsub) = part(s, &rng, c);
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
