# Medians [<img alt="crates.io" src="https://img.shields.io/crates/v/medians?logo=rust">](https://crates.io/crates/medians) [<img alt="crates.io" src="https://img.shields.io/crates/d/medians?logo=rust">](https://crates.io/crates/medians) [<img alt="docs.rs" src="https://img.shields.io/docsrs/medians?logo=rust">](https://docs.rs/medians) [<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/liborty/medians/HEAD?logo=github">](https://github.com/liborty/medians) [![Actions Status](https://github.com/liborty/medians/workflows/test/badge.svg)](https://github.com/liborty/medians/actions)

Fast algorithm for finding 1d medians, implemented in Rust.

## Usage

```rust
use medians::{Med,MStats,Median};
```

## Introduction

Finding the medians is a common task in statistics and general data analysis. At least it should be, if only it did not take so long. We argue in [`rstats`](https://github.com/liborty/rstats), that using the Geometric Median is the most stable way to characterise multidimensional data (nd). That leaves the one dimensional (1d) medians, addressed here. Medians are more stable measure of central tendency than means but they are not used nearly enough. One suspects that this is mostly due to being slower to compute and the fast algorithm developed here being non-trivial. 

See [`tests.rs`](https://github.com/liborty/medians/blob/main/tests/tests.rs) as examples of usage. Their automatically generated output can be found by clicking the 'test' icon at the top of this document and then examining the latest log.

## Naive Median

Median can be found by sorting the list of data and then picking the midpoint. Even when using a good quality sort, the complexity is at best `O(nlog(n))`. Faster median algorithms, with complexity `O(n)`, are based on the observation that not all items need to be fully sorted, only partitioned and counted off.

Therefore the naive median can not compete. It has been deleted as of version 2.0.0.

## Better Algorithms

* `medianf64 and auto_median`
Iteratively partitions data around a pivot estimate. In the past, we estimated the pivot by a secant method. However, this needs both end points of the current interval, that is the maximum and minimum of the data (sub)set, which are relatively expensive to find, involving many comparisons. The arithmetic mean of the data is faster to compute. Summation being faster than comparisons and memory manipulations of 'median of medians' or the previous method. 

Now we use our novel method of estimating the pivot position. The estimate is the data mean, 
inversely weighted by how many items remain to reach the midpoint. This algorithm has linear complexity and performs very well.
Of course, it does rely on the data being quantifiable, as do all non linear equation solving methods.

* `odd_strict_median`
Returns the midpoint of type T, which could be any complex unquantifiable struct type. Traits Ord and Clone have to be implemented for T.  
The algorithm uses `BinaryHeap<T>` to find the unsorted minimum n/2+1 items and then picks their maximum (which is at the root of the max heap already). Thus all comparisons and swaps are kept to the minimum. Furthermore, only pointers to T items are being manipulated, minimising also the moving of the potentially bulky original data items.

* `even_strict_median`
As the data items T are now unquantifiable, we can not simply average the two midpoints of even length data, as we did before. So we return them both as a tuple, the smaller one first. Otherwise very similar to `odd_strict_median`.

## Structs

* **MStats** - centre (here the median), and the data dispersion measure. Here it is MAD = median of absolute differences from the median. MAD is the most stable measure of data spread.
* **Med** - median, lower and upper quartiles, MAD and standard error.

## Trait Medianf64

We list the provided traits in the order of decreasing speed and increasing generality.

Thus Medianf64 is the fastest and simplest implementation, for data of type &[f64].

```rust
/// Fast 1D f64 medians and associated information and tasks
pub trait Medianf64 {
    /// Finds the median of `&[f64]`, fast
    fn medianf64(self) -> Result<f64, ME>; 
    /// Zero median f64 data produced by finding and subtracting the median.
    fn zeromedianf64(self) -> Result<Vec<f64>, ME>;
    /// Median correlation = cosine of an angle between two zero median vecs
    fn mediancorrf64(self, v: &[f64] ) -> Result<f64, MedError<String>>;
    /// Data spread measure: median of absolute differences (MAD).
    fn madf64(self, med: f64) -> Result<f64, ME>;
    /// Median and MAD.
    fn medstatsf64(self) -> Result<MStats, ME>;
    /// Median, quartiles, MAD, Stderr
    fn medinfof64(self) -> Result<Med, ME>;
}
```

## Trait Median
Is the generic version of Medianf64. All the methods take an extra argument, a quantification closure, which evaluates T to f64.
```rust
/// Fast 1D medians and associated information and tasks
pub trait Median<T> {
    /// Finds the median of `&[T]`, fast
    fn median(self, quantify: &mut impl FnMut(&T) -> f64) -> Result<f64, ME>;
    /// Finds the median of odd sized nonquantifiable Ord data
    fn odd_strict_median(self) -> T
    where
        T: Ord + Clone;
    /// Finds the two mid values of even sized nonquantifiable Ord data
    fn even_strict_median(self) -> (T,T)
    where
        T: Ord + Clone;
    /// Zero median f64 data produced by finding and subtracting the median.
    fn zeromedian(self, quantify: &mut impl FnMut(&T) -> f64) -> Result<Vec<f64>, ME>;
    /// Median correlation = cosine of an angle between two zero median vecs
    fn mediancorr(
        self,
        v: &[T],
        quantify: &'static mut impl FnMut(&T) -> f64,
    ) -> Result<f64, MedError<String>>;
    /// Median of absolute differences (MAD).
    fn mad(self, med: f64, quantify: &mut impl FnMut(&T) -> f64) -> Result<f64, ME>;
    /// Median and MAD.
    fn medstats(self, quantify: &mut impl FnMut(&T) -> f64) -> Result<MStats, ME>;
    /// Median, quartiles, MAD, Stderr
    fn medinfo(self, quantify: &mut impl FnMut(&T) -> f64) -> Result<Med, ME>;
}
```
Lib.rs gives an example Ordf64 struct, which is a wrapped f64 that implements Ord. This would enable the use of strict medians on f64 data. Remember that the strict medians require their T to be Ord.
It is here for instruction only, for implementing Ord for user types T.
Normally, on f64s, it is of course more efficient to use Median64 trait.
Only non numeric unquantizable types need the slowest, strict medians algorithms.

## Release Notes

**Version 2.0.7** - Gained some more speed by a new invention: 'secant mean pivoting'. Made `Medianf64` methods to be non-destructive, at the cost of cloning the data.

**Version 2.0.6** - Added trait Medianf64 for simplicity and speed over f64 data.

**Version 2.0.3** - Added methods `odd_strict_median` and `even_strict_median` to trait `Median<T>`.
These methods apply in classical situations where T is unquantifiable, only Ord(ered). They are about 1.75 times slower.
However, this is only a constant factor which does not grow with the length of data.

**Version 2.0.2** - Removed trait parameter Q to ease external usage.

**Version 2.0.1** - Moved all methods directly associated with 1d medians from `rstats` to here. Removed all remaining trait bounds from end data type T. This is one of the benefits of passing explicit `quantify` closures.

**Version 2.0.0** - Better, leaner, faster! Drastically reduced stack usage. Significant speed up using iterative implementation. More concise code. Deleted all old algorithms with inferior performance, such as `naive_median`. Pivot value estimates are now simple arithmetic means. This is not as sophisticated as secant but is fast to evaluate, giving better overall performance. Introduced closure argument `quantify`, allowing dynamic application to any (quantifiable) data types. Yanked versions 1.0.9 & 1.0.10 as returning `Result` was a breaking change which according to `semver` requires major new version, i.e. this one.

**Version 1.0.9** - Added custom MedError and wrapped outputs in Result. Updated `times` dependency.

**Version 1.0.8** - Added fully automated tests by github actions.

**Version 1.0.7** - Updated to `ran 1.0.4`

**Version 1.0.6** - Updated to `times 1.0.4`. Changed the comparison test accordingly.

**Version 1.0.5** - Simplification. Deleted unnecessary w_median. Simplified error test. Updated dev-dependencies `ran 1.0.3` and `times 1.0.3`.

**Version 1.0.4** - Updated dependency `indxvec v.1.4.2`.

**Version 1.0.3** - Added ratio mad/median (standard error) to `struct Med` and improved its Display.

**Version 1.0.2** - Removed unnecessary extra reference from method `median`.

**Version 1.0.1** - Added for convenience `struct MStats` and method `medstats` returning it. It holds here the median and MAD. More generally, any `centre` and `dispersion`. Moved the low level and private functions to module `algos.rs`. Updated `times` dev-dependency.

**Version 1.0.0** -  Updated to the latest `indxvec` dependency, v. 1.2.11. Added `times` crate for timing comparison test.

**Version 0.1.2** - The public methods are now in trait Median.
