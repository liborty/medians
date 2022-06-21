// use std::ops::Sub;
// use std::cmp::Ordering;
// use anyhow::{Result,bail};
use indxvec::{here,tof64,Mutsort,Vecops};

/// Median of a &[T] slice by sorting
/// Works slowly but gives exact results
/// Sorts its mutable slice argument as a side effect
/// # Example
/// ```
/// use medians::naive_median;
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

/// Exactly the same as naive_median, except uses hashsort, 
/// which is about 25% faster for >1K items.
pub fn hash_median<T>(s:&mut [T],min:f64,max:f64) -> f64
    where T: Copy+PartialOrd,f64:From<T> {
    let n = s.len();
    if n == 0 { panic!("{} empty vector!",here!()); };
    if n == 1 { return f64::from(s[0]); };
    if n == 2 { return (f64::from(s[0])+f64::from(s[1]))/2.0; }; 
    s.muthashsort(min,max); 
    let mid = s.len()/2; // midpoint (floors odd sizes)
    if (n & 1) == 0 { (f64::from(s[mid-1]) + f64::from(s[mid])) / 2.0 } // s is even
    else { f64::from(s[mid]) } // s is odd         
}

fn part(s:&[f64],pivot:f64) -> (Vec<f64>,Vec<f64>) {
    let mut ltset = Vec::new();
    let mut gtset = Vec::new();
    for &f in s { 
        if f < pivot { ltset.push(f); } else { gtset.push(f); };
    };
    (ltset,gtset)
}

pub fn r_median<T>(set:&[T],min:f64,max:f64) -> f64 
    where T: Copy+PartialOrd,f64:From<T> {
    let s = tof64(set); // makes an f64 copy
    let n = set.len();
    if n == 0 { panic!("{} empty vector!",here!()) };
    if (n & 1) == 0 { 
        r_med_even(&s,n/2,min,max) } 
        else { r_med_odd(&s,n/2,(min+max)/2.0,min,max) }
}

/// Simple reducing sets median
/// Need is a count of items from start of set to median position, if set was sorted
/// using proportionally subdivided data range as a pivot.
fn r_med_odd(set:&[f64],need:usize,pivot:f64,min:f64,max:f64) -> f64 { 

    if need == 0 { return set.mint() }; 
    let n = set.len();
    if need == n { return set.maxt() };
    if n == 0  { panic!("{} empty vector!", here!()); }; 

    let (ltset,gtset) = part(set,pivot);
    let ltlen = ltset.len();
    let gtlen = gtset.len();
    println!("Need: {}, Pivot {:5.2}, minmax: {:5.2},{:5.2} partitions: {}, {}",need,pivot,min,max,ltlen,gtlen);

    if ltlen < need {
        let newneed = need - ltlen;
        if newneed == 1 { return gtset.mint() };
        let newpivot = if ltlen == 0 { (pivot+max)/2.0 } else { pivot + (max-pivot)*(newneed as f64)/(gtlen as f64)};
        return r_med_odd(&gtset, newneed, newpivot,pivot,max);
    };
    if ltlen == need { return ltset.maxt() }; 
    let newpivot = if gtlen == 0 { (min+pivot)/2.0 } else { min + (pivot-min)*(need as f64)/(ltlen as f64) };
    r_med_odd(&ltset, need, newpivot, min, pivot ) 
}

