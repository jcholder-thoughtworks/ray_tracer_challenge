# 28Jan2020

Day one!

The book starts with using 4-element tuples to represent both points and vectors but I'm going to implement those as separate structs since I can trust Rust's type system to distinguish those for me.

And of course the math operations complicate this, given that the book's system allows mixed but limited operations between points and vectors. Adding argument-specific functions seems like a combinatorial nightmare but, I think, only if I end up with more tuples. The From/Into traits might help here, too.

Dang, I wasted plenty of time and effort by not first re-reading which vector/point operations are actually supported.

Dang again. If I'd realized that operator overloading would have been so easy I would have lead with that instead of bothering with intermediate functions.

Seeing all these operations that include the W component of the example tuples and hoping that my W-less Vector struct doesn't cause me problems in regards to this later.

Successful physics implementation! Go me!

Okay, Rust only has the two float types and the book says I won't need the higher precisions of f64 (just look at EPSILON) so I won't bother templating that.

# 29Jan2020

Time for colors! But first, time to clean up the code a bit first. Linter! Running `rustup update`, too, before installing `clippy`. I've used `clippy` before and look forward to revisiting it. Hah! Only a single piece of simple feedback! Go me!

I'm mixing references and non-references less consistently than I would like. Might help to go back and take out all the references until I run into actual borrow issues. I expect that to make the API more predictable.

Oops. Somehow swapped `height` for `width` on the pixel methods on `Canvas`. Had to add another `assert!` which verified that the exact pixel in the 1D array I'm using was correct. Took me a surprisingly long time to debug that, though. Probably just needed to walk away from the code for a while instead of sticking with it for so long.

Phew! Finally done with PPM export. That took a bit more trouble than I would have liked but it's done.

Break over. Time to start on the projectile trajectory rendering challenge! Projectile rendering was a cinch! And I had a little fun with the Z coordinate and color. Glad to have a coworker sitting next to me who knows the math of all this better than I do. (Haven't needed their help yet but it's still a way to connect and socialize.)

Realization: I can use Cucumber proper with Rust! Specifically: https://github.com/bbqsrc/cucumber-rust. That was pleasantly easy to set up! (Aside from a hiccup with needing to add all the example code first before anything would work.)

Customizing the Cucumber code to work with the examples from the book is going well so far!
