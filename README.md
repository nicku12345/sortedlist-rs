# SortedList

A fast sorted list data structure in rust, inspired by the python library [Sorted Containers](https://grantjenks.com/docs/sortedcontainers/)

This repository is a work in progress. See [Usage](#usage) and [Documentation](#documentation) for available features.

## Benchmark Tests and Results

- [v0.2.1](./benchmark_results/v0.2.1/result.md)

## Usage

```rust
use sortedlist_rs::SortedList;

let array = vec![90, 19, 25];
let mut sorted_list = SortedList::from(array);

println!("{:?}", sorted_list);
// [19, 25, 90]

sorted_list.insert(100);
sorted_list.insert(1);
sorted_list.insert(20);
println!("{:?}", sorted_list);
// [1, 19, 20, 25, 90, 100]

let x = sorted_list.remove(3);
assert_eq!(25, x);
// removed the 3-rd smallest (0-indexed) element.

assert_eq!(&20, sorted_list.kth_smallest(2));

assert_eq!(20, sorted_list[2]);

println!("{:?}", sorted_list);
// [1, 19, 20, 90, 100]
```

## Documentation

https://docs.rs/sortedlist-rs/latest/sortedlist_rs/