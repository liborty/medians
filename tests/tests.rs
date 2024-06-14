#![allow(unused_imports)]
#![allow(dead_code)]
#[cfg(test)]
use indxvec::{here, printing::*, Indices, Mutops, Printing, Vecops};
use medians::{*,algos::*};
use ran::*;
use core::cmp::{Ordering, Ordering::*};
use std::convert::From;
use std::error::Error;
use times::{benchf64, benchu64, benchu8, mutbenchf64, mutbenchu64};

#[test]
fn partbin() -> Result<(), Me> {
    let mut data = [257_u64,9,8,7,6,5,4,3,2,1];
    println!("Data: {}",data.gr());  
    let n = data.len();
    let gtsub = data.part_binary( &(0..n), 3);
    println!("Partitioned by bit 3: {},{}",data[..gtsub].gr(),data[gtsub..].gr());
    println!("Median: {}",evenmedianu64(&mut data).gr());
    Ok(())
}

#[test]
fn ftest() -> Result<(), Me> {
let a = vec![
    100.0, 
    163.6170150950381,
    224.6127142531872,
    239.91368100304916,
    345.1674002412572,
    402.88833594261706,
    423.6406741377381,
    472.6292699764225,
    487.23306678749594,
    490.94434592125606,
    511.16658896980687,
    516.3472076946555,
    523.052566308903,
    563.6784311991111,
    586.7283185517608,
    633.5580942760708,
    678.4956618813414,
    708.2452516626092,
    741.9710552209048,
    763.476192474483,
    768.6249939324011,
    777.1952444919513,
    785.2192860329102,
    785.3178558989187,
    858.0319001781837,
    927.4228569429413,
    952.453888947949,
    1067.6089037099757,
];
eprintln!("Median: {} ", a.medf_unchecked());
Ok(())
}

#[test]
fn parting() -> Result<(), Me> {
    let data = [
        5.,8.,7.,6.,5.,4.,3.,2.,-f64::NAN,
        1.,0.,1.,-2.,3.,4.,-5.,f64::NAN,f64::NAN,
        6.,7.,7.,
    ];
    println!("Data; {}",data.gr());
    let len = data.len();
    let mut refdata = data.ref_vec(0..data.len()); 
    let (eqsub, gtsub) = <&mut [f64]>::part(&mut refdata, &(0..len), &mut <f64>::total_cmp);
    println!("Pivot {}. {} items found equal to the pivot", data[0].yl(), (gtsub - eqsub).yl()); 
    println!("Partitions:\n{}, {}, {}\n",        
        refdata[0..eqsub].gr(), //to_plainstr(),
        refdata[eqsub..gtsub].gr(),
        refdata[gtsub..len].gr()
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
        From morn till night all day he sang, for a jolly old fellow was he; \
        and this forever the burden of his song seemed to be: \
        I care for nobody, no not I, and nobody cares for me. \
        Tee hee heee, piddle piddledy dee, quoth he.";
    let v = song.split(' ').collect::<Vec<_>>();
    println!("{}", v.gr()); // Display
    // v.mutisort(0..v.len(),|&a,&b| a.len().cmp(&b.len()));
    println!(
        "Insert log sorted by word lengths: {}", 
        v.isort_refs(0..v.len(),|&a,&b| a.len().cmp(&b.len())).gr() 
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
        extremum_refs(&vr, 0..len, &mut |a: &f64, b: &f64| b.total_cmp(a)).gr()
    );
    println!(
        "max2: {}",
        best_two_refs(&vr, 0..len, &mut |a: &f64, b: &f64| b.total_cmp(a)).gr()
    );
    let (eqsub, gtsub) = <&mut [f64]>::part(&mut vr, &(0..v.len()), &mut <f64>::total_cmp);
    println!("Partitioning (pivot {}, commas separate the subranges): {}", v[0].yl(),(eqsub,gtsub).gr()); 
    println!("{GR}[{}, {}, {}]{UN}\nNumber of items equal to the pivot {}",
        vr[0..eqsub].to_plainstr(),
        vr[eqsub..gtsub].to_plainstr(),
        vr[gtsub..len].to_plainstr(),
        (gtsub - eqsub).yl()     
    );
    let median = v.medf_checked()?;
    let mad = v.madf(median);
    println!("Median±mad: {GR}{}±{}{UN}", median, mad);
    println!("Mean:       {GR}{}{UN}", v.iter().sum::<f64>()/(len as f64));
    println!(
        "Weighted median: {GR}{}{UN} ",
        v.medf_weighted(&weights, 0.00001)?
    );
    let prodsum:f64 = v.iter().zip(weights.iter()).map(|(x,w)| x*w ).sum();
    println!("Weighted mean:   {GR}{}{UN}", prodsum/weights.iter().sum::<f64>());
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
            let Ok(mut v) = ranv_u64(d) else {
                return merror("other", "Random vec genertion failed");
            };
            let (m1,m2) = medu64(&mut v)?;
            error += qbalance(&v, &((m1 as f64+m2 as f64)/2.0), |&f| f as f64);
        }
        println!("Even length {GR}{d}{UN}, repeats: {GR}{n}{UN}, errors: {GR}{error}{UN}");
        error = 0_i64;
        for _ in 0..n {
            let Ok(mut v) = ranv_u64(d + 1) else {
                return merror("other", "Random vec genertion failed");
            };
            // v
            //    .as_slice()
            //    .medf_unchecked();
            let (m1,m2) = medu64(&mut v)?;
            error += qbalance(&v, &((m1 as f64+m2 as f64)/2.0), |&f| f as f64);
        }
        println!(
            "Odd  length {GR}{}{UN}, repeats: {GR}{n}{UN}, errors: {GR}{error}{UN}",
            d + 1
        );
    }
    Ok(())
}

#[test]
fn comparison() {
println!("Comparison tests running, please wait....");
const NAMES: [&str; 5] = ["median_by","medf_unchecked","uqmedian","medianu64","medu64"];

const CLOSURESU64: [fn(&mut [u64]); 5] = [
    |v: &mut [_]| {
        v.median_by(&mut <u64>::cmp)
            .expect("median_by closure failed");
    },

    |v: &mut [_]| {
        let vf:Vec<f64> = v.iter().map(|&x| x as f64).collect();
        vf.medf_unchecked();
    //    .expect("medf_checked found NaN");
    },

    |v: &mut [_]| { // already in u64, so using identity quantifier
        v.uqmedian(|&x| x)
        .expect("uqmedian error");
    },

    |v: &mut [_]| {
        medianu64(v)
        .expect("uqmedian error");
    },

    |v: &mut [_]| {
        medu64(v)
        .expect("uqmedian error");
    }

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
    mutbenchu64(100000..100010, 1, 10, &NAMES, &CLOSURESU64);
}
