# Medians

[<img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/liborty/medians/HEAD?logo=github">](https://github.com/liborty/medians)
[<img alt="crates.io" src="https://img.shields.io/crates/v/medians?logo=rust">](https://crates.io/crates/medians)
[<img alt="crates.io" src="https://img.shields.io/crates/d/medians?logo=rust">](https://crates.io/crates/medians)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/medians?logo=rust">](https://docs.rs/medians)

Fast new algorithm for finding 1D medians, implemented in Rust.  
(Do not use, not ready yet, apart from the `naive_median`)

## Introduction

Finding the medians is a common task in statistics and data analysis. At least it should be, if only it did not take so long.

We argue in [rstats](https://github.com/liborty/rstats) that using the Geometric Median is the most stable way to charactarise multidimensional data. There, we solved the problem of finding it efficiently in n dimensions by implementing a stable algorithm with good convergence (an improved Weiszfeld algorithm).

That leaves the one dimensional case, where the medians are not used nearly enough, due to being slower to find than the arithmetic mean.

The median can be found simply by sorting the list of data and then picking the midpoint. The only problem with this 'naive' approach is that, even when using a good quality sort with guaranteed performance, such as the Merge Sort, the complexity is O(n log n).

The quest for faster algorithms with complexity O(n) is motivated by the simple observation, that not all items need to be fully sorted.

## The Algorithms

Floyd-Rivest with the 'Median of Medians' approximation is currently considered to be the best algorithm.

Here we present these algorithms:

* `naive_median` uses sort. It is relatively slow but is useful for comparisons. It gives reliable and exact results.

* `w_median` is a specialisation of n dimensional `gmedian` from `rstats` to one dimensional case. It is iterative and thus even slower than `naive_median` (over large sets of the order of thousands of items).

* `i_median` is promising. It is consistently faster (by about 50%) than any of the above.

There is at least one more algorithms in the pipeline.

## Conclusion
