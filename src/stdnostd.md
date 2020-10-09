# [std] and [no_std]?

The `#![no_std]` attribute in the first line of the program indicates that the program will not make use of the standard library, the `std` crate. Instead, it will use the `core` library, a subset of the standard library that does not depend on an underlying operating system (OS). It is completely platform agnostic, does not require upstream libraries, system libraries, or libc. It is necessary in environments, where this is the first code that is loaded. As a consequence, the core library does not provide all functionalities available within the `std` library. 

## Collections

The core library does not provide Vec, String, and HashMap, as they need a dynamic memory allocator (heap allocation), which `core` does not provide. 

Without using other crates, you are restricted to types with a size known at compile time such as [arrays](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type) and [tuples](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type). 

Another type that works with a bit more flexibility in length are [slices](https://doc.rust-lang.org/book/ch04-03-slices.html#the-slice-type). A slice is a reference into a list of elements stored in contiguous memory. One way to create a slice is to take a reference to an *array*, a fixed-size list of elements stored in contiguous memory.

``` rust
// stack allocated array
let array: [u8; 3] = [0, 1, 2];

let ref_to_array: &[u8; 3] = &array;
let slice: &[u8] = &array;
```

`slice` and `ref_to_array` are constructed in the same way but have different types. `ref_to_array` is represented in memory as a single pointer (1 word / 4 bytes on a 32-bit platform); `slice` is represented as a pointer + length (2 words / 8 bytes on a 32 bit platform).

Because slices track length at runtime rather than at compile time, they can refer to chunks of memory of any length.

``` rust
let array1: [u8; 3] = [0, 1, 2];
let array2: [u8; 4] = [0, 1, 2, 3];

let mut slice: &[u8] = &array1;
log::info!("{:?}", slice); // length = 3

// now point to the other array
slice = &array2;
log::info!("{:?}", slice); // length = 4
```

Another possibility for dealing with this problem:

The [heapless crate](https://docs.rs/heapless/0.5.6/heapless/), which provides `static` friendly data structures that don't require dynamic memory allocation, with a fixed maximum storage size.


### Further Reading:

[The embedded Rust Book](https://docs.rust-embedded.org/book/collections/)
