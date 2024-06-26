# Medians

[![crates.io](https://img.shields.io/crates/v/medians?logo=rust)](https://crates.io/crates/medians) [![crates.io](https://img.shields.io/crates/d/medians?logo=rust)](https://crates.io/crates/medians) [!["GitHub last commit"](https://img.shields.io/github/last-commit/liborty/medians/HEAD?logo=github)](https://github.com/liborty/medians) [![Actions Status](https://github.com/liborty/medians/workflows/test/badge.svg)](https://github.com/liborty/random/actions)

## **by Libor Spacek**

Fast new algorithms for finding medians, implemented in 100% safe Rust.

```rust
use medians::{*,algos::*};
```

## Introduction

Finding medians is a common task in statistics and data analysis. At least it ought to be, because median is a more stable measure of central tendency than mean. Similarly, `mad` (median of absolute differences) is a more stable measure of data spread than standard deviation, which is dominated by squared outliers. Median and `mad` are not used nearly enough mostly for practical historical reasons: they are more difficult to compute. The fast algorithms presented here provide a remedy for this situation.

We argued in [`rstats`](https://github.com/liborty/rstats), that using the Geometric Median is the most stable way to characterise multidimensional data. The one dimensional case is addressed in this crate.

See [`tests.rs`](https://github.com/liborty/medians/blob/main/tests/tests.rs) for examples of usage. Their automatically generated output can also be found by clicking the 'test' icon at the top of this document and then examining the latest log.

## Outline Usage

Best methods/functions to be deployed, depending on the end type of data (i.e. type of the items within the input vector/slice).

- `u8` -> function `medianu8`
- `u64` -> function `medianu64`
- `f64` -> methods of trait Medianf64
- `T` custom quantifiable to u64 -> method `uqmedian` of trait `Median`
- `T` custom comparable by `c` -> method `qmedian_by` of trait `Median`
- `T` custom comparable but not quantifiable -> general method `median_by` of trait `Median`.

## Algorithms Analysis

Short primitive types are best dealt with by radix search. We have implemented it for `u8` and for `u64`:

```rust
/// Medians of u8 end type by fast radix search
pub fn medianu8(s: &[u8]) -> Result<ConstMedians<u8>, Me>;
/// Medians of u64 end type by fast recursive radix search
pub fn medu64(s: &mut [u64]) -> Result<(u64, u64), Me>;
```

More complex data types require general comparison search, see `median_by`. Median can be found naively by sorting the list of data and then picking its midpoint. The best comparison sort algorithms have complexity `O(n*log(n))`. However, faster median algorithms with complexity `O(n)` are possible. They are based on the observation that data need to be all fully sorted, only partitioned and counted off. Therefore, the naive sort method can not compete and has been deleted as of version 2.0.0.

Floyd-Rivest (1975): Median of Medians is currently considered to be 'the state of the art' comparison algorithm. It divides the data into groups of five items, finds median of each group by sort, then finds medians of five of these medians, and so on, until only one remains. This is then used as the pivot for partitioning of the original data. Such pivot will produce good partitioning, though not perfect halving. Counting off and iterating is therefore still necessary.

Finding the best possible pivot estimate is not the main objective. The real objective is to eliminate (count off) eccentric data items as fast as possible, overall. Therefore, the time spent estimating the pivot has to be taken into account. It is possible to settle for less optimal pivots, yet to find the medians faster on average. In any case, efficient partitioning is a must.

Let our average ratio of items remaining after one partitioning be `rs` and the Floyd-Rivest's be `rf`. Typically, `1/2 <= rf <= rs < 1`, i.e. `rf` is more optimal, being nearer to the perfect halving (ratio of `1/2`). Suppose that we can perform two partitions in the time it takes Floyd-Rivest to do one (because of their slow pivot selection). Then it is enough for better performance that `rs^2 < rf`, which is perfectly possible and seems to be born out in practice. For example, `rf=0.65` (nearly optimal), `rs=0.8` (deeply suboptimal), yet `rs^2 < rf`. Nonetheless, some computational effort devoted to the pivot selection, proportional to the data length, is worth it.

We introduce another new algorithm, implemented as function `medianu64`:

```rust
/// Fast medians of u64 end type by binary partitioning
pub fn medianu64(s: &mut [u64]) -> Result<ConstMedians<u64>, Me>
```

  on `u64` data, this runs about twice as fast as the general purpose pivoting of `median_by`. The data is partitioned by individual bit values, totally sidestepping the expense of the pivot estimation. The algorithm generally converges well. However, when the data happens to be all bunched up within a small range of values, it will slow down.

### Summary of he main features of our general median algorithm

- Linear complexity.
- Fast (in-place) iterative partitioning into three subranges (lesser,equal,greater), minimising data movements and memory management.
- Simple pivot selection strategy: median of three samples (requires only three comparisons). Really poor pivots occur only rarely during the iterative process. For longer data, we deploy median of three medians.

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
    /// Median of types quantifiable to u64 by `q`, at the end converted to a single f64.  
    /// For data that is already `u64`, use function `medianu64`
    fn uqmedian(
            self,
            q: impl Fn(&T) -> u64,
        ) -> Result<f64, Me>;
    /// Median by comparison `c`, returns odd/even result
    fn median_by(self, c: &mut impl FnMut(&T, &T) -> Ordering) -> Result<Medians<'a, T>, Me>;
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

**Version 3.0.12** - Adding faster `medu64`, even variant is still work in progress. Fixed a bug.

**Version 3.0.11** - Added method `uqmedian` to trait `Median` for types quantifiable to `u64` by some closure `q`. Fixed a recent bug in `oddmedian_by`, whereby the pivot reference was not timely saved.

**Version 3.0.10** - Added `medianu64`. It is faster on u64 data than the general purpose `median_by`. It is using a new algorithm that partitions by bits, thus avoiding the complexities of pivot estimation.

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
