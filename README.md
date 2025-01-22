# Rust Vector Reference Invalidation Bug

This repository demonstrates a common error in Rust when working with vectors and references. The code attempts to keep a reference to an element in a vector while modifying the vector itself. This leads to a dangling pointer and undefined behavior.

The `bug.rs` file contains the erroneous code, while `bugSolution.rs` demonstrates a corrected approach.

**Key Concepts:**
* Vector mutability in Rust.
* Borrow checker and lifetime management.
* Avoiding dangling pointers.

This example is useful for understanding how to correctly manage references to elements within dynamically sized collections in Rust.