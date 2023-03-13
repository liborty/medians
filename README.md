# Medians [<img alt="crates.io" src="https://img.shields.io/crates/v/medians?logo=rust">](https://crates.io/crates/medians) [<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/liborty/medians/HEAD?logo=github">](https://github.com/liborty/medians) [![Actions Status](https://github.com/liborty/medians/workflows/test/badge.svg)](https://github.com/liborty/medians/actions)

**Author: Libor Spacek**

Fast algorithm for finding 1d medians, implemented in Rust.

## Usage

```rust
use medians::{MStats,Medianf64,Median};
```

## Introduction

Finding medians is a common task in statistics and general data analysis. At least it should be common. Median is a more stable measure of central tendency than a mean. Similarly, MAD (median of absolute differences from median) is more stable measure of data spread than standard deviation. They are not used nearly enough, simply for historical reasons: being slower and more difficult to compute. The fast algorithms developed here present a practical remedy for this situation.

We argued in [`rstats`](https://github.com/liborty/rstats), that using the Geometric Median is the most stable way to characterise multidimensional data. That leaves the one dimensional case, addressed here.

See [`tests.rs`](https://github.com/liborty/medians/blob/main/tests/tests.rs) for examples of usage. Their automatically generated output can also be found by clicking the 'test' icon at the top of this document and then examining the latest log.

## Naive Median

Median can be found by sorting the list of data and then picking the midpoint. When using the best known sort algorithm(s), the complexity is `O(nlog(n))`. Faster median algorithms, with complexity `O(n)`, are based on the observation that not all data items need to be fully sorted, only partitioned and counted off.

Therefore the naive median can not compete. It has been deleted as of version 2.0.0.

## Trait Medianf64

```rust
/// Fast 1D generic medians and associated information and tasks
pub trait Medianf64 {
    /// Finds the median of `&[T]`, fast. 
    fn median(self) -> Result<f64, Me>;  
     /// Zero median data produced by finding and subtracting the median. 
    fn zeromedian(self) -> Result<Vec<f64>, Me>;
    /// Median correlation = cosine of an angle between two zero median vecs
    fn mediancorr(self,v: &[f64]) -> Result<f64, Me>;
    /// Median of absolute differences (MAD).
    fn mad(self, med: f64) -> Result<f64, Me>;
    /// Median and MAD.
    fn medstats(self) -> Result<MStats, Me>;
}
```

### `median`

this method iteratively partitions f64 data around a pivot. The core of this and most of the following algorithms is fast (in place) partitioning function, combined with a simple pivot selection strategy (median of a sample of three). Our partitioning strategy is particularly fast  on data with repeated values.

This is our fastest algorithm. It has linear complexity and performs very well.

## Trait Median

Is the general version of `Medianf64`. Most methods take an extra argument, a quantification closure, which evaluates T to f64.

Five of the methods have the same names and perform the same roles as those in trait `Medianf64` above, except for the extra cost of the quantification conversions, which extend their applicability to all 'quantifiable' generic types T. 
Perhaps keeping their different names may have been simpler but the auto referencing is more automated.

 The `quantify` closures allow not just standard `as` and `into()` conversions but also different competing ways of quantifying more complex types. 

For some types even the quantification is not possible. For those there are methods `generic_odd` and `generic_even`, which return references to the actual central item(s) but otherwise use much the same code. They are particularly suited to large types which we might not wish to move or clone. They incur only the additional cost of the extra layer of referencing.

```rust
/// Fast 1D generic medians and associated information and tasks.  
/// Using auto referencing to disambiguate conflicts 
/// with five more specific Medianf64 methods with the same names.  
/// To invoke specifically these generic versions, add a reference:  
/// `(&v[..]).method` or `v.as_slice().method`
pub trait Median<T> {
    /// Finds the median of `&[T]`, fast. 
    fn median(&self, quantify: &mut impl FnMut(&T) -> f64) 
        -> Result<f64, Me>; 
    /// Odd median for any PartialOrd type T 
    fn generic_odd(&self) -> Result<&T, Me>;
    /// Even median for any PartialOrd type T 
    fn generic_even(&self) -> Result<(&T,&T), Me>;
    /// Finds the item at sort index k. For median, use k = self.len()/2 
    // fn strict_odd(&self, k:usize) -> Result<&T,Me>;
    /// Finds the two items from sort index k. For both even medians, use k = self.len()/2
    // fn strict_even(&self, k:usize) -> Result<(&T, &T),Me>;
    /// Zero median data produced by finding and subtracting the median. 
    fn zeromedian(&self, quantify: &mut impl FnMut(&T) -> f64) 
        -> Result<Vec<f64>, Me>;
    /// Median correlation = cosine of an angle between two zero median vecs
    fn mediancorr(&self,v: &[T],quantify: &mut impl FnMut(&T) -> f64) 
        -> Result<f64, Me>;
    /// Median of absolute differences (MAD).
    fn mad(&self, med: f64, quantify: &mut impl FnMut(&T) -> f64) 
        -> Result<f64, Me>;
    /// Median and MAD.
    fn medstats(&self, quantify: &mut impl FnMut(&T) -> f64) 
        -> Result<MStats, Me>;
}
```

**`even_strict_median`**

When the data items are unquantifiable, we can not simply average the two midpoints of even length data, as we did in `median`. So we return them both as a pair tuple, the smaller one first. Otherwise very similar to `odd_strict_median`. However, note that these two methods return results of different types, so the user has to deal with them explicitly, as appropriate.

## `Struct MStats`

Holds the sample parameters: centre (here the median), and the spread measure, (here MAD = median of absolute differences from the median). MAD is the most stable measure of data spread. Alternatively, MStats can hold the mean and the standard deviation, as computed in crate RStats.

## Release Notes

**Version 2.2.0** - Major new version with much improved speed and generality and some breaking changes (renaming).

**Version 2.1.7** - More pruning and test improvements.

**Version 2.1.6** - Fixed a bug in `fmin2` and `fmax2` that made the median one off the centre sometimes. Apologies. Pruned some code.

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