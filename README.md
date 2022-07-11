# Medians

[<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/liborty/medians/HEAD?logo=github">](https://github.com/liborty/medians)
[<img alt="crates.io" src="https://img.shields.io/crates/v/medians?logo=rust">](https://crates.io/crates/medians)
[<img alt="crates.io" src="https://img.shields.io/crates/d/medians?logo=rust">](https://crates.io/crates/medians)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/medians?logo=rust">](https://docs.rs/medians)

Fast new algorithm(s) for finding 1D medians, implemented in Rust. 

## Usage

```rust
use medians::{Med,Median};
```

## Introduction

Finding the medians is a common task in statistics and general data analysis. At least it should be, if only it would not take so long. We argue in [rstats](https://github.com/liborty/rstats) that using the Geometric Median is the most stable way to characterise multidimensional data (nd). That leaves the one dimensional (1d) medians, addressed here. Medians are more stable measure of central tendency than means but they are not used nearly enough. One suspects that this is due only to being slower to compute than the arithmetic mean.

## The Algorithms

Floyd-Rivest with the 'Median of Medians' approximation is currently considered to be the best algorithm. Here we explore some alternatives:

* `naive_median`  
  is a useful baseline for time comparisons. So our performance comparisons (see `tests.rs`) take it as 100%. The median is found simply by sorting the list of data and then picking the midpoint. In this case, the fastest standard Rust `sort_unstable_by` is used.

  The problem with this approach is that, even when using a good quality sort with guaranteed performance, its complexity is at best O(n log n). The quest for faster median algorithms, with complexity O(n), is motivated by the observation that not all items need to be fully sorted.

* `w_median`  
is a specialisation of n dimensional `gmedian` from [rstats](https://github.com/liborty/rstats) to one dimensional case. It starts at about 84% of naive time for very short vecs. For orders of magnitude 2 to 3 it runs at about 45%. Then it starts slowing down. At the order of 5 and above it becomes slower than `naive_median`.

* `r_median`
recursively partitions data around a pivot computed by a specialised secant method using passed down minimum and maximum values. Beats all other algorithms on vecs of lengths of about 60 upwards. At the order of magnitude 4 it runs at just over 12% and at 5 it runs at just over 10% of the 'naive' time (on f64 data).

* `median`
is the main public entry point, implemented as a method of trait `Median`. It is just a 'big switch'. Depending on the length of the input vector, it calls either `w_median` or `r_median`, in order to always get the best performance.

## Struct Med

Holds the median, lower and upper quartiles and MAD (median of absolute differences from median). MAD is the most stable measure of data spread.

## Trait Median

```rust
pub trait Median<T> {
    /// Finds the median of `&[T]`, fast
    fn median(&self) -> f64;
    /// Median of absolute differences (MAD).
    fn mad(self,m:f64) -> f64;
    /// Median, quartiles and MAD
    fn medinfo(self) -> Med;
}
```


## Release Notes

**Version 1.0.0** -  Update to the latest `indxvec` dependency, v. 1.2.10.

**Version 0.1.2** - The public methods are now in trait Median.