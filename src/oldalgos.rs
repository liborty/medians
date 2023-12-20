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
