# Medians

[<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/liborty/medians/HEAD?logo=github">](https://github.com/liborty/medians)
[<img alt="crates.io" src="https://img.shields.io/crates/v/medians?logo=rust">](https://crates.io/crates/medians)
[<img alt="crates.io" src="https://img.shields.io/crates/d/medians?logo=rust">](https://crates.io/crates/medians)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/medians?logo=rust">](https://docs.rs/medians)

Fast new algorithm(s) for finding 1D medians, implemented in Rust.  

## Introduction

Finding the medians is a common task in statistics and data analysis. At least it should be, if only it did not take so long.

We argue in [rstats](https://github.com/liborty/rstats) that using the Geometric Median is the most stable way to characterise multidimensional data.

That leaves the one dimensional case, where the medians are not used nearly enough either, due to being much slower to find than the arithmetic mean.

## The Algorithms

Floyd-Rivest with the 'Median of Medians' approximation is currently considered to be the best algorithm. Here we explore some alternatives:

* `naive_median`  
is a useful baseline for executions time comparisons, which we take as 100%. It gives reliable and exact results. The median is found simply by sorting the list of data and then picking the midpoint. In this case using the fastest standard Rust `sort_unstable_by`. The problem with this approach is that, even when using a good quality sort with guaranteed performance, its complexity is at best O(n log n). The quest for faster median algorithms, with complexity O(n), is motivated by the observation that not all items need to be fully sorted.

* `w_median`  
is a specialisation of n dimensional `gmedian` from [rstats](https://github.com/liborty/rstats) to one dimensional case. It is iterative. It starts at about 84% of naive time for very short vecs. For orders of magnitude 2 to 3 it runs at about 45%. Then it starts slowing down. At the order of 5 and above it becomes actually slower.

* `r_median` 
recursively partitions the data around a pivot computed as a secant based on minimum and maximum values.
Beats all other algorithms on vecs of length 107 upwards. At the order of magnitude 5 it runs at mere 13% of the 'naive' time.

There is at least one more algorithm in the pipeline.
