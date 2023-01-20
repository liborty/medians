use crate::{here, compare, Vecops};
use core::ops::Range;
use std::cmp::Ordering;

/*
fn min<T: PartialOrd>(s: &[T], idx: &[usize], rng: Range<usize>) -> usize {
    let mut mini = idx[rng.start];
    for i in rng.skip(1) {
        if s[idx[i]] < s[mini] {
            mini = idx[i];
        };
    }
    mini
}

fn min2<T: PartialOrd>(s: &[T], idx: &[usize], rng: Range<usize>) -> (usize, usize) {
    let mut min1 = idx[rng.start];
    let mut min2 = min1;
    for i in rng.skip(1) {
        if s[idx[i]] < s[min1] {
            min2 = min1;
            min1 = idx[i];
        } else if s[idx[i]] < s[min2] {
            min2 = idx[i];
        }
    }
    (min1, min2)
}

fn max<T: PartialOrd>(s: &[T], idx: &[usize], rng: Range<usize>) -> usize {
    let mut max = idx[rng.start];
    for i in rng.skip(1) {
        if s[idx[i]] > s[max] {
            max = idx[i];
        };
    }
    max
}

/// Iterative median, partitioning data range by median of medians as an estimated pivot.
pub fn strict_median<T>(set: &[T], quantify: &mut impl FnMut(&T) -> f64) -> f64 {
    let n = set.len();
    let mut pivot = 0_f64;
    let fset = set
        .iter()
        .map(|tval| {
            let fval = quantify(tval);
            pivot += fval;
            fval
        })
        .collect::<Vec<f64>>();
    pivot /= n as f64; // using arithmetic mean as the pivot
    if (n & 1) == 1 {
        strict_med_odd(fset, 0..n, pivot)
    } else {
        strict_med_even(fset, 0..n, pivot)
    }
}

/// Median of an odd sized set is the central value.
fn strict_med_odd<'a,T:PartialOrd>(set: &'a[T], idx: &[usize]) -> &'a T {
    let need = rng.len() / 2; // need as subscript (one less)
    loop {
        let gtsub = fpart(&mut set, &rng, pivot);
        if need < gtsub {
            rng.end = gtsub;
            if need + 1 == gtsub {
                return fmax(&set, rng.start..gtsub);
            };
        } else {
            rng.start = gtsub;
            if need == gtsub {
                return fmin(&set, gtsub..rng.end);
            };
        };
        let newpivot = set.iter().take(rng.end).skip(rng.start).sum::<f64>() / rng.len() as f64;
        if newpivot == pivot {
            return pivot;
        }
        // in equals region
        else {
            pivot = newpivot;
        };
    }
}

/// Median of an even sized set are both of its central values.
pub fn strict_med_even<T: PartialOrd>(mut set: &[T], idx: &[usize]) -> (&T, &T) {
    let need = rng.len() / 2 - 1;
    loop {
        let gtsub = fpart(&mut set, &rng, pivot);
        if need < gtsub {
            if need + 1 == gtsub {
                return (fmax(&set, rng.start..gtsub) + fmin(&set, gtsub..rng.end)) / 2.;
            };
            rng.end = gtsub;
        } else {
            if need == gtsub {
                fmin2(&set, gtsub..rng.end);
            }
            rng.start = gtsub;
        };
        let newpivot = set.iter().take(rng.end).skip(rng.start).sum::<f64>() / rng.len() as f64;
        if newpivot == pivot {
            return pivot;
        }
        // in equals region
        else {
            pivot = newpivot;
        };
    }
}


/// State of the art Median of Medians algorithm for unquantifiable T.
/// Requires only PartialOrd and does not copy or modify data set.
pub fn odd_median<T: PartialOrd>(set: &[T], idx: &mut [usize], k: usize) -> usize {
}
/// State of the art Median of Medians algorithm for unquantifiable T.
/// Requires only PartialOrd and does not copy or modify data set.
pub fn even_median<T: PartialOrd>(set: &[T], idx: &mut [usize], k: usize) -> (usize,usize) {
}

pub fn medofmeds<T: PartialOrd>(set: &[T], idx: &mut [usize]) -> usize {
    let n = idx.len();
    // recursion termination: if the index has only one or two elements, return the first
    if n < 3 {
        return idx[0];
    };
    // partition the list into groups of 3 elements, ignore remainder items 0-2
    let groups = n/3;
    let remainder = n%3;
    let mut m: Vec<usize> = Vec::with_capacity(groups);
    for g in 0..groups { 
        // subarr.sort_unstable_by(|&a, &b| set[a].partial_cmp(&set[b]).unwrap());
        m.push(median3(set,&idx[g * 3..(g + 1) * 3])); // median of three
    }
    if remainder == 2 { m.push(idx[n-2]); m.push(idx[n-1]); } 
    else if remainder == 1 { m.push(idx[n-1]); }; // push remainders unprocessed 
    // find and return the median of medians recursively
    medofmeds(set, &mut m)
}

/// Returns vector of index items such that their counterparts in s are less then pivot 
/// and vector of index items s.t. their conterparts in s are greater than pivot, 
/// in their original order. Indices of items equal to pivot are ignored.
pub fn partition<T: PartialOrd>(s: &[T], idx: &[usize], pivot: &T) -> ( Vec<usize>, Vec<usize> ) {
    let mut ltset = Vec::new();
    let mut gtset = Vec::new();
    for &ix in idx {
        if s[ix] < *pivot { ltset.push(ix); continue; };
        if s[ix] > *pivot { gtset.push(ix); };
    };
    ( ltset,gtset )
}

/// Finds the median of three items in three comparisons
fn median3<T: PartialOrd>(s: &[T], idx:&[usize]) -> usize {
    if s[idx[0]] > s[idx[1]] { idx.swap(0,1); };
    if s[idx[1]] > s[idx[2]] { idx.swap(1,2); };
    // now the max item is (bubble sorted) to position 2
    if s[idx[1]] > s[idx[0]] { return idx[1]; }
    else { return idx[0]; } 
}  
 
*/
/*
const NONE:usize = usize::MAX;

/// Node of a binary tree
struct Node {
    /// State of this Node (see enum State)
    state: State,
    /// Best value already found for one of the branches but not yet used
    mem: usize,
    /// Address of the left branch = subscript to Vec of Nodes.
    left: usize,
    /// Address of the right branch.
    right: usize
}

/// Memoised state of a Node.
enum State {
    /// This Node and all its progeny are exhausted, simply ignore it.
    Done,
    /// Already sorted leaf node
    Leaf,
    /// Compare an actual leaf value, not a pointer to other nodes
    GetLeftLeaf,
    /// Compare an actual leaf value, not a pointer to other nodes
    GetRightLeaf,
    /// Find the next value of only the left branch; right value is mem.
    GetLeftNode,
    /// Find the next value of only the right branch; left value is mem.
    GetRightNode
}

/// Recursively builds an 'arena' binary tree, whose leaves are subscipts to s.
/// Other Nodes are as defined above. The resulting mutable tree is `&mut Vec<Node>`.
/// It is mutable so as to be able to return lazily just one minimum at a time.
/// Returns the first minimum.
fn maketree<T:PartialOrd>(tree: &mut Vec<Node>, s: &[T], rng: Range<usize>) -> usize {
    let n = rng.len();
    match n {
    0 => panic!("{}: maketree given zero lentgh input",here!()),
    1 => return rng.start,  // only one value so just return it, do not even create a Node
    2 => {
        if s[rng.start] > s[rng.start+1] { // comparing data values subscripted by these index values
            tree.push(
                Node{ state:State::Leaf, mem:rng.start, left:NONE, right:NONE });
            return rng.start+1;
        };
        tree.push( Node{ state:State::Leaf, mem:rng.start+1, left:NONE, right:NONE });
        return rng.start;
    },
    3 => {
        let mut locvec = Vec::from_iter(rng);
        s.isortthree(&mut locvec,0,1,2);
        tree.push( Node{ state:State::Leaf, mem:locvec[1], left:locvec[2], right:NONE});
        return locvec[0];
    },
    4 => {
        let mut locvec = Vec::from_iter(rng);
        s.isorttwo(&mut locvec,0,1);
        s.isorttwo(&mut locvec,2,3);
        if s[locvec[0]] < s[locvec[2]] {
            tree.push( Node{ state:State::GetLeftLeaf,mem:locvec[2], left:locvec[1], right:locvec[3]} );
        return locvec[0];
    },
    _ => {
    // Depth-first recursive descent
    let leftval = maketree( tree, s, rng.start..n/2 ); // first half tree from the first half range
    let leftaddr = tree.len()-1; // last node created is the left root branch (in depth first order)
    let rightval = maketree( tree, s, n/2..rng.end ); // second half tree from the second half range
    if leftval > rightval {
        tree.push( Node{ state:State::GetRightNode, mem:leftval, left:leftaddr, right:tree.len()-1 } );
        return rightval;
    };
    tree.push( Node{ state:State::GetLeftNode, mem:rightval, left:leftaddr, right:tree.len()-1 } );
    return leftval }};
}

/// walks 'arena' tree, sending min values up and returning the global minimum or DONE
fn walktree<T:PartialOrd>(tree: &mut[Node], root: usize, s: &[T]) -> usize {
    let node = &mut tree[root];
    match node.state {
        Done  => NONE,
        Last => {
            /// return the stored last value and mark the node as Done
            node.state = State::Done;
            node.memo
        }
        DoLeft => {
            if node.left == DONE { node.best = DONE; return node.right; }; // this leaf node is now exhausted
            let i = node.left; node.left = DONE; return i;
        },
        DoRight => {
            if node.left == DONE { node.best = DONE; return node.right; }; // this leaf node is now exhausted
            let i = node.left; node.left = DONE; return i;
        },
        TODO => {
            let leftval = walktree(tree, node.left, s);
            let rightval = walktree(tree, node.right, s);
            if leftval == DONE {
                if rightval == DONE { node.best = DONE; return DONE; } // memoize that the this node is now exhausted
                else { return rightval; };
            } else {
                if rightval == DONE { return leftval; }
                else {
                    if s[leftval] > s[rightval] { node.best = }
                }
         },
        _ => { let i = node.best; node.best = TODO; return i; }
        }

    }
*/



