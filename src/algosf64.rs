use core::ops::Range;

/// finds a mid value of sample of three
pub fn midof3f64(item1: f64, item2: f64, item3: f64) -> f64
{
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
pub fn partf64(s: &mut [f64], rng: &Range<usize>, pivot: f64) -> (usize, usize, usize)
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

/// Minimum value within a range in a slice
pub fn minf64(s: &[f64], rng: Range<usize>) -> f64
{
    let mut min = s[rng.start];
    for &si in s.iter().take(rng.end).skip(rng.start + 1) {
        if si < min {
            min = si;
        };
    }
    min
}

/// Maximum value within a range in a slice
pub fn maxf64(s: &[f64], rng: Range<usize>) -> f64
{
    let mut max = s[rng.start];
    for &si in s.iter().take(rng.end).skip(rng.start + 1) {
        if si > max {
            max = si;
        };
    }
    max
}

/// two minimum values, in order
pub fn min2f64(s: &[f64], rng: Range<usize>) -> (f64, f64)
{
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
pub fn max2f64(s: &[f64], rng: Range<usize>) -> (f64, f64)
{
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
pub fn med_oddf64(s: &mut[f64]) -> f64
{
    let mut rng = 0..s.len();
    let mut need = s.len() / 2; // need as subscript 
    loop {
        // Take a sample from start,mid,end of data and use their midpoint as pivot 
        let pivot = midof3f64(s[rng.start],s[(rng.start+rng.end)/2],s[rng.end-1]);
        let (gtsub, ltsub, ltend) = partf64(s, &rng, pivot);
        // if rng.len() < 6 { return strict_odd(&s[rng.start..rng.end], need-rng.start); };
        if need + ltsub - rng.start < rng.end {
            // jump over geset, which was placed at the beginning
            need += ltsub - rng.start;
            if need + 2 == ltend {
                return max2f64(s, ltsub..ltend).0;
            };
            if need + 1 == ltend {
                return maxf64(s, ltsub..ltend);
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
            return minf64(s, gtsub..ltsub);
        };
        if need == gtsub + 1 {
            return min2f64(s, gtsub..ltsub).1;
        };
        rng.start = gtsub;
        rng.end = ltsub;
    }
}

/// Both central values of s of even length
pub fn med_evenf64(s: &mut[f64]) -> (f64, f64)
{
    let mut rng = 0..s.len();
    let mut need = s.len() / 2 - 1; // need as subscript - 1 
    loop {
        let pivot = midof3f64(s[rng.start], s[(rng.start + rng.end) / 2], s[rng.end - 1]);
        let (gtsub, ltsub, ltend) = partf64(s, &rng, pivot);
        // print!("{}-{}:{}:{} ", rng.len(), gtsub, ltsub, ltend);
        // if gtsub == rng.start { return strict_even(&s[rng.start..rng.end], need-rng.start); };
        if need + ltsub - rng.start < rng.end {
            // jump over geset, which was placed at the beginning
            need += ltsub - rng.start;
            if need + 2 == ltend {
                // println!("lt max2");
                return max2f64(s, ltsub..ltend);
            };
            // there will always be at least one item equal to pivot and therefore it is the minimum of the ge set
            if need + 1 == ltend {
                // println!("lt max");
                return (maxf64(s, ltsub..ltend), pivot);
            };
            // need is within the equals sets
            if need >= ltend {
                if need < rng.end-1+gtsub-rng.start { return (pivot,pivot); }; 
                if need == rng.end-1+gtsub-rng.start { 
                    if gtsub > rng.start { return (pivot,pivot); }
                    else { return (pivot,minf64(s, gtsub..ltsub)); }
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
            return (pivot, minf64(s, gtsub..ltsub));
        };
        if need == gtsub {
            return min2f64(s, gtsub..ltsub);
        };
        rng.start = gtsub;
        rng.end = ltsub;
        // println!("gt {} {}", rng.start, rng.end);
    }
}
