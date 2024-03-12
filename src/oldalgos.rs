use core::ops::Range;
use core::fmt::Debug;
use core::ops::{Deref, Neg};

const FSIGN: u64 = 0x8000_0000_0000_0000;

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

/// Copies a slice of f64s, removing any NANs from it.
/// It is advisable to test with `non_nans` first, as there may be none
pub fn scrub_nans(v: &[f64]) -> Vec<f64> {
    v.iter()
        .filter_map(|&f| if f.is_nan() { None } else { Some(f) })
        .collect::<Vec<f64>>()
}

/// Converts one f64, including NaNs etc., to u64, maintaining order
pub fn to_u64(f: f64) -> u64 { 
    let u: u64 = f.to_bits();
    if (u >> 63) == 1 { u^FSIGN } else { !u }
}

/// Converts slice of f64s, including NaNs etc., to Vec<&u64>, maintaining order
pub fn to_u64s(v: &[f64]) -> Vec<u64> { 
    v.iter().map(|&f| to_u64(f)).collect()
}

/// Converts slice of f64s to u64s, so that sort order is preserved, cleaning NANs
pub fn to_clean_u64s(v: &[f64]) -> Vec<u64> {
    v.iter()
        .filter_map(|&f| if f.is_nan() { None } else { Some(to_u64(f)) } )
        .collect()
}

/// Converts f64 to u64 (inverse of `to_u64`).
pub fn to_f64(u: u64) -> f64 {
    f64::from_bits( if (u >> 63) == 1 { !u } else { u^FSIGN } )
}

/// Converts u64s to f64s (inverse of `to_u64s`).
pub fn to_f64s(v: &[u64]) -> Vec<f64> {
    v.iter().map(|&u| to_f64(u)).collect() 
}

/// Maps general `quantify` closure to self, converting the type T -> U
pub fn quant_vec<T, U>(v: &[T], quantify: impl Fn(&T) -> U) -> Vec<U> {
    v.iter().map(quantify).collect::<Vec<U>>()
}

/// middle valued ref out of three, at most three comparisons
pub fn midof3<'a, T>(
    item1: &'a T,
    item2: &'a T,
    item3: &'a T,
    c: &mut impl FnMut(&T, &T) -> Ordering,
) -> &'a T {
    let (min, max) = if c(item2, item1) == Less {
        (item2, item1)
    } else {
        (item1, item2)
    };
    if c(min, item3) != Less {
        return min;
    };
    if c(item3, max) != Less {
        return max;
    };
    item3
}

/// pivot estimate as recursive mid of mids of three
pub fn midofmids<'a, T>(s: &[&T], rng: Range<usize>, c: &mut impl FnMut(&T, &T) -> Ordering) -> &'a T
where
    T: PartialOrd,
{
    if s.len() < 3 { return s[0]; }; 
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

/// Odd median of `&[u16]`
pub fn oddmedianu16(s: &[u16]) -> f64 {
    let need = s.len() / 2; // median target position
    let mut histogram = [0_usize; 65536];
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

/// Even median of `&[u16]`
pub fn evenmedianu16(s: &[u16]) -> f64 {
    let need = s.len() / 2; // first median target position
    let mut histogram = [0_usize; 65536];
    let mut cummulator = 0_usize;
    let mut firstres = true;
    let mut res = f64::MIN;
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

/// The following defines Ord<T> struct which is a T that implements Ord.
/// This boilerplate makes any wrapped T:PartialOrd, such as f64, into Ord
#[derive(Clone,Copy,Debug)]
/// Wrapper type for Ord f64
pub struct Ordf64(f64);

impl<T: std::fmt::Display> std::fmt::Display for Ordf64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self.0}")
    }
}
impl Ordf64 {
    pub fn new(value: f64) -> Self {
        Ordf64(value)
    }
}
impl Deref for Ordf64 {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Neg for Ordf64 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Ordf64::new(-*self)
    }
}
impl PartialOrd for Ordf64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((*self).cmp(other))
    }
}
impl PartialEq for Ordf64 {
    fn eq(&self, rhs: &Ordf64) -> bool {
        (*self).cmp(rhs) == Equal
    }
}
impl Ord for Ordf64 {
    fn cmp(&self, other: &Self) -> Ordering {
        (*self).total_cmp(other)
    }
}
impl Eq for Ordf64 {}

/// Turn v:&[f64] to Vec<Ordf64>
pub fn ord_vec(v: &[f64]) -> Vec<Ordf64> {
    v.iter().map(|f| Ordf64(f)).collect::<Vec<Ordf64>>()
}

pub struct Ordered<T>(pub T);

impl<T: std::fmt::Display > std::fmt::Display for Ordered<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let Ordered(x) = self;
        write!(f, "{x}" )
    }
}

impl<T> Deref for Ordered<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Ordered<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T:PartialOrd> PartialEq for Ordered<T> {
    fn eq(&self, other: &Self) -> bool {
        if **self < **other { return false; };
        if **self > **other { return false; };
        true
    }
}

impl<T:PartialOrd> PartialOrd for Ordered<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if **self < **other { return Some(Less) };
        if **self > **other { return Some(Greater) };
        None
    }
}

impl<T:PartialOrd> Ord for Ordered<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        if **self < **other { return Less };
        if **self > **other { return Greater };
        Equal
    }
}

impl<T:PartialOrd> Eq for Ordered<T> {}

impl<T> From<T> for Ordered<T> {
    fn from(f:T) -> Self {
        Ordered(f)
    }
}

/// Turn v:&[T] into Vec<Ordered<&T>>
pub fn ord_vec<T>(v: &[T]) -> Vec<Ordered<&T>> {
    v.iter().map(Ordered).collect::<Vec<Ordered<&T>>>()
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

/// Subscripts of minimum and maximum locations within rng in slice s
pub fn minmax<T>(
    s: &[T],
    rng: Range<usize>,
    c: &mut impl FnMut(&T, &T) -> Ordering,
) -> (usize, usize) {
    let mut min = rng.start;
    let mut max = min + 1;
    if c(&s[max], &s[min]) == Ordering::Less {
        min = max;
        max = rng.start;
    };
    for i in rng.start + 2..rng.end {
        let ti = &s[i];
        if c(ti, &s[min]) == Ordering::Less {
            min = i;
        } else if c(&s[max], ti) == Ordering::Less {
            max = i;
        };
    }
    (min, max)
}
*/


/// Finds the item at sort index k using the heap method
/// To find the median, use k = self.len()/2
fn strict_odd(&self, k:usize) -> Result<&T,Me>
{
    let os = ord_vec(self);
    let s = os.as_slice();
    if let Some(&m) = s.smallest_k(k+1).peek() { Ok(m) }
        else { Err(merror("other","strict_odd: failed to peek smallest_k heap")) }
}    
/// Finds the two items from sort index k, using the heap method.  
/// To find both even medians, use k = self.len()/2
fn strict_even(&self, k:usize) -> Result<(&T, &T),Me>
{
    let os = ord_vec(self);
    let s = os.as_slice();
        let mut heap = s.smallest_k(k+1); 
        let Some(m1) = heap.pop() else { 
            return Err(merror("other","strict_even: failed to pop smallest_k heap")); };
        let Some(&m2) = heap.peek() else { 
            return Err(merror("other","strict_even: failed to peek smallest_k heap")); };
    Ok((m2,m1))
}
