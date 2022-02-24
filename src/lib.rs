// use std::ops::Sub;
use std::cmp::Ordering;
// use anyhow::{Result,bail};
use indxvec::{here,tof64,merge::hashsort};

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
    hashsort(s,min,max); 
    let mid = s.len()/2; // midpoint (floors odd sizes)
    if (n & 1) == 0 { (f64::from(s[mid-1]) + f64::from(s[mid])) / 2.0 } // s is even
    else { f64::from(s[mid]) } // s is odd     
}

fn nearestlt(set:&[f64],x:f64) -> f64 {
    let mut best = f64::MIN;
    for &s in set {
        if s > x { continue }; 
        if s > best { best = s };
    }
    best
}

fn nearestgt(set:&[f64],x:f64) -> f64 {
    let mut best = f64::MAX;
    for &s in set {
        if s < x { continue }; 
        if s < best { best = s };
    }
    best
}

pub fn balance<T>(s:&[T],x:f64) -> i64 where T: Copy,f64:From<T> {
    let mut bal = 0_i64;
    for &si in s { 
        let d = f64::from(si)-x;
        bal += d.signum() as i64;
    }
    bal
}

/// Iterative move towards the median.
/// Returns ( balance, number of items equal to x,
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
    ( balance,s.len() as i64-left-right,(balance as f64)/recipsum )
}


/// Iterative median based on the modified 1D case
/// of the modified nD Weiszfeld algorithm.
/// Can sometimes fail to give the best answer
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
    loop {
        let (sigs,eqs,dx) = next(s,gm);  
        println!("{} {} {} {}",sigs,eqs,gm,dx);
        if sigs.abs() < 3 { 
            if eqs > 0 { return gm };
            return match sigs.cmp(&0_i64) {
                Ordering::Greater => nearestgt(s, gm),
                Ordering::Less => nearestlt(s, gm),
                Ordering::Equal => gm,   
            }
        } 
        gm += dx;
    }
}

fn even_w_median(s:&[f64],m:f64) -> f64 {
    let mut gm = m; 
    loop {
        let (sigs,eqs,dx) = next(s,gm);
        if sigs.abs() < 2 {
            break (nearestlt(s,gm)+nearestgt(s,gm))/2.0 }; 
        gm += dx;  
    }
}

/* 

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