use core::cmp::{Ordering, Ordering::*};
use indxvec::{Mutops, Vecops};
use std::ops::Range;

/// Mask of the first bit of a u64
pub const FIRST_BIT: u64 = 0x80_00_00_00_00_00_00_00;

/*
/// Copies &[u64] to Vec<u8>
pub fn tobebytes(v: &[u64]) -> Vec<u8> {
    let n = v.len();
    let mut bytes = vec![0u8;8*n];
    for i in 0..n {
        bytes[8*i..][..8].copy_from_slice(&v[i].to_be_bytes());
    }
    bytes
}
*/

/// Partitions `s: &mut [u64]` within range `rng`, using bitmask.  
/// Returns the boundary of the rearranged partitions gtstart, where  
/// `rng.start..gtstart` (may be empty) contains items with zero bit(s) corresponding to bitmask,  
/// `gtstart..rng.end` (may be empty) contains items with one (or more) set bit(s).
pub fn part_binary(s: &mut [u64], rng: &Range<usize>, bitmask: u64) -> usize {
    let mut gtstart = rng.start;
    for &lt in s.iter().take(rng.end).skip(rng.start) {
        if (lt & bitmask) == 0 {
            gtstart += 1;
        } else {
            break;
        };
    }
    for i in gtstart + 1..rng.end {
        if (s[i] & bitmask) == 0 {
            s.swap(gtstart, i);
            gtstart += 1;
        };
    }
    gtstart
}

/// index of middle valued ref of three, using at most three comparisons {
pub fn midof3<T>(
    s: &[T],
    indx0: usize,
    indx1: usize,
    indx2: usize,
    c: &mut impl FnMut(&T, &T) -> Ordering,
) -> usize {
    let (min, max) = if c(&s[indx0], &s[indx1]) == Less {
        (indx0, indx1)
    } else {
        (indx1, indx0)
    };
    let lastref = &s[indx2];
    if c(&s[min], lastref) != Less {
        return min;
    };
    if c(lastref, &s[max]) != Less {
        return max;
    };
    indx2
}

/// index of middle valued ref of three, using at most three comparisons {
fn midof3_refs<T>(
    s: &[&T],
    indx0: usize,
    indx1: usize,
    indx2: usize,
    c: &mut impl FnMut(&T, &T) -> Ordering,
) -> usize {
    let (min, max) = if c(s[indx0], s[indx1]) == Less {
        (indx0, indx1)
    } else {
        (indx1, indx0)
    };
    let lastref = s[indx2];
    if c(s[min], lastref) != Less {
        return min;
    };
    if c(lastref, s[max]) != Less {
        return max;
    };
    indx2
}

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
/// using binary insert k-sort
pub fn best_k<T, F>(s: &[T], k: usize, rng: Range<usize>, c: F) -> &T
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

/// Ref to the minimum item within a range in a slice,  
/// or to the maximum, when invoked with swapped arguments of comparator c: `|a,b| c(b,a)`
pub fn extremum<'a, T>(s: &'a[T], rng: Range<usize>, c: &mut impl FnMut(&T, &T) -> Ordering) -> &'a T {
    let mut min = &s[rng.start];
    for si in s.iter().take(rng.end).skip(rng.start + 1) {
        if c(si, min) == Ordering::Less {
            min = si;
        };
    }
    min
}
/// Refs to the smallest two values within a range in a slice    
/// or to the largest two, when invoked with swapped arguments of comparator c: `|a,b| c(b,a)`.  
/// The first returned item always refers to the extremum.
pub fn best_two<'a, T>(s: &'a[T], rng: Range<usize>, c: &mut impl FnMut(&T, &T) -> Ordering) -> (&'a T, &'a T) {
    let (mut m1, mut m2) = if c(&s[rng.start+1], &s[rng.start]) == Ordering::Less {
        (&s[rng.start+1], &s[rng.start])
    } else {
        (&s[rng.start], &s[rng.start+1])
    };
    for si in s.iter().take(rng.end).skip(rng.start + 2) {
        if c(si,m2) == Ordering::Less {
            if c(si,m1) == Ordering::Less {
                m2 = m1;
                m1 = si;
            } else {
                m2 = si;
            };
        };
    };
    (m1, m2)
}

