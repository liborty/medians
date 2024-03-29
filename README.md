# Medians [![crates.io](https://img.shields.io/crates/v/medians?logo=rust)](https://crates.io/crates/medians) [![crates.io](https://img.shields.io/crates/d/medians?logo=rust)](https://crates.io/crates/medians) [!["GitHub last commit"](https://img.shields.io/github/last-commit/liborty/medians/HEAD?logo=github)](https://github.com/liborty/medians) [![Actions Status](https://github.com/liborty/medians/workflows/test/badge.svg)](https://github.com/liborty/random/actions)

## **by Libor Spacek**

Fast algorithm for finding medians of one dimensional data, implemented in 100% safe Rust.

```rust
use medians::{*,algos::*};
```

## Introduction

Finding medians is a common task in statistics and data analysis. At least it ought to be, because median is a more stable measure of central tendency than mean. Similarly, `mad` (median of absolute differences) is a more stable measure of data spread than standard deviation, which is dominated by squared outliers. Median and `mad` are not used nearly enough mostly for practical historical reasons: they are more difficult to compute. The fast algorithms presented here provide remedy for this situation.

We argued in [`rstats`](https://github.com/liborty/rstats), that using the Geometric Median is the most stable way to characterise multidimensional data. The one dimensional case is addressed here.

See [`tests.rs`](https://github.com/liborty/medians/blob/main/tests/tests.rs) for examples of usage. Their automatically generated output can also be found by clicking the 'test' icon at the top of this document and then examining the latest log.

## Algorithms Analysis

Short primitive types are best dealt with by radix search. We have implemented it for `u8`:

```rust
/// Median of primitive type u8 by fast radix search
pub fn medianu8( s:&[u8] ) -> Result<f64, Me> { ... }
```

More complex data types require general comparison search. Median can be found naively by sorting the list of data and then picking its midpoint. The best comparison sort algorithms have complexity `O(n*log(n))`. However, faster median algorithms, with complexity `O(n)` are possible. They are based on the observation that data need to be sorted, only partitioned and counted off. Therefore, the naive sort method can not compete and has been deleted as of version 2.0.0.

Currently considered to be the 'state of the art' comparison algorithm is Floyd-Rivest (1975): Median of Medians. This divides the data into groups of five items, finds a median of each group by sort and then recursively finds medians of five of these medians, and so on, until only one is left. This is then used as a pivot for the partitioning of the original data. Such pivot will produce reasonably good partitioning, though not necessarily perfect. Therefore, iteration is still necessary.

However, finding the best pivot is not the main objective. Rather, it is to eliminate (count off) eccentric data items as fast as possible. Therefore, the expense of choosing the pivot must be carefully considered. It is possible to use less optimal pivot, and yet to find the median faster (on average).

Let our average ratio of items remaining after one partitioning be `rs` and the Floyd-Rivest's be `rf`. Typically, `1/2 <= rf <= rs < 1`, i.e. `rf` is more optimal, being nearer to the perfect partitioning ratio of `1/2`. However, suppose that we can perform two partitions in the time it takes Floyd-Rivest to do one (because of their expensive pivot selection process). Then it is enough for better performance that `rs^2 < rf`, which is perfectly possible and seems to be born out in practice. For example, `rf=0.65` (nearly optimal), `rs=0.8` (deeply suboptimal), yet `rs^2 < rf`.

Nonetheless, especially on large datasets, one should devote certain limited fraction of the overall computational effort to pivot selection.

### Summary of he main features of our median algorithm

* Linear complexity.
* Fast (in-place) iterative partitioning into three subranges (lesser,equal,greater), minimising data movements and memory management.
* Simple pivot selection strategy: median of three samples (requires only three comparisons). Really poor pivots occur only rarely during the iterative process. For longer data, we do deploy median of three medians but again only on a small sub sample of data.

## Trait Medianf64

```rust
/// Fast 1D medians of floating point data, plus related methods
pub trait Medianf64 {
    /// Median of f64s, NaNs removed
    fn medf_checked(self) -> Result<f64, Me>;
    /// Median of f64s, including NaNs
    fn medf_unchecked(self) -> f64;
    /// Iterative weighted median
    fn medf_weighted(self, ws: Self, eps: f64) -> Result<f64, Me>;
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
    fn med_correlation(
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

**Version 3.0.9** - Improved pivot estimation for large data sets.

**Version 3.0.8** - Added `implementation.rs` module and reorganized the source.

**Version 3.0.7** - Added `medf_weighted`, applying `&[f64]` weights.

**Version 3.0.6** - Moved `part`, `ref_vec` and `deref_vec` into crate `Indxvec`, to allow their wider use.

**Version 3.0.5** - Obsolete code pruning.

**Version 3.0.4** - Some minor code simplifications.

**Version 3.0.3** - Updated dev dependency `ran` to 2.0.

**Version 3.0.2** - Added function `medianu8` that finds median byte by superfast radix search. More primitive types to follow.

**Version 3.0.1** - Renamed `correlation` to `med_correlation` to avoid name clashes elsewhere.

**Version 3.0.0** - Numerous improvements to speed and generality and renaming.
