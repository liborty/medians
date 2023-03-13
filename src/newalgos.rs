use core::ops::Range;

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
