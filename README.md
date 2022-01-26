# Medians
Fast new algorithm for finding the medians, implemented as a Rust crate (library module).

## Introduction
Median can be found simply and easily by sorting the list of data and then picking the midpoint (and quartiles).
The only problem with this approach is that, even when using a good quality sort with guaranteed performance, such as the Merge sort,
the complexity is O(n log n).

It is possible to approach O(n). This is the main claim of what is currently considered to be the best algorithm: 'Median of Medians'.
Here we compete against this algorithm and run some comparisons.
