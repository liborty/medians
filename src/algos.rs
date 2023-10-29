use crate::Me;
use core::cmp::{Ordering, Ordering::*};
use std::ops::Range;

const FSIGN: u64 = 0x8000_0000_0000_0000;

/// Tests a slice of f64s for the presence of NANs
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

/// Insert logsort of refs within [&T].
/// Use for large types T, as they do not get copied here.
/// Pass in reversed comparator for descending sort.
pub fn isort_refs<T>(s: &mut [&T], rng: Range<usize>, c: impl Fn(&T, &T) -> Ordering) {
    if s.len() < 2 {
        return;
    };
    if c(s[rng.start + 1], s[rng.start]) == Less {
        s.swap(rng.start, rng.start + 1);
    };
    for i in rng.start + 2..rng.end {
        // first two already swapped
        if c(s[i], s[i - 1]) != Less {
            continue;
        } // s[i] item is already in order
        let thisref = s[i];
        let insert = match s[rng.start..i - 1].binary_search_by(|&j| c(j, thisref)) {
            Ok(ins) => ins + 1,
            Err(ins) => ins,
        };
        s.copy_within(insert..i, insert + 1);
        s[insert] = thisref;
    }
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

/*
/// Partitions mutable set s within rng by pivot value.
/// The reordering is done in a single pass, with minimal comparisons.
/// Returns a triple of subscripts to new s: `(gtstart, mid, ltend)`.
/// The count of items equal to pivot is: `(gtstart-rng.start) + (rng.end-ltend)`.
/// Items greater than pivot are in range (gtstart,mid)
/// Items less than pivot are in range (mid,ltend).
/// Any of these four resulting sub-slices may be empty.
pub fn part<T>(s: &mut [&T], rng: &Range<usize>, pivot: &T) -> (usize, usize, usize)
where
    T: PartialOrd,
{
    let mut startsub = rng.start;
    let mut gtsub = startsub;
    let mut ltsub = rng.end - 1;
    let mut endsub = rng.end - 1;
    loop {
        while s[gtsub] > pivot {
            if gtsub == ltsub {
                return (startsub, 1 + gtsub, 1 + endsub);
            };
            gtsub += 1;
        }
        if s[gtsub] == pivot {
            s[gtsub] = s[startsub];
            if gtsub == ltsub {
                return (1 + startsub, 1 + gtsub, 1 + endsub);
            };
            startsub += 1;
            gtsub += 1;
            continue;
        };
        'lt: loop {
            if s[ltsub] < pivot {
                ltsub -= 1;
                // s[gtsub] here is already known to be lt pivot, so assign it to lt set
                if gtsub >= ltsub {
                    return (startsub, gtsub, 1 + endsub);
                };
                continue 'lt;
            }
            if s[ltsub] == pivot {
                s[ltsub] = s[endsub];
                ltsub -= 1;
                if gtsub >= ltsub {
                    return (startsub, gtsub, endsub);
                };
                endsub -= 1;
                continue 'lt;
            };
            break 'lt;
        }
        s.swap(ltsub, gtsub);
        gtsub += 1;
        ltsub -= 1;
        if gtsub > ltsub {
            return (startsub, gtsub, 1 + endsub);
        };
    }
}

/// Median of odd sized generic data with Odering comparisons by custom closure
pub fn oddmedian_by<'a, T>(s: &mut [&'a T], c: &mut impl FnMut(&T, &T) -> Ordering) -> &'a T {
{
    let mut rng = 0..set.len();
    let mut need = set.len() / 2; // need as subscript
    let mut s: Vec<&T> = set.iter().collect();
    loop {
        // Take a sample from start,mid,end of data and use their midpoint as a pivot
        let pivot = part_sort4(&mut[s[rng.start],s[rng.start+1],s[rng.end-2],s[rng.end-1]],c);
        let (gtsub, ltsub, ltend) = part(&mut s, &rng, pivot);
        // somewhere within ltset, iterate on it
        if need + ltsub - rng.start + 2 < ltend {
            need += ltsub - rng.start;
            rng.start = ltsub;
            rng.end = ltend;
            continue;
        }
        // when need is within reach of the end of ltset, we have a solution:
        if need + ltsub - rng.start < rng.end {
            // jump over geset, which was placed at the beginning
            need += ltsub - rng.start;
            if need + 2 == ltend {
                return max2(&s, ltsub..ltend).0;
            };
            if need + 1 == ltend {
                return max(&s, ltsub..ltend);
            };
            // else need is in the end equals set (need >= ltend)
            return pivot;
        };
        // geset was placed at the beginning, so reduce need by leset
        need -= rng.end - ltsub;
        // somewhere within gtset, iterate on it
        if need > gtsub+1 {
            rng.start = gtsub;
            rng.end = ltsub;
            continue;
        }
        // here need is within reach of the beginning of the ge set, we have a solution:
        // does it fall within the first equals set?
        if need < gtsub {
            return pivot;
        };
        if need == gtsub {
            return min(&s, gtsub..ltsub);
        };
        // else need == gtsub + 1
        return min2(&s, gtsub..ltsub).1;
    }
}

/// Median of even sized generic data with Odering comparisons by custom closure
pub fn evenmedian_by<'a, T>(
    s: &mut [&'a T],
    c: &mut impl FnMut(&T, &T) -> Ordering,
) -> (&'a T, &'a T) {
    let mut rng = 0..set.len();
    let mut need = set.len() / 2 - 1; // need as subscript - 1
    let mut s: Vec<&T> = set.iter().collect();
    loop {
        let pivot = part_sort4(&mut[s[rng.start],s[rng.start+1],s[rng.end-2],s[rng.end-1]],c);
        // let pivot = midof3(s[rng.start], s[(rng.start + rng.end) / 2], s[rng.end - 1]);
        let (gtsub, ltsub, ltend) = part(&mut s, &rng, pivot);
        // need falls somewhere within ltset, iterate on it
        if need + ltsub - rng.start + 2 < ltend {
                need += ltsub - rng.start;
                rng.start = ltsub;
                rng.end = ltend;
                continue;
        };
        // if need is within reach of the end of ltset, we have a solution:
        if need + ltsub - rng.start < rng.end {
            // jump over geset, which was placed at the beginning
            need += ltsub - rng.start;
            if need + 2 == ltend {
                return max2(&s, ltsub..ltend);
            };
            // there will always be at least one item equal to pivot and therefore it is the minimum of the ge set
            if need + 1 == ltend {
                return (max(&s, ltsub..ltend), pivot);
            };
            // need is within the equals sets (need >= ltend)
            let eqend = rng.end-1+gtsub-rng.start;
            if need < eqend { return (pivot,pivot); };
            if need == eqend {
                if gtsub > rng.start { return (pivot,pivot); }
                else { return (pivot,min(&s, gtsub..ltsub)); }
            };
        };
        // geset was placed at the beginning, so reduce need by leset
        need -= rng.end - ltsub;
        // somewhere within gtset, iterate on it
        if need+1 > gtsub {
            rng.start = gtsub;
            rng.end = ltsub;
            continue;
        };
        // need is within reach of the beginning of the ge set, we have a solution:
        // is need in the first equals set?
        if need+1 < gtsub {
            return (pivot,pivot);
        };
        // last of the first equals set
        if need+1 == gtsub {
            return (pivot, min(&s, gtsub..ltsub));
        };
        // first of the gtset
        if need == gtsub {
            return min2(&s, gtsub..ltsub);
        };
    }
}
*/
