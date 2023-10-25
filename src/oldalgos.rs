use core::ops::Range;
use core::fmt::Debug;
use core::ops::{Deref, Neg};

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

/// Partitions by pivot, in a single pass, mutable set `s` within `rng`.
/// s[rng.end-1] is used as the ref of the pivot. Any prior pivot
/// selection scheme must place it there.
/// The three partitions are divided by returned `(gtstart,eqstart)`, where:
/// `rng.start..gtstart` (may be empty) contains refs to items less than the pivot,
/// `gtstart..eqstart` (may be empty) contains refs to items greater than the pivot,
/// `eqstart..rng.end` contains zero or more undefined refs, ending with the pivot.
/// The total number of items equal to the pivot is: `rng.end-eqstart` > 0.
pub fn part<T>(
    s: &mut [&T],
    rng: &Range<usize>,
    c: &mut impl FnMut(&T, &T) -> Ordering,
) -> (usize, usize) {
    let pivot = s[rng.end - 1];
    let mut ltlast = rng.start;
    let mut gtstart = rng.end - 2;
    let mut eqstart = rng.end - 1;
    loop {
        match c(pivot, s[gtstart]) {
            Less => {
                if ltlast == gtstart {
                    return (gtstart, eqstart);
                };
                gtstart -= 1;
                continue;
            }
            Equal => {
                eqstart -= 1;
                s[gtstart] = s[eqstart]; // shifts gt range one down
                if ltlast == gtstart {
                    return (ltlast, eqstart);
                };
                gtstart -= 1;
                continue;
            }
            Greater => (), // *s[gtstart] < *pivot
        };
        'lt: loop {
            match c(s[ltlast], pivot) {
                Less => {
                    if gtstart == ltlast {
                        return (gtstart + 1, eqstart);
                    };
                    ltlast += 1;
                    continue 'lt;
                }
                Equal => {
                    s[ltlast] = s[gtstart]; // making ltlast lt
                    eqstart -= 1;
                    s[gtstart] = s[eqstart]; // shifts gt range one down
                }
                Greater => {
                    s.swap(ltlast, gtstart);
                }
            };
            ltlast += 1;
            gtstart -= 1;
            if gtstart < ltlast {
                return (gtstart + 1, eqstart);
            };
            break 'lt;
        }
    }
}

/// Partitions in-place mutable set `s` within range `rng`,
/// using the given (ref of a) pivot.
/// The two partitions are divided by gtstart, where:  
/// for all items x in `rng.start..gtstart`: `x < pivot`,  
/// for all items y in `gtstart..gtend`: `y > pivot`,
/// `rng.end-gtend` is the number of items equal to pivot.
/// The actual values in this last subrange are undefined.
pub fn part<T>(s: &mut [T], rng: &Range<usize>, pivot: &T, mut c: impl FnMut(&T, &T) -> Ordering) -> usize { 
    let mut gtstart = rng.end;
    for ltend in rng.start..gtstart {
        if c(pivot,&s[ltend]) == Less { 
            gtstart -= 1;
            swap(&s[ltend],&s[gtstart]); 
         };
    };
    gtstart
}

/// Partitions in a single pass mutable set `s` within range `rng`,
/// into three non empty subranges. Uses middle pair of a sample of four data values
/// as `min_pivot` and `max_pivot`, moved to the last two positions. 
/// The partitions are divided by `(gtstart,midstart)`, where:  
/// for all items x in `rng.start..gtstart`: `x < min_pivot`,  
/// for all items y in `gtstart..midstart`: `y > max_pivot`,   
/// for all items z in midstart..rng.end `min_pivot <= z <= max_pivot`.
pub fn part<T>(s: &mut [T], rng: &Range<usize>, mut c: impl FnMut(&T, &T) -> Ordering) 
    -> (usize, usize) { 
    let mut midstart = rng.end-2; 
    let mut ltend = rng.start;
    let mut gtstart = midstart-1;
    // sort in place four samples of data
    insert_sort(s, &[ ltend, midstart, midstart+1, gtstart ], c);
    let mut minpivot = &s[midstart];
    let mut maxpivot = &s[midstart+1]; 
    loop {
        gtstart -= 1;
        if gtstart < ltend { return (ltend,midstart); }; 
        if c(maxpivot,&s[gtstart]) == Less { continue; };
        if c(&s[gtstart],minpivot) != Less { 
            midstart -= 1;
            s.swap(gtstart,midstart); 
            continue;
        }; 
        // s[gtstart] < minpivot
        loop {
            ltend += 1; 
            if gtstart < ltend { return (ltend,midstart); };  
            if c(&s[ltend],minpivot) == Less { continue; };
            if c(maxpivot,&s[ltend]) == Less { 
                s.swap(ltend,gtstart);
            } else {
                let savedmidval = s[ltend];
                s[ltend] = s[gtstart]; 
                midstart -= 1;
                s[gtstart] = s[midstart];  
                s[midstart] = savedmidval;
            };
            break;
        };
    };
}

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

