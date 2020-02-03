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

`ndarray` for the win! When I saw how much manual effort would be required to handle matrix math, I ditched the unnecessary agony and went with a crate. Life's too short to burn on reimplementing that! And besides, there's raycasting to learn instead.

# 30Jan2020

Back to work! It still feels like "cheating" to not implement matrices on my own, which makes it tempting to revisit that later. But actually getting to a render matters more! Maintain that momentum.

There's plenty of duplication in some of the matrix-assigning Cucumber step implementations but it's so incredibly unlikely that we'll be changing this code that I won't prioritize refactoring that. Similarly, I'll refrain from reimplementing my earlier unit tests as Cucumber tests because those tests already provide their value as-is. If I were publishing this in a book or otherwise polishing the code then yes, I would refactor that. But I'm not.

Okay, I _would_ benefit from going through more of this matrix implementation work now that we're getting deeper into the math of it. (It literally makes my head hurt a little.) I'll still go through the features to make sure I at least understand how to use `ndarray` for those tasks. (And I _do_ have to make them work with my `Point` and `Vector` implementations.)

And it's a good thing I _did_ proceed to implement more tests! I made the false assumption that the `*` operator would perform matrix multiplication but it only performs element multiplication in the `ndarray` crate. That would have lead to some confusing results later!

The "A matrix multiplied by a tuple" scenario was particularly interesting. Going to take a quick peek into any helper methods for tuple to/from matrix conversion. After all, that scenario is truly testing my ability to multiply my custom types against matrices.

(Skipping the `Multiplying the identity matrix by a tuple` scenario for now since it doesn't test anything particularly interesting from the looks of it, at least in regards to my understanding of `ndarray`.)

Never mind on duplicating the code for matrix generation from scenario table data. I've ended up duplicating it far too often at this point. Function extraction time!

... Another 300 units (not pages) of book to go on just inverting matrices. Egads. (That's similar in size to the whole section on tuples, points, and vectors). Well, time to dive in!

Determinants might prove _interesting_. Wanted to try the ` ndarray-linalg` crate in combination with `openblas` but getting compiler errors. MacOS is explicitly not supported and I want to avoid the non-free Intel MKL library. I then looked at the `linxal` crate but it's explicitly non-optimized for low-dimensional arithmetic. Given that these libraries might make this code of mine impractical for WASM port, I think I'll actually attempt a manual implementation.

Took a few tries but I finally figured out how to make implementing a determinant function work with associated types! I'm leveling up on the Rust type system! I'd like to generalize the implementation to any numeric type but I only really care about i32 and f32 so it won't be that much duplicate code.

Hrm. Extracting submatrices seems more troublesome than I expected, at least with `ndarray`. I've looked but haven't found a built-in way to produce the kinds of axis-excluding submatrices described in the book. Maybe I'm overthinking it?

# 31Jan2020

Rust 1.14.0 was released today! Let's upgrade! And success!

And submatrix success! I needed `Array#select`. Combining two of those did the trice (one for each axis). Behold the power of (a) taking a break and (b) reading the documentation more thoroughly! Well, near success? Let's debug this. Success! For real! Inclusive vs. exclusive range confusion.

Quick refactor to make our traits less redundant.

Coworker was excited to see math in action!

Time to write up a `minor` method! ... Well, that was easy! One line function!

Phew! Done with determinants! That wasn't too bad. I'm definitely learning about matrix math as I go along.

Matrix inversion time! Going to try out some default implementations for traits, too. Good a time as any! Darn, no luck with extracting `submatrix`. Need to figure out the right type system stuff for that. Probably just need to read the docs for `ndarray` more. Other than that, I'm done for now!

Joke's on me! Now I need to handle fractional values from matrices with previously only integer values.

# 03Feb2020

Back after the weekend! Dithered a while before starting today because I knew I was neglecting some other, non-coding matters at work that took priority. Helped to, well, actually act on those nagging feelings before starting today.

Damn. Might as well go back and refactor everything matrix-y to use f32 instead of i32.
