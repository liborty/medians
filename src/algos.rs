use core::ops::Range;

/// Turn v:&[T] to Vec<Ordered<&T>>
//pub fn ord_vec<T>(v: &[T]) -> Vec<Ordered<&T>> {
//    v.iter().map(Ordered).collect::<Vec<Ordered<&T>>>()
//}

/// Maps general `quantify` closure to self, converting the type T -> U
pub fn quant_vec<T, U>(v: &[T], quantify: &mut impl FnMut(&T) -> U) -> Vec<U> {
    v.iter().map(quantify).collect::<Vec<U>>()
}

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

/// finds a mid value of some three locations
pub fn midof3<'a, T>(item1: &'a T, item2: &'a T, item3: &'a T) -> &'a T
where
    T: PartialOrd,
{
    //let item1 = &s[sub1];
    //let item2 = &s[sub2];
    //let item3 = &s[sub3];

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

/// Partitioning by pivot value: reorders mutable set s within rng and divides it into four subsets.  
/// All items greater than or equal to pivot come before `mid`, followed by all items less than or equal to pivot.
/// Additionally, items equal to pivot are swapped either before gt set or after lt set (their ascending order positions).
/// Which get swapped to the beginning and which to the end depends on where they happen to be encountered.
/// The processing is done in-place, in a single pass, with minimal comparisons.  
/// Returns a triple of subscripts to s: (gtstart, mid, ltend).
/// mid marks the end of the gtset (gtend) and the beginning of the ltset (ltstart).  
/// The length of the first eqset is gtstart-rng.start,  
/// the length of gtset is mid-gtstart,  
/// the length of ltset is ltend-mid,  
/// and the length of the second eqset is rng.end-ltend.
/// Any of these lengths may be zero.
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
            if gtsub > startsub {
                s.swap(startsub, gtsub);
            };
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
                if ltsub < endsub {
                    s.swap(ltsub, endsub);
                };
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

fn min<'a, T>(s: &[&'a T], rng: Range<usize>) -> &'a T
where
    T: PartialOrd,
{
    let mut min = &s[rng.start];
    for si in s.iter().take(rng.end).skip(rng.start + 1) {
        if *si < *min {
            min = si;
        };
    }
    min
}

fn max<'a, T>(s: &[&'a T], rng: Range<usize>) -> &'a T
where
    T: PartialOrd,
{
    let mut max = &s[rng.start];
    for si in s.iter().take(rng.end).skip(rng.start + 1) {
        if *si > *max {
            max = si;
        };
    }
    max
}

/// two minimum values, in order
fn min2<'a, T>(s: &[&'a T], rng: Range<usize>) -> (&'a T, &'a T)
where
    T: PartialOrd,
{
    let (mut min1, mut min2) = if s[rng.start + 1] < s[rng.start] {
        (&s[rng.start + 1], &s[rng.start])
    } else {
        (&s[rng.start], &s[rng.start + 1])
    };
    for si in s.iter().take(rng.end).skip(rng.start + 2) {
        if *si < *min1 {
            min2 = min1;
            min1 = si;
        } else if *si < *min2 {
            min2 = si;
        }
    }
    (min1, min2)
}

/// two maximum values, in order
fn max2<'a, T>(s: &[&'a T], rng: Range<usize>) -> (&'a T, &'a T)
where
    T: PartialOrd,
{
    let (mut max1, mut max2) = if s[rng.start + 1] > s[rng.start] {
        (&s[rng.start + 1], &s[rng.start])
    } else {
        (&s[rng.start], &s[rng.start + 1])
    };
    for si in s.iter().take(rng.end).skip(rng.start + 2) {
        if *si > *max1 {
            max2 = max1;
            max1 = si;
        } else if *si > *max2 {
            max2 = si;
        }
    }
    (max2, max1)
}

/// Median of an odd sized set: the value at midpoint (if sorted)
pub fn med_odd<T>(set: &[T]) -> &T
where
    T: PartialOrd,
{
    let mut rng = 0..set.len();
    let mut need = set.len() / 2; // need as subscript
    let mut s: Vec<&T> = set.iter().collect();
    loop {
        // Take a sample from start,mid,end of data and use their midpoint as pivot
        let pivot = midof3(s[rng.start], s[(rng.start + rng.end) / 2], s[rng.end - 1]);
        let (gtsub, ltsub, ltend) = part(&mut s, &rng, pivot);
        // if rng.len() < 6 { return strict_odd(&s[rng.start..rng.end], need-rng.start); };
        if need + ltsub - rng.start < rng.end {
            // jump over geset, which was placed at the beginning
            need += ltsub - rng.start;
            if need + 2 == ltend {
                return max2(&s, ltsub..ltend).0;
            };
            if need + 1 == ltend {
                return max(&s, ltsub..ltend);
            };
            // need is in the end equals set
            if need >= ltend {
                return pivot;
            };
            rng.start = ltsub;
            rng.end = ltend;
            continue;
        };
        // geset, is at the beginning, so reduce need by leset
        need -= rng.end - ltsub;
        // need is in the first equals set
        if need < gtsub {
            return pivot;
        };
        if need == gtsub {
            return min(&s, gtsub..ltsub);
        };
        if need == gtsub + 1 {
            return min2(&s, gtsub..ltsub).1;
        };
        rng.start = gtsub;
        rng.end = ltsub;
    }
}

/// Median of an even sized set is half of the sum of the two central values.
pub fn med_even<T>(set: &[T]) -> (&T, &T)
where
    T: PartialOrd
{
    let mut rng = 0..set.len();
    let mut need = set.len() / 2 - 1; // need as subscript - 1
    let mut s: Vec<&T> = set.iter().collect();
    loop {
        let pivot = midof3(s[rng.start], s[(rng.start + rng.end) / 2], s[rng.end - 1]);
        let (gtsub, ltsub, ltend) = part(&mut s, &rng, pivot);
        // print!("{}-{}:{}:{} ", rng.len(), gtsub, ltsub, ltend);
        // if gtsub == rng.start { return strict_even(&s[rng.start..rng.end], need-rng.start); };
        if need + ltsub - rng.start < rng.end {
            // jump over geset, which was placed at the beginning
            need += ltsub - rng.start;
            if need + 2 == ltend {
                // println!("lt max2");
                return max2(&s, ltsub..ltend);
            };
            // there will always be at least one item equal to pivot and therefore it is the minimum of the ge set
            if need + 1 == ltend {
                // println!("lt max");
                return (max(&s, ltsub..ltend), pivot);
            };
            // need is within the equals sets
            if need >= ltend {
                if need < rng.end-1+gtsub-rng.start { return (pivot,pivot); }; 
                if need == rng.end-1+gtsub-rng.start { 
                    if gtsub > rng.start { return (pivot,pivot); }
                    else { return (pivot,min(&s, gtsub..ltsub)); }
                }
            } 
            rng.start = ltsub;
            rng.end = ltend;
            // println!("lt {} {}", rng.start, rng.end);
            continue; 
        };
        // geset, is at the beginning, so reduce need by leset
        need -= rng.end - ltsub;
        // need is in the first equals set
        if need+1 < gtsub {
            return (pivot,pivot);
        }; 
        if need+1 == gtsub {
            return (pivot, min(&s, gtsub..ltsub));
        };
        if need == gtsub {
            return min2(&s, gtsub..ltsub);
        };
        rng.start = gtsub;
        rng.end = ltsub;
        // println!("gt {} {}", rng.start, rng.end);
    }
}