# Medians [![crates.io](https://img.shields.io/crates/v/medians?logo=rust)](https://crates.io/crates/medians) [![crates.io](https://img.shields.io/crates/d/medians?logo=rust)](https://crates.io/crates/medians) [!["GitHub last commit"](https://img.shields.io/github/last-commit/liborty/medians/HEAD?logo=github)](https://github.com/liborty/medians) [![Actions Status](https://github.com/liborty/medians/workflows/test/badge.svg)](https://github.com/liborty/random/actions)

## **by Libor Spacek**

Fast algorithm for finding medians of one dimensional data, implemented in 100% safe Rust.

```rust
use medians::{Medians,Medianf64,Median};
```

## Introduction

Finding medians is a common task in statistics and data analysis. At least it ought to be, because median is a more stable measure of central tendency than mean. Similarly, `mad` (median of absolute differences) is a more stable measure of data spread than standard deviation, dominated by squared outliers. Median and mad are not used nearly enough mostly for practical historical reasons: they are more difficult to compute. The fast algorithms presented here provide a practical remedy for this situation.

We argued in [`rstats`](https://github.com/liborty/rstats), that using the Geometric Median is the most stable way to characterise multidimensional data. The one dimensional case is addressed here.

See [`tests.rs`](https://github.com/liborty/medians/blob/main/tests/tests.rs) for examples of usage. Their automatically generated output can also be found by clicking the 'test' icon at the top of this document and then examining the latest log.

## Algorithms Analysis

Median can be found naively by sorting the list of data and then picking its midpoint. The best comparison sort algorithm(s) have complexity `O(nlog(n))`. However, faster median algorithms, with complexity `O(n)` are possible. They are based on the observation that not all data need to be sorted, only partitioned and counted off. Therefore, the sort method can not compete, as is demonstrated by the tests. It has been deleted as of version 2.0.0.

Currently considered to be the 'state of the art' algorithm is Floyd-Rivest (1975) Median of Medians. This divides the data into groups of five items, finds a median of each group and then recursively finds medians of five of these medians, and so on, until only one is left. This is then used as a pivot for the partitioning of the original data. Such  pivot is guaranteed to produce 'pretty good' partitioning, though not necessarily perfect, so iteration is necessary.

However, to find the best pivot is not the main overall objective. Rather, it is to eliminate (count off) eccentric data items as fast as possible. Therefore, the expense of choosing the pivot must be considered. It is possible to allow less optimal pivots, as we do here and yet, on average, to find the median faster.

Let our average ratio of items remaining after one partitioning be rs and the Floyd-Rivest be rf. Where `1/2 <= rf <= rs < 1` and rf is more optimal, being nearer to the perfect ratio `1/2`. However, suppose that we can perform two partitions in the time it takes Floyd-Rivest to do one (because of their expensive pivot selection process). Then it is enough for better performance that `rs^2 < rf`, which is entirely possible and seems to be confirmed in practice. For example, `rf=0.65` (nearly optimal), `rs=0.8` (deeply suboptimal), yet `rs^2 < rf`.

The main features of our median algorithm are:

* Linear complexity.
* Fast (in place) iterative partitioning into three subranges (lesser,equal,greater), minimising data movements and memory management.
* Simple pivot selection strategy. We define the `middling` value of a sample of four as one of the middle pair of sorted items. Whereas full sort of four items takes at least five comparisons, we only need three. A `middling` pivot is enough to guarantee convergence of iterative schemes, such as the search for the median. Also, poor pivots are unlikely to be picked repeatedly.

## Trait Medianf64

```rust
/// Fast 1D medians of floating point data, plus related methods
pub trait Medianf64 {
    /// Median of f64s, checked for NaNs
    fn medf_checked(self) -> Result<f64, Me>;
    /// Median of f64s, not checked for NaNs
    fn medf_unchecked(self) -> f64;
    /// Zero mean/median data produced by subtracting the centre
    fn medf_zeroed(self, centre: f64) -> Vec<f64>;
    /// Median correlation = cosine of an angle between two zero median vecs
    fn medf_correlation(self, v: Self) -> Result<f64, Me>;
    /// Median of absolute differences (MAD).
    fn madf(self, centre: f64) -> f64;
}
```

## Trait Median

These methods are provided especially for generic, arbitrarily complex and/or large data end-types. The data is never copied during partitioning, etc.

Most of its methods take a comparison closure `c` which returns an ordering between its arguments of generic type `&T`. This allows comparisons in any number of different ways between any custom types.

Most of its methods take a quantify closure `q`, which converts its generic argument to f64. This facilitate not just standard Rust `as` and `.into()` conversions but also any number of flexible ways of quantifying more complex custom data types.

Weaker partial ordinal comparison is used instead of numerical comparison. The search algorithm remains the same. The only additional cost is the extra layer of referencing to prevent the copying of data.

**`median_by()`**  
For all end-types quantifiable to f64, we simply averaged the two midpoints of even length data to obtain a single median (of type `f64`). When the data items are unquantifiable to `f64`, this is no longer possible. Then `median_by` should be used. It returns both middle values within `Medians` enum type, the lesser one first:

```rust
/// Enum for results of odd/even medians
pub enum Medians<'a, T> {
    /// Odd sized data results in a single median
    Odd(&'a T),
    /// Even sized data results in a pair of (centered) medians
    Even((&'a T, &'a T)),
}
```

```rust
/// Fast 1D generic medians, plus related methods
pub trait Median<'a, T> {
    /// Median by comparison `c`, at the end quantified to a single f64 by `q`
    fn qmedian_by(
        self,
        c: &mut impl FnMut(&T, &T) -> Ordering,
        q: impl Fn(&T) -> f64,
    ) -> Result<f64, Me>;
    /// Median by comparison `c`, returns odd/even result
    fn median_by(self, c: &mut impl FnMut(&T, &T) -> Ordering) 
        -> Result<Medians<'a, T>, Me>;
    /// Zero mean/median data, produced by subtracting the centre
    fn zeroed(self, centre: f64, quantify: impl Fn(&T) -> f64) -> Result<Vec<f64>, Me>;
    /// Median correlation = cosine of an angle between two zero median Vecs
    fn correlation(
        self,
        v: Self,
        c: &mut impl FnMut(&T, &T) -> Ordering,
        q: impl Fn(&T) -> f64,
    ) -> Result<f64, Me>;
    /// Median of absolute differences (MAD).
    fn mad(self, centre: f64, quantify: impl Fn(&T) -> f64) -> f64;
}
```

## Release Notes

**Version 3.0.0** - Numerous improvements to speed and generality and renaming. 

**Version 2.3.1** - Further speed optimisation of `partf64`.

**Version 2.3.0** - Some minor changes to `algosf64.rs`. Improvements to this manual.

**Version 2.2.6** - Improved `README.md`. No changes to the code.

**Version 2.2.5** - Upped dependency on `indxvec` to version 1.8.

**Version 2.2.3** - Slight further improvement to efficiency of `part`.

**Version 2.2.2** - Corrected some comment and readme typos. No change in functionality.

**Version 2.2.1** - Some code pruning and streamlining. No change in functionality.

**Version 2.2.0** - Major new version with much improved speed and generality and some breaking changes (renaming).
