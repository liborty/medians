use crate::Me;
use core::cmp::{Ordering, Ordering::*};
use std::ops::Range;

const FSIGN: u64 = 0x8000_0000_0000_0000;

/// Scan a slice of f64s for being free of NANs
pub fn no_nans(v: &[f64]) -> bool {
    for &f in v {
        if f.is_nan() {
            return false;
        };
    }
    true
}

/// Copies a slice of f64s, removing any NANs from it.
/// It is advisable to test with `non_nans` first, as there may be none
pub fn scrub_nans(v: &[f64]) -> Vec<f64> {
    v.iter()
        .filter_map(|&f| if f.is_nan() { None } else { Some(f) })
        .collect::<Vec<f64>>()
    //        Err(merror("nan", "median_checked: NaN encountered in data"))
}

/// Converts f64s to &u64s, so that sort order is preserved
pub fn to_u64s(v: &[f64]) -> Result<Vec<u64>, Me> {
    v.iter()
        .map(|f| {
            let mut u: u64 = f.to_bits();
            if (u >> 63) == 1 {
                u ^= FSIGN;
            } else {
                u = !u;
            };
            Ok(u)
        })
        .collect()
}

/// Converts f64s to &u64s, so that sort order is preserved
pub fn to_f64s(v: &[u64]) -> Vec<f64> {
    v.iter()
        .map(|&u| {
            if (u >> 63) == 1 {
                f64::from_bits(!u)
            } else {
                f64::from_bits(u ^ FSIGN)
            }
        })
        .collect()
}

/// Constructs ref wrapped `Vec<&T>` from `&[T] in rng`
pub fn ref_vec<T>(v: &[T], rng: Range<usize>) -> Vec<&T> {
    v.iter().take(rng.end).skip(rng.start).collect()
}

/// Builds Vec<T> from refs in Vec<&T> (inverse of ref_vec())
pub fn deref_vec<T>(v: &[&T]) -> Vec<T> 
where T:Clone {
    v.iter().map(|&x| x.clone()).collect()
}

/// Insert logsort of refs (within range).
/// Use for large types T, they do not get copied.
/// Pass in reversed comparator for descending sort.
pub fn isort_refs<T>(s: &[T], rng: Range<usize>, c: impl Fn(&T, &T) -> Ordering) -> Vec<&T> {
    match rng.len() {
    0 => return vec![],
    1 => return vec![&s[rng.start];1],
    _ => ()
    };
    // build a mutable vec of refs
    let mut rv:Vec<&T> = s.iter().take(rng.end).skip(rng.start).collect();
    if c(rv[rng.start + 1], rv[rng.start]) == Less {
        rv.swap(rng.start, rng.start + 1);
    };
    for i in rng.start+2..rng.end {
        // first two already swapped
        if c(rv[i], rv[i - 1]) != Less {
            continue;
        } // rv[i] item is already in order
        let thisref = rv[i];
        let insert = match rv[rng.start..i - 1].binary_search_by(|&j| c(j, thisref)) {
            Ok(ins) => ins + 1,
            Err(ins) => ins,
        };
        rv.copy_within(insert..i, insert + 1);
        rv[insert] = thisref;
    }
    rv
}

/// middle valued ref out of three, at most three comparisons
pub fn midof3<'a, T>(item1: &'a T, item2: &'a T, item3: &'a T, c: &mut impl FnMut(&T, &T) -> Ordering) -> &'a T
{
    let (min, max) = if c(item2,item1) == Less {
        (item2, item1)
    } else {
        (item1, item2)
    };
    if c(min,item3) != Less {
        return min;
    };
    if c(item3,max) != Less {
        return max;
    };
    item3
}

