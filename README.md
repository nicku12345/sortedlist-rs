# SortedList

A fast sorted list data structure in rust.

This repository is under active development with numerous available features. See the [Usage](#usage) and the [Documentation](#documentation).

## Usage

```rust
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
println!("{}", x);
// 25
// removed the 3-rd smallest (0-indexed) element.

println!("{}", sorted_list.kth_smallest(2));
// 20

println!("{}", sorted_list[2]);
// 20

println!("{:?}", sorted_list);
// [1, 19, 20, 90, 100]
```

## Documentation

https://docs.rs/sortedlist-rs/latest/sortedlist_rs/