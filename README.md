# Rust AVL

## Overview

I'm in the process of learning Rust and I wanted to explore implementing a somewhat non-trivial tree structure. For the
specific structure, I chose to implement an AVL-like tree. The implementation is somewhat basic, but it does support 
`insert`, `contains`, and `delete` all with self-balancing.

See the tests cases for example usage and more details.

This is entirely a toy implementation and should not be used in any real setting. Various aspects are non-optimal:

- Height and balance factor are not cached in any way.
- `contains` doesn't use the Borrow trait bound approach.
- The memory layout is not cache friendly.
- Etc.

Overall, I found the experience very insightful.
