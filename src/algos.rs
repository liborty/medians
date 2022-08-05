use indxvec::{here,Vecops};

/// Median of a &[T] slice by sorting
/// Works slowly but gives exact results
/// Sorts its mutable slice argument as a side effect
/// # Example
/// ```
/// use medians::algos::naive_median;
/// let mut v = vec![1_u8,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
/// let res = naive_median(&mut v);
/// assert_eq!(res,8_f64);
/// ```
pub fn naive_median<T>(s:&mut [T]) -> f64
    where T: Copy+PartialOrd,f64:From<T> {
    let n = s.len();
    if n == 0 { panic!("{} empty vector!",here!()); };
    if n == 1 { return f64::from(s[0]); };
    if n == 2 { return (f64::from(s[0])+f64::from(s[1]))/2.0; }; 
    s.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap()); // fastest Rust sort
    let mid = s.len()/2; // midpoint (floors odd sizes)
    if (n & 1) == 0 { (f64::from(s[mid-1]) + f64::from(s[mid])) / 2.0 } // s is even
    else { f64::from(s[mid]) } // s is odd     
}

/// Iterative move towards the median. Used by w_medians
/// Returns ( positive imbalance, number of items equal to x,
/// increment of x position towards the median )
fn next(s:&[f64],x:f64) -> (i64,i64,f64) {
    let mut recipsum = 0_f64;
    let (mut left,mut right) = (0_i64,0_i64); 
    for &si in s {
        if si < x { left += 1; recipsum += 1./(x-si); continue; };
        if si > x { right += 1; recipsum += 1./(si-x); 
        }
    }
    let balance = right-left;
    ( balance.abs(),s.len() as i64-left-right,(balance as f64)/recipsum )
}


/// Used by w_medians
fn nearestlt(set:&[f64],x:f64) -> f64 {
    let mut best = f64::MIN;
    for &s in set {
        if s > x { continue }; 
        if s > best { best = s };
    }
    best
}

/// Used by w_medians
fn nearestgt(set:&[f64],x:f64) -> f64 {
    let mut best = f64::MAX;
    for &s in set {
        if s < x { continue }; 
        if s < best { best = s };
    }
    best
}

/// Iterative median based on the heavily modified 1D case
/// of the improved nD Weiszfeld algorithm.
pub fn w_median<T>(set:&[T]) -> f64
    where T: Copy,f64:From<T> {
    let n = set.len();
    match n {
        1 => f64::from(set[0]),
        2 => f64::from(set[0])+f64::from(set[1])/2.0,
        _ => {
            let s = set.tof64(); // makes an f64 copy
            // arithmetic mean as a initial iterative median 
            let sumx:f64 = s.iter().sum();
            let mean = sumx/(n as f64); 
            if (n & 1) == 0 { even_w_median(&s,mean) } 
            else { odd_w_median(&s,mean) }}
    }
}

fn odd_w_median(s:&[f64],m:f64) -> f64 {
    let mut gm = m; 
    let mut lastsig = 0_i64;
    loop {
        let (sigs,eqs,dx) = next(s,gm);  
        // println!("{} {} {} {}",sigs,eqs,gm,dx);
        // in the midst of the central equal items, return old gm
        if sigs < eqs { return gm }; 
        gm += dx; // update gm
        if (sigs < lastsig) && (sigs >= 3) { // normal converging iteration
            lastsig = sigs;    
            continue; 
        };
        // not converging much or near the centre already, 
        // find manually the nearest item in the dx direction
        if dx > 0. { gm = nearestgt(s, gm); }
        else if dx < 0. { gm = nearestlt(s, gm); };
        if sigs < 3 { return gm;  }; // at the centre, return it
        lastsig = sigs; // otherwise continue with this new value
    }
}

fn even_w_median(s:&[f64],m:f64) -> f64 {
    let mut gm = m; 
    let mut lastsig = 0_i64;
    loop {
        let (sigs,eqs,dx) = next(s,gm);  
        // println!("{} {} {} {}",sigs,eqs,gm,dx);
        // in the midst of the central equal items, return old gm
        if sigs < eqs { return gm }; 
        gm += dx; // update gm
        if (sigs < lastsig) && (sigs >= 2) { // normal converging iteration
            lastsig = sigs;    
            continue; 
        };
        // not converging much or near the centre already, 
        // find manually the nearest item in the dx direction
        if sigs < 2 { return  (nearestgt(s, gm) + nearestlt(s, gm))/2.;  }; // at the centre, return it
        lastsig = sigs; // otherwise continue with
        if dx > 0. { gm = nearestgt(s, gm); }
        else if dx < 0. { gm = nearestlt(s, gm); };
    }
}