/*
/// pivot estimate as recursive mid of mids of three
pub fn midofmids<'a, T>(s: &[&T], rng: Range<usize>, c: &mut impl FnMut(&T, &T) -> Ordering) -> &'a T
where
    T: PartialOrd,
{
    if s.len() < 3 { return s[0]; };
    for i in 
    let (min, max) = if *item1 <= *item2 {
        (item1, item2)
    } else {
        (item2, item1)
    };
    if *item3 <= *min {
        return min;
    };
    if *item3 <= *max {
        return item3;
    };
    max
}

/// Mid two values of four refs. Makes four comparisons
fn mid2(
    idx0: usize,
    idx1: usize,
    idx2: usize,
    idx3: usize,
    c: &mut impl FnMut(usize, usize) -> Ordering,
) -> (usize,usize) {
    let (min1,max1) = if c(idx0, idx1) == Less { (idx0,idx1) } else { (idx1,idx0) };
    let (min2,max2) = if c(idx2, idx3) == Less { (idx2,idx3) } else { (idx3,idx2) };
    let mid1 = if c(min1, min2) == Less { min2 } else { min1 };
    let mid2 = if c(max1, max2) == Less { max1 } else { max2 };
    (mid1,mid2)
}
*/

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

/*
/// Maps general `quantify` closure to self, converting the type T -> U
pub fn quant_vec<T, U>(v: &[T], quantify: impl Fn(&T) -> U) -> Vec<U> {
    v.iter().map(quantify).collect::<Vec<U>>()
}
*/

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
    };
    if bal == 0 {
        return 0;
    };
    if bal.abs() <= eq {
        return 0;
    };
    1
}

/// Partitions `s: &mut [&T]` within range `rng` by pivot, which is the first item: `s[rng.start]`.  
/// The three resulting partitions are divided by eqstart,gtstart, where:  
/// `rng.start..eqstart` (may be empty) contains refs to items lesser than the pivot,  
/// `gtstart-eqstart` is the number (>= 1) of items equal to the pivot, values within this subrange are undefined,  
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
    let mut resbucket = 0_u8;
    for &u in s.iter() {
        histogram[u as usize] += 1;
    };
    for &hist in histogram.iter() {
        cummulator += hist;
        if need < cummulator { break; };
        resbucket += 1;
    };
    resbucket as f64
}

/// Even median of `&[u8]`
pub fn evenmedianu8(s: &[u8]) -> f64 {
    let mut need = s.len() / 2; // first median target position
    let mut histogram = [0_usize; 256];
    let mut cummulator = 0_usize;
    let mut resbucket = 0u8;
    let mut res = f64::MIN;
    for &u in s.iter() {
        histogram[u as usize] += 1;
    }
    for &hist in histogram.iter() {
        cummulator += hist; 
        if need < cummulator { break; }; // cummulator exceeds need, found at least two items
        if need == cummulator { // the last (possibly only) item in this bucket
            if res == f64::MIN { // is the first median
                res = resbucket as f64; // save it
                need += 1; // next look for the second one (in the following buckets)
            } else { break; }; // this item is already the second median
        };
        resbucket += 1;
        continue;
    };
    if  res == f64::MIN { resbucket as f64 } else { (res + resbucket as f64)/ 2.0 } 
}

/// Odd median of `&[u16]`
pub fn oddmedianu16(s: &[u16]) -> f64 {
    let need = s.len() / 2; // median target position
    let mut histogram = [0_usize; 65536];
    let mut cummulator = 0_usize;
    let mut resbucket = 0_u16;
    for &u in s.iter() {
        histogram[u as usize] += 1;
    };
    for &hist in histogram.iter() {
        cummulator += hist;
        if need < cummulator { break; };
        resbucket += 1;
    };
    resbucket as f64
}

/// Even median of `&[u16]`
pub fn evenmedianu16(s: &[u16]) -> f64 {
    let mut need = s.len() / 2; // first median target position
    let mut histogram = [0_usize; 65536];
    let mut cummulator = 0_usize;
    let mut resbucket = 0_u16;
    let mut res = f64::MIN;
    for &u in s.iter() {
        histogram[u as usize] += 1;
    }
    for &hist in histogram.iter() {
        cummulator += hist; 
        if need < cummulator { break; }; // cummulator exceeds need, found at least two items
        if need == cummulator { // the last (possibly only) item in this bucket
            if res == f64::MIN { // is the first median
                res = resbucket as f64; // save it
                need += 1; // next look for the second one (in the following buckets)
            } else { break; }; // this item is already the second median
        };
        resbucket += 1;
        continue;
    };
    if  res == f64::MIN { resbucket as f64 } else { (res + resbucket as f64)/ 2.0 } 
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
