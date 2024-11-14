
## Overview

An implementation of the [rope](<https://en.wikipedia.org/wiki/Rope_(data_structure)>) data structure, built on top of a splay tree structure.

A splay tree is a self-balancing binary search tree. Leaf nodes of this BST store an immutable string segment, between 64 and 127 bytes in size, along with the length of that segment. The splay tree then keeps the most frequently accessed Nodes / indices at the top, making them easily & efficiently accessible on future operations for that Node. This is done through tree rotations performed during insertion and access operations.

#### Example of Rope structure

<p align="center">
    <img src="https://upload.wikimedia.org/wikipedia/commons/thumb/8/8a/Vector_Rope_example.svg/640px-Vector_Rope_example.svg.png" alt="rope-structure-diagram" width="800">
</p>

Ropes are a more efficient data structure for large strings (or more generally indexed sequences of bytes). In fact, they ensure that an arbitrary length string performs similarly to shorter counterparts. This is a very desirable property in programs like text editors, which store large buffers of text which the user frequently operates on.

Unlike ordinary strings, ropes let you do some operations up to exponentially faster than their counterparts:

- **Add bytes** to the beginning, middle, or end — any index you want.
- **Delete any rope substring** or move it to a different position within the rope.
- **Splice / concatenate any substring** of a rope with any other rope.
- **Read any substring** with random access.

Instead of @memcpy'ing the bytes of a string like we might do with a []const u8 when attempting similar operations as the Rope allows, an insertion at any index is instead done by creating a new splay tree Node in the Rope structure at the desired index, and then simply rebalancing the tree as needed.

## Project Notes

<strong>I very much welcome contributions and feedback/discussion and issues, if people find this useful or maybe use it in a project of theirs, or just play with the code to learn or understand splay trees and ropes better as a concept. It's free to use for any purpose (MIT).</strong>

My only goal with this repo is both to use it in another project of mine as a text buffer data structure, and will be maintaining it for that purpose as much as my use-case requires, and I also intend to improve the API a bit for use outside of it's original purpose, i.e making it more generic, and readable/accessible, as well as adding new methods and features that broaden the potential uses of it.

I also had the idea of modularizing the underlying tree structure to implement a B+Tree backing type in addition to the existing splay tree implementation, just for my own research and fun. I will tackle that in another branch at some point, hopefully! :)

## Acknowledgements

This code was forked from the `redis-rope` repo, created by Eric Zhang ([@ekzhang1](https://twitter.com/ekzhang1))

I made the fork since the original repo was vendor-specific and had lots of unnecessary code related to the actual interop with Redis' infra. It was also written and organized in a way that was not ideal general use as an open-source library, and instead was optimized for Redis' purposes. In the past, it seems to have been used as the actual string data type in Redis at one point. Though, it appears to have fallen out of maintenance and presumably use, with the most recent commit being 2 years ago.

Licensed under the [MIT license](LICENSE).
