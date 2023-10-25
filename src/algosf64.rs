use core::ops::Range;

/// finds a mid value of sample of three
pub fn midof3<T:PartialOrd>(item1: T, item2: T, item3: T) -> T {
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

/// Reorders (partitions by pivot) mutable set `s` within subrange `rng`.  
/// Returns `(gtsub,endsub)` after a single efficient pass. 
/// All items in rng.start..gtsub will be less than the pivot. 
/// All items in gtsub..endsub will be greater than the pivot.
/// Items in endsub..rng.end will have arbitrary values but the length 
/// of this subrange is the number of items equal to the pivot.
pub fn part<T: PartialOrd+Copy>(s: &mut [T], pivot:T, rng: &Range<usize>) -> (usize, usize) { 
    let mut ltsub = rng.start;
    let mut endsub = rng.end;
    let mut gtsub = endsub-1;

    loop {
        if gtsub < ltsub { return (ltsub,endsub); };
        if s[gtsub] > pivot {  
            gtsub -= 1;
            continue;
        };
        if s[gtsub] == pivot {
            endsub -= 1;
            s[gtsub] = s[endsub]; // replace eq item with gt item   
            gtsub -= 1;
            continue;
        };
        // now s[gtsub] < pivot
        'ltloop: loop {
            if ltsub == gtsub { return (ltsub+1,endsub); };
            if s[ltsub] < pivot { 
                ltsub += 1;
                continue 'ltloop;
            };
            if s[ltsub] == pivot {
                s[ltsub] = s[gtsub]; // s[gtsub] < pivot, so copy it to the lt set
                endsub -= 1;
                s[gtsub] = s[endsub]; // shift gt set one down into the vacated space
            } else { 
                s.swap(ltsub, gtsub); 
            };
            // now s[ltsub] < pivot < s[gtsub]
            ltsub += 1;
            if ltsub == gtsub { 
                return (gtsub,endsub); // ltsub and gtsub are adjacent and swapped           
            };
            gtsub -= 1; 
        };
    };
}

/// Minimum value within a range in a slice
pub fn min<T:PartialOrd+Copy>(s: &[T], rng: Range<usize>) -> T {
    let mut min = s[rng.start];
    for &si in s.iter().take(rng.end).skip(rng.start + 1) {
        if si < min {
            min = si;
        };
    }
    min
}

/// Maximum value within a range in a slice
pub fn max<T:PartialOrd+Copy>(s: &[T], rng: Range<usize>) -> T {
    let mut max = s[rng.start];
    for &si in s.iter().take(rng.end).skip(rng.start + 1) {
        if si > max {
            max = si;
        };
    }
    max
}

/// two minimum values, in order
pub fn min2<T:PartialOrd+Copy>(s: &[T], rng: Range<usize>) -> (T, T) {
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
pub fn max2<T:PartialOrd+Copy>(s: &[T], rng: Range<usize>) -> (T, T) {
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
pub fn med_odd<T:PartialOrd+Copy>(s: &mut [T]) -> T {
    let mut rng = 0..s.len();
    let mut need = s.len() / 2; // need as subscript
    loop {
        // todo: test here for rng.len() == 3
        // Takes three samples of data and uses their midpoint as a pivot
        let pivot = midof3(s[rng.start], s[(rng.start+rng.end)/2], s[rng.end-1]);
        let (gtsub, endsub) = part(s, pivot, &rng);
        // we are somewhere within ltset, iterate on it
        if need + 2 < gtsub {  
            rng.end = gtsub;
            continue;
        };
        // close to the end of ltset, we have a solution:
        if need + 2 == gtsub { return max2(s, rng.start..gtsub).0; };
        if need + 1 == gtsub { return max(s, rng.start..gtsub); };

        // jump over equals set
        need -= rng.end - endsub;
        if need < gtsub { return pivot; }; // in equals set

        // somewhere within gtset, iterate on it
        if need > gtsub + 1 {
            rng.start = gtsub;
            rng.end = endsub;
            continue;
        };
        // at the beginning of the gt set
        if need == gtsub { return min(s, gtsub..endsub); };
        // else must be second in the gt set
        return min2(s, gtsub..endsub).1;
    };
}

/// Both central values of s of even length
pub fn med_even<T:PartialOrd+Copy>(s: &mut [T]) -> (T, T) {
    let mut rng = 0..s.len();
    let mut need = s.len() / 2 - 1; // need as subscript - 1
    loop {
        let pivot = midof3(s[rng.start], s[(rng.start+rng.end)/2], s[rng.end-1]);
        let (gtsub, endsub) = part(s, pivot, &rng);
        // we are somewhere within ltset, iterate on it
        if need + 2 < gtsub {  
            rng.end = gtsub;
            continue;
        };
        // need is close to the end of ltset, we have a solution
        if need + 2 == gtsub { return max2(s, rng.start..gtsub); };
        // there will always be at least one item equal to pivot and therefore it is the minimum of the ge set
        if need + 1 == gtsub { return ( max(s, rng.start..gtsub), pivot ); };

        // jump over equals set
        need -= rng.end - endsub;
        if need + 1 < gtsub { return (pivot,pivot); }; // in equals set 
        if need + 1 == gtsub { return ( pivot, min(s, gtsub..endsub)); }; // last of equals set

        // somewhere within gt set, iterate on it
        if need > gtsub + 1 {
            rng.start = gtsub;
            rng.end = endsub;
            continue;
        };
        // at the beginning of the gt set
        if need == gtsub { return min2(s, gtsub..endsub); }; 
    }
}
