#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]
use devtimer::{DevTime, SimpleTimer};
use indxvec::{here, printing::*, Indices, Mutops, Printing, Vecops};
use medians::algos::{balance, med_odd, midof3};
use medians::algosf64::partf64;
use medians::{Me, Median, Medianf64};
use ran::{generators::*, *};
// use std::io::{stdout,Write};
use std::convert::From;
use times::{benchf64, benchu64, benchu8, mutbenchf64};

const NAMES: [&str; 4] = [
    "medianf64",
    "quantified median",
    "generic median",
    "maive median",
];

const CLOSURESF64: [fn(&[f64]); 4] = [
    |v: &[_]| {
        v.median().expect("medianf64 closure failed");
    },
    |v: &[_]| {
        (&v).median(&mut |&x| x).expect("median closure failed");
    },
    |v: &[_]| {
        if v.len() & 1 == 0 {
            v.generic_even().expect("even median closure failed");
        } else {
            v.generic_odd().expect("odd median closure failed");
        };
    },
    |v: &[_]| {
        let mut vm = v.to_owned();
        vm.sort_by(|a, b| a.partial_cmp(b).expect("naive median failed"));
    },
]; // use x.into() when not f64

#[test]
fn parting() {
    let mut data = [
        8., 7., 6., 5., 4., 3., 2., 1., 0., 1., 2., 3., 4., 5., 6., 7., 8.,
    ];
    let mid = midof3(&0, &16, &5);
    println!("Mid of three: {mid}");
    // let mut v: Vec<&f64> = data.iter().collect();
    let pivot = 7_f64;
    println!("Pivot {}, Input:\n{}", pivot.yl(), data.gr());
    let len = data.len();
    let (gtstart, mid, ltend) = partf64(&mut data, &(0..len), pivot);
    println!(
        "[gtstart,mid,ltend]: {}\nCommas show the subscripts' positions:\n\
        {GR}[{}, {}, {}, {}]{UN}\n{} items equal to pivot",
        (gtstart, mid, ltend).gr(),
        data[0..gtstart].to_plainstr(),
        data[gtstart..mid].to_plainstr(),
        data[mid..ltend].to_plainstr(),
        data[ltend..len].to_plainstr(),
        (gtstart + len - ltend).yl()
    );
}

#[test]
fn text() {
    let song = "There was a *jolly* miller once who lived on the river Dee. \
        From morn till night all day he sang for a jolly old fellow was he; \
        and this forever the burden of his song seemed to be: \
        I care for nobody, no not I, and nobody cares for me. \
        Tee hee, quoth he.";
    let v = song.split(' ').collect::<Vec<_>>();
    println!("{}", v.gr()); // Display
    println!(
        "Hash sorted by word lengths: {}",
        v.sorth(|s| s.len() as f64, true).gr()
    );
    let median_word = (&v[..])
        .median(&mut |&s| s.len() as f64)
        .expect("text(): Median failed\n");
    println!("Median word length in bytes is: {}", median_word.yl());
    println!("Merge sorted by lexicon: {}", v.sortm(true).gr());
    println!(
        "Even median lexographic words are: {}",
        (&v[..]).generic_even().expect("strict_eve failed").yl()
    );
}

#[test]
fn medf64() {
    let v = [
        1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16., 17., 18.,
    ];
    let res = v.medstats().expect("MStats struct missing");
    println!("\nMedstats: {}", res);
}

#[test]
fn comparison() {
    set_seeds(7777777777_u64); // intialise random numbers generator
                               // Rnum encapsulates the type of random data to be generated
    benchf64(Rnum::newf64(), 7..15000, 500, 5, &NAMES, &CLOSURESF64);
}

#[test]
fn errors() -> Result<(), Me> {
    let n = 10_usize; // number of vectors to test for each magnitude
    set_seeds(77777777_u64); // intialise random numbers generator
    let rv = Rnum::newu8();
    for d in [10, 50, 100, 1000, 10000, 100000] {
        let mut error = 0_i64;
        trait Eq: PartialEq<Self> {}
        impl Eq for f64 {}
        for _ in 0..n {
            let v = rv.ranv(d).expect("Random vec genertion failed").getvu8()?; // random vector
            let med = v
                .as_slice()
                .median(&mut |f| *f as f64)
                .expect("even errors test"); // &mut |t:&u8| *t as f64
            error += balance(&v, med, &mut |f| *f as f64);
        }
        println!("Even length {GR}{d}{UN}, repeats: {GR}{n}{UN}, errors: {GR}{error}{UN}");
        error = 0_i64;

        for _ in 0..n {
            let v = ranvu8(d + 1).expect("Random vec genertion failed"); // random vector
            let med = v
                .as_slice()
                .median(&mut |t: &u8| *t as f64)
                .expect("odd errors test");
            error += balance(&v, med, &mut |f| *f as f64);
        }
        println!(
            "Odd  length {GR}{}{UN}, repeats: {GR}{}{UN}, errors: {GR}{}{UN}",
            d + 1,
            n,
            error
        );
    }
    Ok(())
}