fn select(arr: &mut [i32], k: usize) -> i32 {
    let len = arr.len();
    if len <= 10 {
        arr.sort();
        return arr[k];
    }

    let mut medians: Vec<i32> = vec![];
    for i in (0..len).step_by(5) {
        let end = i + 5;
        let group = &mut arr[i..end];
        group.sort();
        medians.push(group[2]);
    }

    let pivot = select(&mut medians, medians.len() / 2);

    let mid = partition(arr, pivot);
    if k == mid {
        return pivot;
    } else if k < mid {
        return select(&mut arr[..mid], k);
    } else {
        return select(&mut arr[mid + 1..], k - mid - 1);
    }
}

fn partition(arr: &mut [i32], pivot: i32) -> usize {
    let (mut i, mut j) = (0, arr.len() - 1);
    loop {
        while arr[i] < pivot {
            i += 1;
        }
        while arr[j] > pivot {
            j -= 1;
        }
        if i >= j {
            return j;
        }
        arr.swap(i, j);
    }
}


struct BinHeap {
    
}

fn median_idx(heap: &BinaryHeap<usize>mut [usize]) -> usize {
    let len = heap.len();
    let mut count = 0;
    let mut median = 0;
    for x in heap {
        count += 1;
        if count == len / 2 + 1 {
            median = *x;
            break;
        }
    }
    median
}

