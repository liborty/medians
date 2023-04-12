use core::ops::Range;

/// finds a mid value of sample of three
pub fn midof3f64(item1: f64, item2: f64, item3: f64) -> f64 {
    let (min, max) = if item1 <= item2 {
        (item1, item2)
    } else {
        (item2, item1)
    };
    if item3 <= min {
        return min;
    };
    if item3 <= max {
        return item3;
    };
    max
}

/// Partitions mutable set s within rng by pivot value. 
/// The reordering is done in a single pass, with minimal comparisons.   
/// Returns a triple of subscripts to new s: (gtstart, mid, ltend).  
/// Items equal to pivot are either before gtstart or starting from ltend.  
/// Items greater than pivot are in range (gtstart,mid) 
/// Items lesser than pivot are in range (mid,ltend). 
/// Any of these four resulting sub-slices may be empty.
pub fn partf64(s: &mut [f64], rng: &Range<usize>, pivot: f64) -> (usize, usize, usize) {
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

/// Minimum value within a range in a slice
pub fn minf64(s: &[f64], rng: Range<usize>) -> f64 {
    let mut min = s[rng.start];
    for &si in s.iter().take(rng.end).skip(rng.start + 1) {
        if si < min {
            min = si;
        };
    }
    min
}

/// Maximum value within a range in a slice
pub fn maxf64(s: &[f64], rng: Range<usize>) -> f64 {
    let mut max = s[rng.start];
    for &si in s.iter().take(rng.end).skip(rng.start + 1) {
        if si > max {
            max = si;
        };
    }
    max
}

/// two minimum values, in order
pub fn min2f64(s: &[f64], rng: Range<usize>) -> (f64, f64) {
    let (mut min1, mut min2) = if s[rng.start + 1] < s[rng.start] {
        (s[rng.start + 1], s[rng.start])
    } else {
        (s[rng.start], s[rng.start + 1])
    };
    for &si in s.iter().take(rng.end).skip(rng.start + 2) {
        if si < min1 {
            min2 = min1;
            min1 = si;
        } else if si < min2 {
            min2 = si;
        }
    }
    (min1, min2)
}

/// two maximum values, in order
pub fn max2f64(s: &[f64], rng: Range<usize>) -> (f64, f64) {
    let (mut max1, mut max2) = if s[rng.start + 1] > s[rng.start] {
        (s[rng.start + 1], s[rng.start])
    } else {
        (s[rng.start], s[rng.start + 1])
    };
    for &si in s.iter().take(rng.end).skip(rng.start + 2) {
        if si > max1 {
            max2 = max1;
            max1 = si;
        } else if si > max2 {
            max2 = si;
        }
    }
    (max2, max1)
}

/// Median of slice s of odd length
pub fn med_oddf64(s: &mut [f64]) -> f64 {
    let mut rng = 0..s.len();
    let mut need = s.len() / 2; // need as subscript
    loop {
        // Take a sample from start,mid,end of data and use their midpoint as a pivot
        let pivot = midof3f64(s[rng.start], s[(rng.start + rng.end) / 2], s[rng.end - 1]);
        let (gtsub, ltsub, ltend) = partf64(s, &rng, pivot);
        // somewhere within ltset, iterate on it
        if need + ltsub - rng.start + 2 < ltend {
            need += ltsub - rng.start;
            rng.start = ltsub;
            rng.end = ltend;
            continue;
        }
        // need is within reach of the end of ltset, we have a solution:
        if need + ltsub - rng.start < rng.end {
            // jump over geset, which was placed at the beginning
            need += ltsub - rng.start;
            if need + 2 == ltend {
                return max2f64(s, ltsub..ltend).0;
            };
            if need + 1 == ltend {
                return maxf64(s, ltsub..ltend);
            };
            // else need is in the end equals set (need >= ltend)
            return pivot;
        };
        // geset was placed at the beginning, so reduce need by leset
        need -= rng.end - ltsub;
        // somewhere within gtset, iterate on it
        if need > gtsub + 1 {
            rng.start = gtsub;
            rng.end = ltsub;
            continue;
        }
        // need is within reach of the beginning of the ge set, we have a solution:
        // does it fall within the first equals set?
        if need < gtsub {
            return pivot;
        };
        if need == gtsub {
            return minf64(s, gtsub..ltsub);
        };
        // else need == gtsub + 1
        return min2f64(s, gtsub..ltsub).1;
    }
}

/// Both central values of s of even length
pub fn med_evenf64(s: &mut [f64]) -> (f64, f64) {
    let mut rng = 0..s.len();
    let mut need = s.len() / 2 - 1; // need as subscript - 1
    loop {
        let pivot = midof3f64(s[rng.start], s[(rng.start + rng.end) / 2], s[rng.end - 1]);
        let (gtsub, ltsub, ltend) = partf64(s, &rng, pivot);
        // somewhere within ltset, iterate on it
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
                return max2f64(s, ltsub..ltend);
            };
            // there will always be at least one item equal to pivot and therefore it is the minimum of the ge set
            if need + 1 == ltend {
                return (maxf64(s, ltsub..ltend), pivot);
            };
            // need is within the equals sets (need >= ltend)
            let eqend = rng.end - 1 + gtsub - rng.start;
            if need < eqend {
                return (pivot, pivot);
            };
            if need == eqend {
                if gtsub > rng.start {
                    return (pivot, pivot);
                } else {
                    return (pivot, minf64(s, gtsub..ltsub));
                }
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
            return (pivot, pivot);
        };
        // last of the first equals set
        if need+1 == gtsub {
            return (pivot, minf64(s, gtsub..ltsub));
        };
        // first of the gtset
        if need == gtsub {
            return min2f64(s, gtsub..ltsub);
        };
    }
}