/// Used by r_median
fn part<T>(s:&[T],pivot:f64) -> (Vec<T>,Vec<T>) where T:Copy, f64:From<T> {
    let mut ltset = Vec::new();
    let mut gtset = Vec::new();
    for &f in s { 
        if f64::from(f) < pivot { ltset.push(f); } else { gtset.push(f); };
    };
    (ltset,gtset)
}

/// Recursive Reducing Median
pub fn r_median<T>(set:&[T]) -> f64 
    where T: Copy+PartialOrd,f64:From<T> {
    // let s = tof64(set); // makes an f64 copy
    let n = set.len();
    // starting pivot
    let (min,max) = set.minmaxt();
    let pivot = (f64::from(min)+f64::from(max))/2.;
    // passing min max just to stop recomputing it
    if (n & 1) == 0 { r_med_even(set,n/2,pivot,f64::from(min),f64::from(max)) } 
    else { r_med_odd(set,n/2+1,pivot,f64::from(min),f64::from(max)) }
}

/// Reducing sets median using `minmax()` and secant
/// with proportionally subdivided data range as a pivot.
/// Need is a count of items from start of set to anticipated median position
fn r_med_odd<T>(set:&[T],need:usize,pivot:f64,setmin:f64,setmax:f64) -> f64
    where T:PartialOrd+Copy,f64:From<T> {  
    if need == 1 { return setmin }; 
    let n = set.len();
    if need == n { return setmax }; 

    let (ltset,gtset) = part(set,pivot);
    let ltlen = ltset.len();
    let gtlen = gtset.len();
    // println!("Need: {}, Pivot {:5.3}, partitions: {}, {}",need,pivot,ltlen,gtlen);

    match need {
    1 => f64::from(ltset.mint()),
    x if x < ltlen => {
        let max = f64::from(ltset.maxt());
        if setmin == max { return f64::from(ltset[0]) }; // all equal, done     
        let newpivot = setmin + (need as f64)*(max-setmin)/(ltlen as f64);
        r_med_odd(&ltset, need, newpivot,setmin,max) 
        },
    x if x == ltlen => f64::from(ltset.maxt()),
    x if x == ltlen+1 => f64::from(gtset.mint()),
    x if x == n => f64::from(gtset.maxt()),  
    _ => { // need > ltlen
        let newneed = need - ltlen;
        let min = f64::from(gtset.mint()); 
        if min == setmax { return f64::from(gtset[0]) }; // all equal, done
        let newpivot = min + (setmax-min)*(newneed as f64)/(gtlen as f64);
        r_med_odd(&gtset, newneed, newpivot,min,setmax)
        }
    }
}

/// Reducing sets median using `minmax()` and secant
/// with proportionally subdivided data range as a pivot.
/// Need is a count of items from start of set to anticipated median position
fn r_med_even<T>(set:&[T],need:usize,pivot:f64,setmin:f64,setmax:f64) -> f64
    where T:PartialOrd+Copy,f64:From<T> {
    let n = set.len();
    let (ltset,gtset) = part(set,pivot);
    let ltlen = ltset.len();
    let gtlen = gtset.len();
    // println!("Need: {}, Pivot {}, partitions: {}, {}",need,pivot,ltlen,gtlen);
    match need {
    // 1 => ltset.mint(),
    x if x < ltlen => {
        let max = f64::from(ltset.maxt());
        if setmin == max { return f64::from(ltset[0]) }; // all equal, done     
        let newpivot = setmin + (need as f64)*(max-setmin)/(ltlen as f64);
        r_med_even(&ltset, need, newpivot,setmin,max) 
        },
    x if x == ltlen => (f64::from(ltset.maxt())+f64::from(gtset.mint()))/2., // at the boundary 
    x if x == n => f64::from(gtset.maxt()),  
    _ => { // need > ltlen
        let newneed = need - ltlen;
        let min = f64::from(gtset.mint()); 
        if min == setmax { return f64::from(gtset[0]) }; // all equal, done
        let newpivot = min + (newneed as f64)*(setmax-min)/(gtlen as f64);
        r_med_even(&gtset, newneed, newpivot,min,setmax)
        }
    }
}