/// Fast partial pivoting.
/// Reorders mutable set within the given range so that all items less than or equal to pivot come first,  
/// followed by items greater than or equal to pivot.
pub fn spart<T>(s: &mut [T], mut ltsub: usize, mut gtsub: usize, pivot: &T) -> usize
where
    T: PartialOrd,
{
    gtsub -= 1;
    loop {
        while s[ltsub] <= *pivot {
            ltsub += 1;
            if ltsub > gtsub {
                return ltsub;
            };
        }
        while s[gtsub] >= *pivot {
            gtsub -= 1;
            if gtsub <= ltsub {
                return ltsub;
            };
        }
        s.swap(ltsub, gtsub);
        ltsub += 1;
        gtsub -= 1;
        if gtsub <= ltsub {
            return ltsub;
        };
    }
}

/// Pivoting: reorders mutable set s within ltsub..gtsub so that all items equal to pivot come first.  
/// Can be used after `part` on geset for total partition: `[ltset,eqset,gtset]`
pub fn eqpart<T>(s: &mut [T], mut ltsub: usize, mut gtsub: usize, pivot: &T) -> usize
where
    T: PartialOrd,
{
    gtsub -= 1;
    assert!(ltsub < gtsub);
    loop {
        while s[ltsub] == *pivot {
            ltsub += 1;
            if ltsub > gtsub {
                return ltsub;
            };
        }
        while s[gtsub] != *pivot {
            gtsub -= 1;
            if gtsub == ltsub {
                return gtsub;
            };
        }
        s.swap(ltsub, gtsub);
        ltsub += 1;
        gtsub -= 1;
        if ltsub >= gtsub {
            return ltsub;
        };
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

fn fmax2(s: &[f64], rng: Range<usize>) -> f64 {
    let mut max1 = s[rng.start];
    let mut max2 = max1;
    for &si in s.iter().take(rng.end).skip(rng.start + 1) {
        if si > max1 {
            max2 = max1;
            max1 = si;
        } else if si > max2 {
            max2 = si;
        }
    }
    (max1 + max2) / 2.0
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
/// Need is the subscript of the item required.
/// For median, it should be the midpoint.
pub fn med_odd(set: &mut [f64]) -> f64 {
    let mut firsttime = true;
    let mut max = 0_f64;
    let mut min = 0_f64;
    let n = set.len();
    let mut rngstart = 0_usize;
    let mut rngend = n;
    let need = n / 2;
    let mut pivot = set.iter().sum::<f64>() / (n as f64); // initially the mean
    println!();
    loop {
        let (gtsub, eq) = fpart(set, rngstart, rngend, pivot);
        print!("{gtsub} ");
        if gtsub == rngend {
            return set[rngend - 1];
        };
        if gtsub == rngstart {
            return set[rngstart];
        };
        if need < gtsub {
            rngend = gtsub;
            if need + 1 == gtsub {
                return fmax(set, rngstart..rngend);
            };
            max = pivot;
            if firsttime {
                min = fmin(set, rngstart..rngend);
                firsttime = false;
            };
        } else {
            rngstart = gtsub;
            if need < gtsub + eq {
                return pivot;
            }; // in equal set
            min = pivot;
            if firsttime {
                max = fmax(set, rngstart..rngend);
                firsttime = false;
            };
        };
        pivot = min + (max - min) * ((need - rngstart) as f64) / ((rngend - rngstart) as f64);
    }
}

/// Median of an even sized set is half of the sum of the two central values.
pub fn med_even(set: &mut [f64]) -> f64 {
    let mut firsttime = true;
    let mut max = 0_f64;
    let mut min = 0_f64;
    let n = set.len();
    let mut rngstart = 0_usize;
    let mut rngend = n;
    let need = n / 2 - 1;
    let mut pivot = set.iter().sum::<f64>() / (n as f64); // initially the mean
    loop {
        // print!("{pivot} ");
        let (gtsub, eq) = fpart(set, rngstart, rngend, pivot);
        if need < gtsub {
            //rngend = gtsub;
            if rngend == gtsub {
                return pivot;
            } else {
                rngend = gtsub;
            }
            if need + 2 == gtsub {
                return fmax2(set, rngstart..rngend);
            };
            if need + 1 == gtsub {
                return (fmax(set, rngstart..rngend) + fmin(set, rngstart..rngend)) / 2.;
            };
            max = pivot;
            if firsttime {
                min = fmin(set, rngstart..rngend);
                firsttime = false;
            };
        } else {
            if need + 1 < gtsub + eq {
                return pivot;
            }; // in equal set
               //rngstart = gtsub;
            if rngstart == gtsub {
                return pivot;
            } else {
                rngstart = gtsub;
            };
            if need == gtsub + eq {
                return fmin2(set, rngstart..rngend);
            }
            min = pivot;
            if firsttime {
                max = fmax(set, rngstart..rngend);
                firsttime = false;
            };
        };
        pivot = min + (max - min) * ((need - rngstart) as f64) / ((rngend - rngstart) as f64);
        // pivot = (max+min)/2.;
    }
}
