# 28Jan2020

Day one!

The book starts with using 4-element tuples to represent both points and vectors but I'm going to implement those as separate structs since I can trust Rust's type system to distinguish those for me.

And of course the math operations complicate this, given that the book's system allows mixed but limited operations between points and vectors. Adding argument-specific functions seems like a combinatorial nightmare but, I think, only if I end up with more tuples. The From/Into traits might help here, too.

Dang, I wasted plenty of time and effort by not first re-reading which vector/point operations are actually supported.

Dang again. If I'd realized that operator overloading would have been so easy I would have lead with that instead of bothering with intermediate functions.