/// Ref to the minimum item within a range in a slice of refs  
/// or to the maximum, when invoked with swapped arguments of comparator c: `|a,b| c(b,a)`
pub fn extremum_refs<'a, T>(s: &[&'a T], rng: Range<usize>, c: &mut impl FnMut(&T, &T) -> Ordering) -> &'a T {
    let mut m = s[rng.start];
    for si in s.iter().take(rng.end).skip(rng.start + 1) {
        if c(si, m) == Ordering::Less {
            m = si;
        };
    }
    m
}
/// Refs to two smallest items within a range in a slice of refs    
/// or to the largest two, when invoked with swapped arguments of comparator c: `|a,b| c(b,a)`.  
/// The first returned item is always the extremum.
pub fn best_two_refs<'a, T>(
    s: &[&'a T],
    rng: Range<usize>,
    c: &mut impl FnMut(&T, &T) -> Ordering
) -> (&'a T, &'a T) {
    let (mut m1, mut m2) = if c(s[rng.start+1], s[rng.start]) == Ordering::Less {
        (s[rng.start+1], s[rng.start])
    } else {
        (s[rng.start], s[rng.start+1])
    };
    for si in s.iter().take(rng.end).skip(rng.start + 2) {
        if c(si,m2) == Ordering::Less {
            if c(si,m1) == Ordering::Less {
                m2 = m1;
                m1 = si;
            } else {
                m2 = si;
            };
        };
    };
    (m1, m2)
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

/// Odd median of `&u[8]`
pub fn oddmedianu8(s: &[u8]) -> u8 {
    let need = s.len() / 2; // median target position
    let mut histogram = [0_usize; 256];
    let mut cummulator = 0_usize;
    for &u in s.iter() {
        histogram[u as usize] += 1;
    }
    for i in 0_u8..255 {
        let hist = histogram[i as usize];
        if hist == 0 {
            continue;
        };
        cummulator += hist;
        if need < cummulator {
            return i;
        };
    }
    255
}

/// Even medians of `&[u8]`
pub fn evenmedianu8(s: &[u8]) -> (u8,u8) {
    let need = s.len() / 2; // first median target position
    let mut histogram = [0_usize; 256];
    let mut cummulator = 0_usize;
    let mut firstres = true;
    let mut res1 = 255_u8;
    for &u in s.iter() {
        histogram[u as usize] += 1;
    }
    for i in 0_u8..255 {
        let hist = histogram[i as usize];
        if hist == 0 {
            continue;
        };
        cummulator += hist;
        if firstres {
            if cummulator > need {
                return (i, i);
            }; // cummulator exceeds need, found both items
            if cummulator == need {
                // found first item (last in this bucket)
                res1 = i;
                firstres = false;
            }; // while cummulator < need, loop also continues
        } else {
            // the second item is in the first following non-zero bucket
            return (res1,i);
        }; // found the second
    }
    if firstres {
        (255, 255)
    } else {
        (res1, 255)
    }
}

/// Median of odd sized u64 data
pub fn oddmedianu64(s: &mut [u64]) -> &u64 {
    let mut rng = 0..s.len();
    let need = s.len() / 2; // median target position in fully partitioned
    let mut bitval = FIRST_BIT; // set the most significant bit
    loop {
        let gtsub = part_binary(s, &rng, bitval);
        if bitval == 1 {
            // termination of bit iterations: same values left
            if need < gtsub {
                return &s[gtsub - 1];
            };
            return &s[gtsub];
        };
        // well inside lt partition, iterate on it
        if need + 2 < gtsub {
            rng.end = gtsub;
            bitval >>= 1; // next bit
            continue;
        };
        // well inside gt partition, iterate on it
        if need > gtsub + 1 {
            rng.start = gtsub;
            bitval >>= 1; // next bit
            continue;
        };
        // penultimate place in lt partition, find the second maximum
        if need + 2 == gtsub {
            return best_two(s, rng.start..gtsub,&mut |a,b| b.cmp(a)).1;
        };
        // last place in the lt partition, find its maximum
        if need + 1 == gtsub {
            return extremum(s, rng.start..gtsub,&mut |a,b| b.cmp(a));
        };
        // first place in gt partition, find its minimum
        if need == gtsub {
            return extremum(s, gtsub..rng.end, &mut |a,b| a.cmp(b));
        };
        // second place in gt partition, find its second minimum
        return best_two(s, gtsub..rng.end, &mut |a,b| a.cmp(b)).1;
    }
}

/// Median of even sized u64 data
pub fn evenmedianu64(s: &mut [u64]) -> (&u64, &u64) {
    let mut rng = 0..s.len();
    let need = s.len() / 2 - 1; // first median target position
    let mut bitval = FIRST_BIT; // set the most significant bit
    loop {
        let gtsub = part_binary(s, &rng, bitval);
        if bitval == 1 {
            // termination of bit iterations: same values left
            if need < gtsub - 1 {
                return (&s[gtsub - 2], &s[gtsub - 1]);
            };
            if need == gtsub - 1 {
                return (&s[gtsub - 1], &s[gtsub]);
            };
            return (&s[gtsub], &s[gtsub + 1]);
        };
        // well inside lt partition, iterate on it
        if need + 2 < gtsub {
            rng.end = gtsub;
            bitval >>= 1; // next bit
            continue;
        };
        // well inside gt partition, iterate on it
        if need > gtsub {
            rng.start = gtsub;
            bitval >>= 1; // next bit
            continue;
        };
        // penultimate place in lt partition, solution is the maxima pair:
        if need + 2 == gtsub {
            let (m1,m2) = best_two(s, rng.start..gtsub,&mut |a,b| b.cmp(a));
            return (m2,m1)
        };
        // last place in the lt partition, return max of lt and min of gt partitions
        if need + 1 == gtsub {
            return (extremum(s, rng.start..gtsub,&mut |a,b| b.cmp(a)),
                    extremum(s, gtsub..rng.end,&mut |a,b| a.cmp(b)));
        };
        // first place in gt partition, the solution is its minima pair:
        if need == gtsub {
            return best_two(s, gtsub..rng.end,&mut |a,b| a.cmp(b));
        };
    }
}

/// Collects all items that have given byte equal to val
pub fn select(bytes: &[[u8; 8]], byteno: usize, val: u8) -> Vec<[u8; 8]> {
    let mut res = Vec::new();
    for &item in bytes {
        if item[byteno] == val {
            res.push(item);
        };
    }
    res
}

/// Median of odd sized u64 data (recursive)
/// byteno is 0..7 from the most significant (left hand)
pub(super) fn oddmedu64(bytes: &[[u8; 8]], byteno: usize, need: usize) -> u64 {
    let n = bytes.len();
    assert_ne!(n, 0, "oddmedu64 failed to find median");
    if n == 1 {
        return u64::from_be_bytes(bytes[0]);
    };
    if n < 8 { // small number of items remaining, just use sort
        let idx = bytes.isort_refs(0..bytes.len(), |a:&[u8; 8],b| a.cmp(b));
        return u64::from_be_bytes(*idx[need]);
    };
    let mut histogram = [0_usize; 256];
    let mut cummulator = 0_usize;
    for &u in bytes {
        histogram[u[byteno] as usize] += 1;
    };
    let mut medianbyte = 255_u8;
    for (i, &h) in histogram.iter().enumerate() {
        if h == 0_usize { continue };
        cummulator += h;
        if cummulator > need {
            medianbyte = i as u8;
            cummulator -= h; // number of items < medianbyte
            break;
        };
    }
    if byteno == 7 {
        // termination of recursion
        // bytes are all the same now, except the last one
        let mut res = bytes[0]; // so can return any item
        res[7] = medianbyte; // assign the last median byte
        return u64::from_be_bytes(res);
    };
    oddmedu64(
        &select(bytes, byteno, medianbyte),
        byteno + 1,
        need - cummulator,
    ) // tail recursion
}

/*
/// Median of even sized u64 data (recursive)
pub(super) fn evenmedu64(bytes: &[[u8;8]], byteno:usize, need:usize) -> (u64, u64) {
    let n = bytes.len();
    assert_ne!(n,0,"evenmedu64 failed to find median");
    if n == 2 {
        let (m1,m2) = (u64::from_be_bytes(bytes[0]),u64::from_be_bytes(bytes[1]));
        if m2 < m1 { return (m2,m1); } else { return (m1,m2); }; 
    };
    if n < 8 { let sorted_bytes = bytes.isor
    let mut histogram = [0_usize; 256];
    let mut cummulator = 0_usize;
    for &u in bytes {
        histogram[u[byteno] as usize] += 1;
    };
    let mut firstres = true;
    let mut medianbyte = 255_u8;
    for (i,h) in histogram.iter().enumerate() {
        if h == 0_usize { continue };
        cummulator += h;
        if firstres {
            if cummulator > need {
                medianbyte = i as u8;
                cummulator -= h; // number of items < medianbyte
                break;
                return (i as u64, i as u64);
            }; // cummulator exceeds need, found both items
            if need == cummulator {
                // found first item (last in this bucket)
                res1 = i as u64;
                firstres = false;
            };
        } else {
            // the second item is in the first following non-zero bucket
            return (res1, i as u64);
        }; // found the second
    };
    if firstres { (255,255) } else { (res1,255) }
}


        let gtsub = part_binary(s, &rng, bitval);
        if bitval == 1 {
            // termination of bit iterations: same values left
            if need < gtsub - 1 {
                return (s[gtsub - 2], s[gtsub - 1]);
            };
            if need == gtsub - 1 {
                return (s[gtsub - 1], s[gtsub]);
            };
            return (s[gtsub], s[gtsub + 1]);
        };
        // well inside lt partition, iterate on it
        if need + 2 < gtsub {
            rng.end = gtsub;
            bitval >>= 1; // next bit
            continue;
        };
        // well inside gt partition, iterate on it
        if need > gtsub {
            rng.start = gtsub;
            bitval >>= 1; // next bit
            continue;
        };
        // penultimate place in lt partition, solution is the maxima pair:
        if need + 2 == gtsub {
            return max2u64(s, rng.start..gtsub);
        };
        // last place in the lt partition:
        if need + 1 == gtsub {
            return (maxu64(s, rng.start..gtsub), minu64(s, gtsub..rng.end));
        };
        // first place in gt partition, the solution is its minima pair
        if need == gtsub {
            return min2u64(s, gtsub..rng.end);
        };
    }
}
*/

/// Median of odd sized generic data with Odering comparisons by custom closure
pub(super) fn oddmedian_by<'a, T>(
    s: &mut [&'a T],
    c: &mut impl FnMut(&T, &T) -> Ordering,
) -> &'a T {
    let mut rng = 0..s.len();
    let need = s.len() / 2; // first median target position
    loop {
        let mut pivotsub = midof3_refs(s, rng.start, rng.start + need, rng.end - 1, c);
        if rng.len() == 3 {
            return s[pivotsub];
        };
        if rng.len() > 100 {
            let pivotsub2 = midof3_refs(s, rng.start + 1, rng.start + need + 1, rng.end - 2, c);
            let pivotsub3 = midof3_refs(s, rng.start + 2, rng.start + need + 2, rng.end - 3, c);
            pivotsub = midof3_refs(s, pivotsub, pivotsub2, pivotsub3, c);
        }
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
            return best_two_refs(s, rng.start..eqsub, &mut |a, b| c(b, a)).1;
        };
        // last place in the lt partition, solution is its maximum:
        if need + 1 == eqsub {
            // swapped comparator arguments to get maximum
            return extremum_refs(s, rng.start..eqsub, &mut |a, b| c(b, a));
        };
        if need < gtsub {
            // within equals partition, return the pivot
            return pivotref;
        };
        // first place in gt partition, the solution is its minimum
        if need == gtsub {
            return extremum_refs(s, gtsub..rng.end, c);
        };
        // second place in gt partition, the solution is the next minimum
        if need == gtsub + 1 {
            return best_two_refs(s, gtsub..rng.end, c).1;
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
        let mut pivotsub = midof3_refs(s, rng.start, rng.start + need, rng.end - 1, c);
        if rng.len() > 100 {
            let pivotsub2 = midof3_refs(s, rng.start + 1, rng.start + need + 1, rng.end - 2, c);
            let pivotsub3 = midof3_refs(s, rng.start + 2, rng.start + need + 2, rng.end - 3, c);
            pivotsub = midof3_refs(s, pivotsub, pivotsub2, pivotsub3, c);
        };
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
        // penultimate place in lt partition, return its two maxima:
        if need + 2 == eqsub {
            let (m1, m2) = best_two_refs(s, rng.start..eqsub, &mut |a, b| c(b, a));
            return (m2,m1); // in ascending order
        };
        // last place in the lt partition, return its maximum and pivot:
        if need + 1 == eqsub {
            return (extremum_refs(s, rng.start..eqsub, &mut |a, b| c(b, a)), pivotref);
        };
        // fully within equals partition
        if need + 1 < gtsub {
            return (pivotref, pivotref);
        };
        // last place in equals partition, return pivot and minimum of gt partition:
        if need + 1 == gtsub {
            return (pivotref, extremum_refs(s, gtsub..rng.end, c));
        };
        // first place in gt partition, return its two minima
        if need == gtsub {
            return best_two_refs(s, gtsub..rng.end, c);
        };
        // inside gt partition, iterate on it, narrowing the range
        rng.start = gtsub;
    }
}