/// Simple reducing sets median
/// Need is a count of items from start of set to median position, if set was sorted
/// using proportionally subdivided data range as a pivot.
fn r_med_even(set:&[f64],need:usize,min:f64,max:f64) -> f64 { 
    match set.len() {
        0 => panic!("{} empty vector!", here!()),
        1 => return set[0],
        2 => return (set[0]+set[1])/2.0,
        _ => {} };
    
    let pivot = set[need-1];
    let (ltset,eqset,gtset) = set.partition(pivot);
    let ltlen = ltset.len();
    if ltlen > need { return r_med_even(&ltset, need, min, max ) };    
    let eqlen = eqset.len(); 
    if ltlen+eqlen > need { return r_med_even(&eqset, need-ltlen, min, max ) };
    r_med_even(&gtset, need-ltlen-eqlen, min, max)
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
/// of the modified nD Weiszfeld algorithm.
pub fn w_median<T>(set:&[T]) -> f64
    where T: Copy,f64:From<T> {
    let n = set.len();
    match n {
        0 => panic!("{} empty vector!",here!()),
        1 => return f64::from(set[0]),
        2 => return f64::from(set[0])+f64::from(set[1])/2.0,
        _ => {}
    };
    let s = tof64(set); // makes an f64 copy
    // arithmetic mean as a starting iterative median 
    let sumx:f64 = s.iter().sum();
    let mean = sumx/(n as f64); 
    if (n & 1) == 0 { even_w_median(&s,mean) } 
    else { odd_w_median(&s,mean) }
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
/*

/// Iterative median based on the heavily modified 1D case
/// of the modified nD Weiszfeld algorithm.
/// Reducing the target set.
pub fn wr_median<T>(set:&[T]) -> f64
    where T: Copy,f64:From<T> {
    let n = set.len();
    match n {
        0 => panic!("{} empty vector!",here!()),
        1 => return f64::from(set[0]),
        2 => return f64::from(set[0])+f64::from(set[1])/2.0,
        _ => {}
    };
    let s = tof64(set); // makes an f64 copy
    // arithmetic mean as a starting iterative median 
    let sumx:f64 = s.iter().sum();
    let mean = sumx/(n as f64); 
    if (n & 1) == 0 { even_wr_median(&s,0,n,mean) } 
    else { odd_wr_median(&s,0,n,mean) }
}

fn odd_wr_median(s:&[f64],i:usize,n:usize,m:f64) -> f64 {
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

fn even_wr_median(s:&[f64],i:usize,n:usize,m:f64) -> f64 {
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

/// swap two slice items if they are out of ascending order
fn compswap<T>(s: &mut [T], i1: usize, i2: usize) 
where T: PartialOrd { if s[i1] > s[i2] { s.swap(i1,i2) } }

    
/// N recursive hash sort.
/// Sorts mutable first argument (slice) in place
/// Requires [min,max], the data range, that must enclose all its values. 
/// The range is often known in advance. If not, it can be obtained with `minmaxt`.
pub fn h_median<T>(s: &mut [T], min:f64, max:f64) -> f64
where T: PartialOrd + Copy, f64:From<T> {
    if min >= max { panic!("{} data range must be min < max",here!()); };
    let n = s.len();
    match n {
        0 => panic!("{} empty input",here!()),
        1 => f64::from(s[0]),
        2 => (f64::from(s[0])+f64::from(s[1]))/2.0,
        3 => {
            compswap(s,0,1);
            compswap(s,1,2);
            compswap(s,0,1);
            return f64::from(s[1])
        },
        _ => if (n & 1) == 0 { h_medr_even(s,0,n,min,max) } 
            else { h_medr_odd(s,0,n,min,max) }
    }   
}

fn h_medr_odd<T>(s:&mut [T], i:usize, n:usize, min:f64, max:f64) -> f64
where T: PartialOrd+Copy, f64:From<T>
{ 
    if n == 0 { panic!("{} unexpected zero length",here!())};  
    // hash is a constant s.t. (x-min)*hash is in [0,n) 
    // subtracting a small constant stops subscripts quite reaching n 
    let hash = (n as f64 - 1e-10 ) / (max-min);  
    let mut freqvec:Vec<Vec<T>> = vec![Vec::new();n];
    // group current index items into buckets by their associated s[] values
    for &xi in s.iter().skip(i).take(n) { 
        freqvec[(hash*(f64::from(xi)-min)).floor() as usize].push(xi);
    };
    // count the items in buckets  
    let mut isub = i;  
    for v in freqvec.iter() { 
        let vlen = v.len();
        if vlen == 0 { continue; };
        isub += vlen;
        if isub <= n/2 { continue; };  
        match vlen { 
        1 => return f64::from(v[0]),
        2 => { 
            if isub == n/2 { return f64::from(v[1]) }
            else 
        },
        3 => {
            s[isub] = v[0]; s[isub+1] = v[1]; s[isub+2] = v[2];   
            compswap(s,isub,isub+1);
            compswap(s,isub+1,isub+2);
            compswap(s,isub,isub+1);
            isub += 3;
        },
        x if x == n => { 
            // this bucket alone is populated, 
            // items in it are most likely all equal
            // we need not copy v back as no sorting took place
            let mx = minmax_slice(s,  isub, vlen);
            if mx.minindex < mx.maxindex {  // not all the same
                let mut hold = s[i]; // swap minindex to the front
                s[i] = s[mx.minindex]; 
                s[mx.minindex] = hold;
                hold = s[i+n-1]; // swap maxindex to the end
                s[i+n-1] = s[mx.maxindex]; 
                s[mx.maxindex] = hold;
                // recurse to sort the rest, within the new reduced range
                hashsortr(s,i+1,n-2,f64::from(mx.min),f64::from(mx.max)); 
            };
            return; // all items were equal, or are now sorted
        },
        _ => { 
            // first fill the index with the grouped items from v
            let isubprev = isub;
            for &item in v { s[isub] = item; isub += 1; }; 
            let mx = minmax_slice(s,  isubprev, vlen);
            if mx.minindex < mx.maxindex { // else are all equal 
                let mut hold = s[isubprev]; // swap minindex to the front
                s[isubprev] = s[mx.minindex]; 
                s[mx.minindex] = hold;
                hold = s[isub-1]; // swap maxindex to the end
                s[isub-1] = s[mx.maxindex]; 
                s[mx.maxindex] = hold;
                // recurse to sort the rest
                hashsortr(s,isubprev+1,vlen-2,f64::from(mx.min),f64::from(mx.max)); 
                }; // the items in this bucket were equal or are now sorted but there are more buckets
            } 
        } // end of match 
    } // end of for v
}
*/