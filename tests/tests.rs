#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]
use indxvec::{here, printing::*, Indices, Mutops, Printing, Vecops};
use medians::{*,algos::*};
use ran::*;
use core::cmp::{Ordering, Ordering::*};
use std::convert::From;
use std::error::Error;
use times::{benchf64, benchu64, benchu8, mutbenchf64};

#[test]
fn parting() -> Result<(), Me> {
    let data = [
        5.,
        8.,
        7.,
        6.,
        5.,
        4.,
        3.,
        2.,
        -f64::NAN,
        1.,
        0.,
        1.,
        -2.,
        3.,
        4.,
        -5.,
        f64::NAN,
        f64::NAN,
        6.,
        7.,
        7.,
    ];
    // println!("To u64s: {}",to_u64s(&data).gr());
    // println!("To f64s: {}",to_f64s(&to_u64s(&data)).gr());
    // println!("Scrubbed: {}", scrub_nans(&to_f64s(&to_u64s(&data))).gr());
    let len = data.len();
    println!("Pivot {}: {}", data[0].yl(), data.gr());
    let mut refdata = data.ref_vec(0..len);
    let (eqsub, gtsub) = <&mut [f64]>::part(&mut refdata, &(0..len), &mut <f64>::total_cmp);
    println!(
        "Result: {}\nCommas show the subranges:\n\
        {GR}[{}, {}, {}]{UN}\n{} items equal to pivot {}",
        (eqsub, gtsub).yl(),
        refdata[0..eqsub].to_plainstr(),
        refdata[eqsub..gtsub].to_plainstr(),
        refdata[gtsub..len].to_plainstr(),
        (gtsub - eqsub).yl(),
        data[0].yl()
    );
    let refindex = data.isort_refs(0..len, |a, b| a.total_cmp(b));
    println!("isort_refs ascending sorted:\n{}", &refindex.gr());
    let indx = data.isort_indexed(0..len, |a, b| b.total_cmp(a));
    println!("isort_index (descending):\n{}", indx.gr());
    println!("Unindexed:\n{}", indx.unindex(&data, true).gr());
    Ok(())
}

#[test]
fn text() {
    let song = "There was a *jolly* miller once who lived on the river Dee. \
        From morn till night all day he sang for a jolly old fellow was he; \
        and this forever the burden of his song seemed to be: \
        I care for nobody, no not I, and nobody cares for me. \
        Tee hee heee, quoth he.";
    let v = song.split(' ').collect::<Vec<_>>();
    println!("{}", v.gr()); // Display
    println!(
        "Hash sorted by word lengths: {}",
        v.sorth(|s| s.len() as f64, true).gr()
    );
    println!(
        "Median word(s) by length: {GR}{}{UN}",
        (&v[..])
            .median_by(&mut |a, b| a.len().cmp(&b.len()))
            .expect("text(): median_by length failed\n")
    );
    println!("Sorted by lexicon: {}", v.sortm(true).gr());
    println!(
        "Median word(s) by lexicon: {GR}{}{UN}",
        (&v[..])
            .median_by(&mut <&str>::cmp)
            .expect("text(): median_by lexicon failed\n")
    );
}

#[test]
fn medf64() -> Result<(), Me> {
    let v = [
        9., 10., 18., 17., 16., 15., 14., 1., 2., 3., 4., 5., 6., 7., 8., 17., 10., 11., 12., 13.,
        14., 15., 16., 18., 9.,
    ];
    let weights = [
        1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16., 17., 18., 19., 20.,
        21., 22., 23., 24., 25.,
    ];
    println!("Data:    {}", v.gr());
    println!("Weights: {}", weights.gr());
    let len = v.len();
    let mut vr = v.ref_vec(0..len);
    println!(
        "max: {}",
        min(&vr, 0..len, &mut |a: &f64, b: &f64| b.total_cmp(a)).gr()
    );
    println!(
        "max2: {}",
        min2(&vr, 0..len, &mut |a: &f64, b: &f64| b.total_cmp(a)).gr()
    );
    let (eqsub, gtsub) = <&mut [f64]>::part(&mut vr, &(0..v.len()), &mut <f64>::total_cmp);
    println!(
        "Result: {}\nCommas separate the subranges:\n\
        {GR}[{}, {}, {}]{UN}\n{} items equal to the pivot {}",
        (eqsub, gtsub).yl(),
        vr[0..eqsub].to_plainstr(),
        vr[eqsub..gtsub].to_plainstr(),
        vr[gtsub..len].to_plainstr(),
        (gtsub - eqsub).yl(),
        v[0].yl()
    );
    let median = v.medf_checked()?;
    let mad = v.madf(median);
    println!("Median±mad: {GR}{}±{}{UN}", median, mad);
    println!(
        "Weighted median: {GR}{}{UN} ",
        v.medf_weighted(&weights, 0.0001)?
    );
    Ok(())
}

#[test]
fn correlation() -> Result<(), Me> {
    let v1 = ranv_f64(100).expect("Random vec1 generation failed"); // random vector
    let v2 = ranv_f64(100).expect("Random vec2 generation failed"); // random vector
    println!("medf_correlation: {}", v1.medf_correlation(&v2)?.gr());
    Ok(())
}

#[test]
fn errors() -> Result<(), Me> {
    let n = 10_usize; // number of vectors to test for each magnitude
                      // set_seeds(33333);
    for d in [10, 50, 100, 1000, 10000, 100000] {
        let mut error = 0_i64;
        trait Eq: PartialEq<Self> {}
        impl Eq for f64 {}
        for _ in 0..n {
            let Ok(v) = ranv_u8(d) else {
                return merror("other", "Random vec genertion failed");
            };
            let med = medianu8(&v)?; // random vector
                                     // v.as_slice().medf_unchecked();
            error += qbalance(&v, &med, |&f| f as f64);
        }
        println!("Even length {GR}{d}{UN}, repeats: {GR}{n}{UN}, errors: {GR}{error}{UN}");
        error = 0_i64;
        for _ in 0..n {
            let Ok(v) = ranv_u8(d + 1) else {
                return merror("other", "Random vec genertion failed");
            }; // random vector
            let med = medianu8(&v)?;
            // v
            //    .as_slice()
            //    .medf_unchecked();
            error += qbalance(&v, &med, |&f| f as f64);
        }
        println!(
            "Odd  length {GR}{}{UN}, repeats: {GR}{n}{UN}, errors: {GR}{error}{UN}",
            d + 1
        );
    }
    Ok(())
}

const NAMES: [&str; 2] = ["median_by","medf_unchecked"];

const CLOSURESF64: [fn(&[f64]); 2] = [
    |v: &[_]| {
        v.median_by(&mut <f64>::total_cmp)
            .expect("even median closure failed");
    },
    |v: &[_]| {
        v.medf_unchecked();
    },
    /*
    |v: &[_]| {
        let mut sorted: Vec<&f64> = v.iter().collect();
        sorted.sort_unstable_by(|&a, &b| a.total_cmp(b));
        // sorted[sorted.len()/2];
    },

    |v: &[_]| {
        v.qmedian_by(&mut <f64>::total_cmp,|&x| x)
        .expect("even median closure failed");
    },
    */

    /*
    |v: &[_]| {
        medianu8(v)
            .expect("medianu8 closure failed");
    }
    */
];

#[test]
fn comparison() {
    // set_seeds(0); // intialise random numbers generator
    // Rnum encapsulates the type of random data to be generated
    benchf64(93..110, 1, 10, &NAMES, &CLOSURESF64);
}
