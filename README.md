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

## Algorithms Analysis

Median can be found naively by sorting the list of data and then picking the midpoint. When using the best known sort algorithm(s), the complexity is `O(nlog(n))`. Faster median algorithms, with complexity `O(n)`, are based on the observation that not all data items need to be fully sorted, only partitioned and counted off. Therefore the naive median can not compete. It has been deleted as of version 2.0.0.

Currently considered to be the 'state of the art' algorithm is Floyd-Rivest (1975) Median of Medians. This divides the data into groups of five items, finds a median of each group and then recursively finds medians of five of these medians, and so on, until only one is left. This is then used as a pivot for the partitioning of the original data. This pivot is guaranteed to produce 'pretty good' partitions.

However, the overall objective is not to find the optimal pivot. Rather, the fastest algorithm will be the one capable of eliminating the maximum number of items per unit of time. Therefore the expense of choosing the pivot enters into the equation. It is possible to allow less optimal pivots, as we do, and yet on average achieve more eliminations.

Let our average ratio of items remaining after one partition be RS and Floyd-Rivest be RF. Where `1/2 <= RF <= RS < 1` (RF is more optimal). But suppose that we can do two partitions in the time it takes Floyd-Rivest to do one (because of their expensive pivot selection process). We then get  better performance when: `RS^2 < RF`, which is entirely possible and seems to be confirmed in practice. For example, RF=0.65 (nearly optimal), RS=0.8 (deeply suboptimal), yet `RS^2 < RF`.

## Trait Medianf64

```rust
/// Fast 1D f64 medians and associated tasks
pub trait Medianf64 {
    /// Finds the median, fast. 
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
/// `(&v[..]).method` or `v.as_slice().method`.  
/// Apart from `generic_odd` and `generic_even`, a `quantify` closure
/// also has to be added as an argument.
pub trait Median<T> {
    /// Finds the median of `&[T]`, fast. 
    fn median(&self, quantify: &mut impl FnMut(&T) -> f64) 
        -> Result<f64, Me>; 
    /// Odd median for any PartialOrd type T 
    fn generic_odd(&self) -> Result<&T, Me>;
    /// Even median for any PartialOrd type T 
    fn generic_even(&self) -> Result<(&T,&T), Me>;
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

**`generic_even`**

When the data items are unquantifiable, we can not simply average the two midpoints of even length data, as we did in `median`. So we return them both as a pair tuple, the smaller one first. Otherwise very similar to `generic_odd`. Note however, that these two methods return results of different types: `(&T,&T)` and `&T` respectively. Therefore the user has to deal with them explicitly, as appropriate.

## `Struct MStats`

Holds the sample parameters: centre (here the median), and the spread measure, (here MAD = median of absolute differences from the median). MAD is the most stable measure of data spread. Alternatively, MStats can hold the mean and the standard deviation, as computed in crate RStats.

## Release Notes

**Version 2.2.2** - Corrected some comment and readme typos. No change in functionality.

**Version 2.2.1** - Some code pruning and streamlining. No change in functionality.

**Version 2.2.0** - Major new version with much improved speed and generality and some breaking changes (renaming).

**Version 2.1.7** - More pruning and test improvements.

**Version 2.1.6** - Fixed a bug in `fmin2` and `fmax2` that made the median one off the centre sometimes. Apologies. Pruned some code.

**Version 2.1.3** - Simplified odd/even strict medians. Updated indxvec dependency. Fixed dependency and tests.

**Version 2.1.1** - Simplified/improved the display of struct MStats.

**Version 2.1.0** - Added omitted method `zeromedianf64`. Upped the version.