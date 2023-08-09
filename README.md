# Medians [<img alt="crates.io" src="https://img.shields.io/crates/v/medians?logo=rust">](https://crates.io/crates/medians) [<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/liborty/medians/HEAD?logo=github">](https://github.com/liborty/medians) [![Actions Status](https://github.com/liborty/medians/workflows/test/badge.svg)](https://github.com/liborty/medians/actions)

### **by Libor Spacek**

Fast algorithm for finding medians of one dimensional data, implemented in 100% safe Rust.

### Usage

```rust
use medians::{MStats,Medianf64,Median};
```

## Introduction

Finding medians is a common task in statistics and data analysis. At least it ought to be, because median is a more stable measure of central tendency than mean. Similarly, `mad` (median of absolute differences from median) is a more stable measure of data spread than standard deviation. Median and mad are not used nearly enough mostly for historical reasons: they are more difficult to compute. The fast algorithms presented here provide a practical remedy for this situation.

We argued in [`rstats`](https://github.com/liborty/rstats), that using the Geometric Median is the most stable way to characterise multidimensional data. The one dimensional case is addressed here.

See [`tests.rs`](https://github.com/liborty/medians/blob/main/tests/tests.rs) for examples of usage. Their automatically generated output can also be found by clicking the 'test' icon at the top of this document and then examining the latest log.

## Algorithms Analysis

Median can be found naively by sorting the list of data and then picking the midpoint. When using the best known sort algorithm(s), the complexity is `O(nlog(n))`. Faster median algorithms, with complexity `O(n)` are possible. They are based on the observation that not all data items need to be fully sorted, only partitioned and counted off. Therefore the naive method can not compete. It has been deleted as of version 2.0.0.

Currently considered to be the 'state of the art' algorithm is Floyd-Rivest (1975) Median of Medians. This divides the data into groups of five items, finds a median of each group and then recursively finds medians of five of these medians, and so on, until only one is left. This is then used as a pivot for the partitioning of the original data. Such  pivot is guaranteed to produce 'pretty good' partitioning, though not necessarily perfect, so iteration is necessary.

However, to find the best pivot is not the main overall objective. Rather, it is to eliminate (count off) eccentric data items as fast as possible. Therefore, the expense of choosing the pivot must be considered. It is possible to allow less optimal pivots, as we do here and yet, on average, to find the median faster.

Let our average ratio of items remaining after one partitioning be rs and the Floyd-Rivest be rf. Where `1/2 <= rf <= rs < 1` and rf is more optimal, being nearer to the perfect ratio `1/2`. However, suppose that we can perform two partitions in the time it takes Floyd-Rivest to do one (because of their expensive pivot selection process). Then it is enough for better performance that `rs^2 < rf`, which is entirely possible and seems to be confirmed in practice. For example, `rf=0.65` (nearly optimal), `rs=0.8` (deeply suboptimal), yet `rs^2 < rf`.

The main features of our median algorithm are:

* Linear complexity.
* Fast (in place) iterative partitioning, minimising data movements and memory management.
* Simple pivot selection strategy (median of a sample of three). This is enough to guarantee convergence, even in the worst case. However, poor pivots are unlikely to be picked repeatedly.

## Trait `Medianf64`

```rust
/// Fast 1D f64 medians and associated tasks
pub trait Medianf64 {
    /// Finds the median, fast. 
    fn median(self) -> Result<f64, Me>;  
     /// finds and subtracts the median from data. 
    fn zeromedian(self) -> Result<Vec<f64>, Me>;
    /// Median correlation = cosine of an angle between two zero median vecs
    fn mediancorr(self,v: &[f64]) -> Result<f64, Me>;
    /// Median of absolute differences (MAD).
    fn mad(self, med: f64) -> Result<f64, Me>;
    /// Median and MAD.
    fn medstats(self) -> Result<MStats, Me>;
}
```

## Trait `Median`

Is the generic version of `Medianf64`. Most of its methods take an extra argument, a `quantify` closure, which evaluates (converts) an argument of generic type T to f64. This extends applicability of `Median` to all quantifiable generic types. The closures facilitate not just standard Rust `as` and `.into()` conversions but also any number of custom ways of quantifying more complex data types. The conversion incurs some extra cost but it is only done once.

Five of the methods of this trait fulfil the same roles as those in trait `Medianf64` above. Instead of renaming them, we use auto referencing.

**`generic_odd, generic_even`**

These extra two methods are provided specifically for arbitrarily complicated data items, for which the quantification is not possible. Also, they are particularly suitable for large data types, as the data is not moved around.

Weaker partial ordinal comparison is used instead of numerical comparison. The search algorithm remains the same. References to the central item(s) are returned instead of the items themselves. The only additional cost is this extra layer of referencing.

In trait `Medianf64` we simply averaged the two midpoints of even length data to obtain a single median of type `f64`. When the data items are unquantifiable, we can no longer do that. Instead, we return them both as a pair tuple, the lesser one first. Therefore these two methods for odd and even length data return results of types: `&T` and `(&T,&T)` respectively and users have to deal with them appropriately.

```rust
/// Fast 1D generic medians and associated information and tasks.  
/// Using auto referencing to disambiguate conflicts 
/// with five more specific Medianf64 methods with the same names.  
/// To invoke specifically these generic versions, add a reference:  
/// `(&v[..]).method` or `v.as_slice().method`
pub trait Median<T> {
    /// Finds the median of `&[T]`, fast. 
    fn median(&self, quantify: impl Fn(&T) -> f64) -> Result<f64, Me>; 
    /// Odd median for any PartialOrd type T 
    fn generic_odd(&self) -> Result<&T, Me>;
    /// Even median for any PartialOrd type T 
    fn generic_even(&self) -> Result<(&T,&T), Me>;
    /// Zero median data produced by finding and subtracting the median. 
    fn zeromedian(&self, quantify: impl Copy + Fn(&T) -> f64) -> Result<Vec<f64>, Me>;
    /// Median correlation = cosine of an angle between two zero median vecs
    fn mediancorr(&self,v: &[T],quantify: impl Copy + Fn(&T) -> f64) -> Result<f64, Me>;
    /// Median of absolute differences (MAD).
    fn mad(&self, med: f64, quantify: impl Fn(&T) -> f64) -> Result<f64, Me>;
    /// Median and MAD.
    fn medstats(&self, quantify: impl Copy + Fn(&T) -> f64) -> Result<MStats, Me>;
}
```

## `Struct MStats`

Holds the sample parameters: centre (here the median), and the spread measure, (here MAD = median of absolute differences from the median). MAD is the most stable measure of data spread. Alternatively, MStats can hold the mean and the standard deviation, as computed in crate RStats.

## Release Notes

**Version 2.3.1** - Minor further speed optimisation of `partf64`

**Version 2.3.0** - Some minor changes to `algosf64.rs`. Improvements to this manual.

**Version 2.2.6** - Improved `README.md`. No changes to the code.

**Version 2.2.5** - Upped dependency on `indxvec` to version 1.8.

**Version 2.2.3** - Slight further improvement to efficiency of `part`.

**Version 2.2.2** - Corrected some comment and readme typos. No change in functionality.

**Version 2.2.1** - Some code pruning and streamlining. No change in functionality.

**Version 2.2.0** - Major new version with much improved speed and generality and some breaking changes (renaming).
