# Medians [<img alt="crates.io" src="https://img.shields.io/crates/v/medians?logo=rust">](https://crates.io/crates/medians) [<img alt="crates.io" src="https://img.shields.io/crates/d/medians?logo=rust">](https://crates.io/crates/medians) [<img alt="docs.rs" src="https://img.shields.io/docsrs/medians?logo=rust">](https://docs.rs/medians) [<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/liborty/medians/HEAD?logo=github">](https://github.com/liborty/medians) [![Actions Status](https://github.com/liborty/medians/workflows/test/badge.svg)](https://github.com/liborty/medians/actions)

Fast algorithm(s) for finding 1d median, implemented in Rust.

## Usage

```rust
use medians::{Med,MStats,Median};
```

## Introduction

Finding the medians is a common task in statistics and general data analysis. At least it should be, if only it would not take so long. We argue in [`rstats`](https://github.com/liborty/rstats), that using the Geometric Median is the most stable way to characterise multidimensional data (nd). That leaves the one dimensional (1d) medians, addressed here. Medians are more stable measure of central tendency than means but they are not used nearly enough. One suspects that this is mostly due to being slower to compute than the arithmetic mean.

See [`tests.rs`](https://github.com/liborty/medians/blob/main/tests/tests.rs) as examples of usage. Their automatically generated output can be seen by clicking the 'test' icon at the top of this document and then examining the latest log.

## The Algorithms

* `naive_median`  
  is a useful baseline for time comparisons in our performance benchmark (see [`tests.rs`](https://github.com/liborty/medians/blob/main/tests/tests.rs)). The naive median is found simply by sorting the list of data and then picking the midpoint. In this case, the fastest standard Rust `sort_unstable_by` is used.

  The problem with this approach is that, even when using a good quality sort with guaranteed performance, its complexity is at best `O(n log n)`. The quest for faster median algorithms, with complexity `O(n)`, is based on the observation that not all items need to be fully sorted.

* `r_median`
recursively partitions data around a pivot computed by a specialised secant method using passed down minimum and maximum values. Beats all other algorithms on `Vecs` of lengths of 50 upwards. At the order of magnitude 4 it runs at just over 12% and at 5 it runs at just over 10% of the 'naive' time (on f64 data).  In other words, it is approaching the linear complexity.

* `median`
is the main public entry point, implemented as a method of trait `Median`. It is just a 'switch' between `naive_median`, used only for very short data vectors, and `r_median`, for longer ones. Thus it gives optimal performance over all lengths of data and is the recommended method to use.

## Structs

* **MStats** - centre (here the median), dispersion, (here MAD = median of absolute differences from median). MAD is the most stable measure of data spread.
* **Med** - median, lower and upper quartiles, MAD and standard error.

## Trait Median

```rust
/// Finding 1D medians, quartiles, and MAD (median of absolute differences)
pub trait Median {
    /// Finds the median of `&[T]`, fast
    fn median(self) -> Result<f64,MedError<String>>;
    /// Median of absolute differences (MAD).
    fn mad(self, median: f64) -> Result<f64,MedError<String>>;
    /// Median and MAD.
    fn medstats(self) -> Result<MStats,MedError<String>>;
    /// Median, quartiles, MAD, Stderr
    fn medinfo(self) -> Result<Med,MedError<String>>;
}
```

## Release Notes

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