/// Create an index heap from an unsorted vector, 
/// testing for wrong order with closure `is_wrong`
fn heapify(len: usize, is_wrong: impl Fn(usize, usize) -> bool) -> Vec<usize> {
    if len == 0 { return vec![]; };
    if len == 1 { return vec![0]; };
    let mut idx = Vec::from_iter(0..len); 
    let mut n = len - 1;
    while n > 0 {
        let p = (n - 1) / 2;
        if is_wrong(idx[p],idx[n]) {
            idx.swap(p, n);
        } else {
            break;
        }
        n = p;
    };
    idx
}

fn get(index: &mut Vec<usize>, is_wrong: impl Fn(usize, usize) -> bool) -> Option<usize> {
    if index.is_empty() {
        return None;
    }
    let root = index[0];
    let last = index.pop().unwrap();
    if !index.is_empty() {
        index[0] = last;
        let mut i = 0;
        while 2 * i + 1 < index.len() {
            let left_child = 2 * i + 1;
            let right_child = 2 * i + 2;
            let mut largest = i;
            if is_wrong(index[largest], index[left_child]) {
                largest = left_child;
            }
            if right_child < index.len() && is_wrong(index[largest], index[right_child]) {
                largest = right_child;
            }
            if largest == i {
                break;
            }
            index.swap(i, largest);
            i = largest;
        }
    }
    Some(root)
}

/// Inserts item into heap_max index
fn insert(item: usize, index: &mut Vec<usize>, is_wrong: impl Fn(usize, usize) -> bool) {
    index.push(item);
    let mut i = index.len() - 1;
    while i > 0 {
        let parent = (i - 1) / 2;
        if is_wrong(index[parent], index[i]) {
            index.swap(i, parent);
            i = parent;
        } else {
            break;
        }
    }
}

/// Heap sort algorithm that uses a binary heap to sort k best items.
/// Example:
/// let mut arr = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
/// heap_sort(&mut arr, k, &|a, b| a.cmp(b));
/// println!("{:?}", arr);
/// This will print the sorted array in ascending order: [-31, 0, 1, 2, 2]
/// To sort the array in descending order, you can use the following comparison function:
/// heap_sort(&mut arr, &|a, b| b.cmp(a));
pub fn heap_ksort<T: PartialOrd>(
    set: &[T],
    idx: &mut Vec<usize>,
    k: usize,
    ascending: bool,
) -> Vec<usize> {
    let n = idx.len();
    let mut res = Vec::new();
    if n == 0 {
        return res;
    };
    // closure detecting wrong order of subscripted set items
    // the tests are negated because we are constructing heap_max
    let is_wrong = |a: usize, b: usize| {
        if ascending {
            set[a] < set[b]
        } else {
            set[a] > set[b]
        }
    };
    for _i in 0..k {
        if let Some(r) = get(idx, is_wrong) {
            res.push(r);
        } else {
            break;
        };
    }
    res
}
