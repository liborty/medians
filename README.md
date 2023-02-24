# Medians [<img alt="crates.io" src="https://img.shields.io/crates/v/medians?logo=rust">](https://crates.io/crates/medians) [<img alt="crates.io" src="https://img.shields.io/crates/d/medians?logo=rust">](https://crates.io/crates/medians) [<img alt="docs.rs" src="https://img.shields.io/docsrs/medians?logo=rust">](https://docs.rs/medians) [<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/liborty/medians/HEAD?logo=github">](https://github.com/liborty/medians) [![Actions Status](https://github.com/liborty/medians/workflows/test/badge.svg)](https://github.com/liborty/medians/actions)

Fast algorithm for finding 1d medians, implemented in Rust.

## Usage

```rust
use medians::{MStats,Medianf64,Median};
```

## Introduction

Finding medians is a common task in statistics and general data analysis. At least it should be common. Medians are more stable measure of central tendency than means. They are not used nearly enough, one suspects simply due to being slower and more difficult to compute. The fast algorithms developed here are non-trivial.

We argued in [`rstats`](https://github.com/liborty/rstats), that using the Geometric Median is the most stable way to characterise multidimensional data (nd). That leaves the one dimensional (1d) medians, addressed here.

See [`tests.rs`](https://github.com/liborty/medians/blob/main/tests/tests.rs) for examples of usage. Their automatically generated output can also be found by clicking the 'test' icon at the top of this document and then examining the latest log.

## Naive Median

Median can be found by sorting the list of data and then picking the midpoint. Even when using a good quality sort, the complexity is at best `O(nlog(n))`. Faster median algorithms, with complexity `O(n)`, are based on the observation that not all items need to be fully sorted, only partitioned and counted off.

Therefore the naive median can not compete. It has been deleted as of version 2.0.0.

## Fast Algorithms

**`medianf64`** (in trait Medianf64)  
Iteratively partitions data around a pivot. The arithmetic mean is used as the pivot estimate. Summation is faster to compute than comparisons and memory manipulations of currently popular 'median of medians' methods.

This algorithm has linear complexity and performs very well. However, it does rely on the data being of end type f64.

We also supply `trait Median` for applications when data can be converted to f64. This is accomplished in a general way by a user defined explicit 'quantify' closure. Whenever the quantification is possible, it is the recommended way, as these algorithms are generally faster and always supply f64 type results.

**`odd_strict_median`** (in trait Median)  
Returns the midpoint of type T, which could be any complex unquantifiable struct type. Only traits Ord and Clone have to be implemented for T.

This algorithm uses `BinaryHeap<T>` to find the unsorted minimum n/2+1 items and then picks their maximum (which is at the root of the max heap already). Thus all comparisons and swaps are kept to the minimum. Furthermore, only pointers to `<T>` items are being manipulated, minimising also the moving of the potentially bulky original end data items.

**`even_strict_median`**  
When the data items are unquantifiable, we can not simply average the two midpoints of even length data, as we did before. So we return them both as a pair tuple, the lesser one first. Otherwise very similar to `odd_strict_median`. However, note that these two methods return results of different types, so the user has to deal with them explicitly, as appropriate.

## `Struct MStats`

Holds the sample parameters: centre (here the median), and the spread measure, (here MAD = median of absolute differences from the median). MAD is the most stable measure of data spread. Alternatively, MStats can hold the mean and the standard deviation, as computed in crate RStats.

## Trait Medianf64

This is the fastest and simplest implementation for data of type &[f64].

```rust
///Fast 1D f64 medians and associated information and tasks
pub trait Medianf64 {
    /// Finds the median of `&[f64]`, fast
    fn medianf64(self) -> Result<f64, ME>;
    /// Zero median data produced by subtracting the median.
    fn zeromedianf64(self) -> Result<Vec<f64>, ME>;
    /// Median correlation = cosine of an angle between two zero median vecs
    fn mediancorrf64(self, v: &[f64]) -> Result<f64, MedError<String>>;
    /// Median of absolute differences (MAD).
    fn madf64(self, med: f64) -> Result<f64, ME>;
    /// Median and MAD.
    fn medstatsf64(self) -> Result<MStats, ME>;
}
```

## Trait Median
Is the generic version of Medianf64. All the methods take an extra argument, a quantification closure, which evaluates T to f64.

```rust
/// Fast 1D generic medians and associated information and tasks
pub trait Median<T> {
    /// Finds the median of `&[T]`, fast
    fn median(self, quantify: &mut impl FnMut(&T) -> f64) -> Result<f64, ME>;
    /// Finds the median of odd sized nonquantifiable Ord data
    fn odd_strict_median(&self) -> &T
    where
        T: Ord + Clone;
    /// Finds the two mid values of even sized nonquantifiable Ord data
    fn even_strict_median(&self) -> (&T, &T)
    where
        T: Ord + Clone;
    /// Zero median f64 data produced by finding and subtracting the median.
    fn zeromedian(self, quantify: &mut impl FnMut(&T) -> f64) -> Result<Vec<f64>, ME>;
    /// Median correlation = cosine of an angle between two zero median vecs
    fn mediancorr(self, v: &[T], quantify: &mut impl FnMut(&T) -> f64) 
        -> Result<f64, MedError<String>>;
    /// Median of absolute differences (MAD).
    fn mad(self, med: f64, quantify: &mut impl FnMut(&T) -> f64) -> Result<f64, ME>;
    /// Median and MAD.
    fn medstats(self, quantify: &mut impl FnMut(&T) -> f64) -> Result<MStats, ME>;
}
```
Lib.rs gives an example Ordf64 struct, which is a wrapped f64 that implements Ord. This would enable the use of strict medians on f64 data. Remember that the strict medians require their T to be Ord.
It is here for instruction only, for implementing Ord for user types T.
Normally, on f64s, it is of course more efficient to use Median64 trait.
Only non numeric unquantizable types need the slowest, strict medians algorithms.

## Release Notes

**Version 2.1.3** - Simplified odd/even strict medians. Updated indxvec dependency. Fixed dependency and tests.

**Version 2.1.1** - Simplified/improved the display of struct MStats.

**Version 2.1.0** - Added omitted method `zeromedianf64`. Upped the version.

**Version 2.0.9** - Code simplifications. Removed quartiles and struct Med holding them. Mad, supplied via MStats,can do everything that quartiles did and better.

**Version 2.0.8** - Added another test. Fixed a typo bug in `Median` and `Medianf64`.

**Version 2.0.7** - Gained some more speed by a new invention: 'secant mean pivoting'. Made `Medianf64` methods to be non-destructive, at the cost of cloning the data.

**Version 2.0.6** - Added trait Medianf64 for simplicity and speed over f64 data.

**Version 2.0.3** - Added methods `odd_strict_median` and `even_strict_median` to trait `Median<T>`.
These methods apply in classical situations where T is unquantifiable, only Ord(ered). They are about 1.75 times slower.
However, this is only a constant factor which does not grow with the length of data.

**Version 2.0.2** - Removed trait parameter Q to ease external usage.

**Version 2.0.1** - Moved all methods directly associated with 1d medians from `rstats` to here. Removed all remaining trait bounds from end data type T. This is one of the benefits of passing explicit `quantify` closures.

**Version 2.0.0** - Better, leaner, faster! Drastically reduced stack usage. Significant speed up using iterative implementation. More concise code. Deleted all old algorithms with inferior performance, such as `naive_median`. Pivot value estimates are now simple arithmetic means. This is not as sophisticated as secant but is fast to evaluate, giving better overall performance. Introduced closure argument `quantify`, allowing dynamic application to any (quantifiable) data types. Yanked versions 1.0.9 & 1.0.10 as returning `Result` was a breaking change which according to `semver` requires major new version, i.e. this one.