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

Well, holy crap that was quick and easy! That's what happens when I keep my f32 implemention of RaytracerMatrix up to date with the i32 one and refactor for default implementations proactively! Shout out to Rust's type system for making this foolproof, too!

Phew! Matrix progress. But still so far to go until _Ray-Sphere Intersection_. Still so far ... Wouldn't mind seeing more visual results in the interim in order to keep me going. Anyway, back to the code mines!

Not happy with the implementation of `transposed`. Too effort-heavy. Demonstrates a gap in my knowledge of how I can best take advantage of the typing system in Rust. Something to refactor before starting the next chapter. Also betrays a gap in my knowledge of what exactly happens when we transpose a matrix.

Transformations time! We're going to find out whether or not it was wise to implement `Pointer` and `Vector` as their own types instead of as identical tuples.

Not happy with the type specificity of `translation_f32` but then it's not like I'll actually need it to be generalizable for my purposes. Priorities! In fact, I'll go ahead and rename it to `translation`. The good news? It doesn't seem insurmountable! (And a small mental break helped here. Just had to flip some of my thinking around.) Feels like I'm doing some unnecessary conversion just to make use of the type system, though. Something to refactor out _later_, only after I get things working!

# 04Feb2020

On to scaling! But first, some refactoring to make the API more consistent. It's odd to have `translation` as a stand-alone function.

Might end up deleting all of the `i32` implementations for `RaytracerMatrix`, too, if I end up with no code that uses it.

Silly me, I don't need `RaytracerMatrix::translation`. I just need a `Translation` type! Actually, back to the original idea! I don't need `Translation::new`; that's habituation from other languages (particularly Ruby) speaking. Best to have a `translated` method on `RaytracerMatrix`. Waaaait, why am I worried about this at all? I need `translation` for the tests (as written) anyway. Just leave it alone for now. Packages. Specifically, a `transformations` package. _That_ is what I wanted.

Well! Scaling sure was easy after all that.

Not super happy about implementing the `radians` parameter on `rotation_x` as an `f32` instead of using a more abstract representation, but I'll only revisit that if it proves onerous.

Mental note to only use `round` and `rounded` at the last possible moment (e.g. when calling `assert_eq!` and no sooner).

I do not feel like I understand the rotation transformations. A topic to dwell on later.

Yay, shearing is easy!

Back from lunch. Rather perplexed that the last step in the `Individual transformations are applied in sequence` scenario is failing for me. Oops! Just a little array index mistake that I duplicated a few times. Fixed!

Observation: I'm using `clone` too much in the specs. It shouldn't be necessary for the `Mul` overload of matrices. Will take a stab at fixing this before proceeding ... Or I could use `dot` as intended. (The former point still holds, just not for the present scenario.) Good news! That refactoring was much easier than expected. Multiplying against references feels a bit odd but that will spare us a lot of unnecessary cloning.

And now, clock time! (After a break.) Aside: I am so very glad for the "example" functionality built into `cargo`. It makes it so much easier to demonstrate how to use the crate.

"Good" old overflow error! At least I eventually figured it out. And I didn't have to look at the hints in the book at all! Let's clean up some of these magic numbers and questionable type conversions.

And _finally_, time to work on ray-sphere intersections! (But only after a walk break.)

Sphere time! Oh, the meme potential.

# 05Feb2020

Back to work! Okay, this `intersection` object will prove interesting. Since it stores a reference to the intersected object, I predict that I'll have to think more about Rust lifetimes while implementing this than I have for all the preceding code. Huh, this _first_ stab at it hasn't been too bad but the proof is in the eating of the pudding and I still haven't really taken a bite.

And yep, the trouble starts as soon as I try using `Intersection` (with its lifetimed reference property) on the `MyWorld` struct of the Cucumber code. Still haven't fixed my lifetime issues but I'm incredibly grateful for the diagnostic usefulness of the Rust compiler's error messages. The devs have done such good work here. And _huh_! Looks like good old `Box` did the trick. I hope I'm using it right.

Now comparing objects will be the sticky widget. Rust doesn't have automatic object IDs like an object-oriented language would so I'll have to do something different. We can't just compare values, after all, since two spheres with the exact same coordinates are still distinct spheres. Unique integer IDs are an adequate solution but  _something_ has to uniquely increment those IDs. It's time for a `World` of some kind.

That wasn't too bad after all! Comparing IDs directly in the test feels a bit like cheating but it avoided confusing variable type issues.

Back from a coffee break and running into borrow complications with the `Aggregating intersections`. I want to think it's an unnecessary test since I already have `Vec` but it does raise important questions (even if I can't enumerate them explicitly, only as a gut feeling). Maybe I need to use `Rc` instead of `Box`. And success! `Rc` _was_ the answer! Feeling pretty smart for having figured that out on my own. My mastery of the Rust memory model grows.

Torn about whether to implement `hit` as a standalone function or as a method on a type. The latter is my default but it's also a habit from OOP. The former _would_ break the pattern established by my API, though, so best to aim for consistency.

Observation: We have too many explicit numerical types floating around. We could use more type aliases, e.g. `Time = f32`. I'm making better use of `Option`, too.

Oops: I think I've used `Intersection` and `Intersections` when I mention to use `Interception` and `Interceptions`. Something to refactor quickly. Actually, nevermind! Looks like I used both those words correctly.

I do feel bad copying and pasting so much for these specs but, on the other hand, that has lead to only nominal subsequent inconveniences. Sometimes bad practice for production code, sure, but test code should change even less frequently than production code.

I misspoke earlier about my API patterns. I was looking at `lib.rs`, which focuses on methods, but `transforms` _does_ use a few stand-alone functions. One more reason to do some refactoring sooner rather than later, perhaps?

Buh-wah. I have some scenarios that have started failing randomly. Utter confounded since I haven't touched them in ages. Leaves me wondering where the seeming nondeterministic element could have possibly come from.

Phew. Still plugging away at this transformation work but I can see a fair amount of refactoring ahead of me., I'm clearly too brain-drained here at 4:24pm to tackle that, though, so time to call it a day!


# 06Feb2020

New day, new challenges! (Well, the same challenges from yesterday!)

I considered having a general `Transformation::Rotation` type that took a new `Axis` type but since we'll only ever have the three axes that might be overkill'. Besides, that would complicate matching down the road. A potential future refactoring.

I will need to refactor so that functions return either `Array` or `Rc<Array>` more consistently. (At least, I think I will.)

Wondering if I'm making unnecessary trouble for myself by having `Transformation` as an enum instead of making each variant its own type. There's probably a more ergonomic way to handle it, though, instead of manually raising errors on mismatches like I am.

It feels like the end of the refactoring is in sight! Goodness knows I'll write more ownership-aware code going forward!

Yeah, I think my instincts were right. `Transformation` needs to be a `struct` with a `matrix` property and a `type` property since every transformation (so far) will have a `matrix` but I don't always care about the `type`.

Refactoring wasn't so bad, it seems! Type system sure helped there. Still need to fix a couple specs, though.

Surprised I'm having so much trouble with `scaling`. I wonder what's up with that. Hopefully just getting the math wrong. Well, huh. I recreated the test in a new `transformations` example and it seemed to work just fine! So something is wrong with my test setup. Not surprising since I had already run into intermittently failing tests. Guess it's time to fix those without delay!

I haven't fixed the intermittent errors but I solved the scaling issue: It was the `when "r2 ← transform(r, m)"` step this entire time! I knew that matrix reconstitution was a bad idea. I bet I haven't even needed to distinguish these transformations by type this entire time, either ... That fixed it! Oh well, lesson learned: building out separate `Point`/`Vector` types appears to have been overkill. Something to refactor away later.

I've just seen the specs return different results across multiple runs with no recompilation inbetween. Prediction: The intermittent errors stemmed from `MyWorld` not being reset properly between scenarios and the errors come from the scenarios being run in a randomized order. I'm going to try fixing that now. The comments _say_ the world is being recreated for each scenario, though, so what the heck. Okay! So sometimes `world.transform` doesn't get set or gets reset?? Confounding. So it seems to be a matter of `world.transform` not being set properly in some cases (because I can change the default and the error results reflect that). Maybe I'm using `Rc` wrong? Well _huh_. Sometimes the `Given` step isn't getting called! What the heck. Maybe some funky cache thing in the crate? I'm filing an issue with the crate's GitHub repo because I can't for the life of me figure out how this could be failing non-deterministically ([the issue](https://github.com/bbqsrc/cucumber-rust/issues/44).

Going to refactor away that redundant `Transformation` type before proceeding.

Uncertain about how to proceed with attaching transformations to `Sphere`. I think I'm misusing `Rc` or something. Probably need to use more references. Huh. Should I have been using `Rc::clone` this whole time? I'm probably going overboard with methods vs. functions. Feels odd to be cloning `world.s` (a `Sphere`) so much but I remind myself that this is probably an artifact of the test setup. Production code would hand out more `Rc` instances that would get cloned instead. Or I could refactor `world.s` to _be_ an `Rc<Sphere>` and do this correctly. Well, that refactoring turned into a headbanger that I still haven't resolved. Since I can live without that change right now, I'll shelve that and revisit it only if necessary.

# 07Feb2020

It was the regular expressions! I forgot my `^` and `$` anchors!

And now, time to render (the silhouette of) a sphere! And success! (So slow, though! Still a success!)

Trying to use [`cargo-instruments`](https://github.com/cmyr/cargo-instruments/) to determine why the `sphere` example is running so slowly but I get a `failed to parse lock file` error when I run `cargo instruments`. Tried running `cargo update` in case that might help but no luck.

Oops! Forgot to journal. So profiling was a bust _but_ my program is suddenly inexplicably much faster than I expected? Wondering if something in the background was eating up a lot of processing power.

Now onto _amazing_ news: We have perspective projection! Whoooo!

And some primitive shading!!

And now, time to calculate some normals! Smooth sailing so far.

Woohoo! The issue I filed for `cargo-instruments` has been addressed! I can profile my code again! That makes _two_ crates improved or fixed because of my work here!

# 10Feb2020

Another day, another feature! I've had my coffee so let's get started.

Ooph, yeah. My use of different types for points vs. vectors vs. other matrices is coming back to bite me. Might have to refactor that whole distinction away. Actually, wasn't _that_ bad once I remembered to use operators properly but this still involves more context-switching and variable type conversion than necessary. (Core problem here turned out to be an error in the Cucumber step definition anyway.)

But first, time to improve my tooling! That is to day, exclude the `target` directory from what the `CtrlP` in `vim` searches. _Wow_ that's a night and day difference on performance! Totally worth the minute or less it took to do that.

And finally, lighting and material! Excited!

Oops, I've exceeded the recursion limit on my Cucumber tests! It's probably time to start separating those out into multiple files but I'll just increase the recursion limit for now.

Oh no, something about my `Material#lightning` function is off math-wise. I predict that I'm using `*` where I mean to use `#dot`. At least all the types are playing nicely together on my first take! Surprise! It was actually an issue with my spec code grabbing the wrong values! (Array index issues, specifically.) Phew!

Taking a quick stab at refactoring `sphere.rs` and the code it uses in order to make the example more efficient. Also! Finally remembered that I can use the `--release` flag to dramatically increase its speed when I don't need debugging support. That'll save me a lot of time. Seems to come down to a lot of `ArrayBase` function calls. `ArrayBase` is overbuilt for my 1-4D matrix work so writing my own matrix code might actually be worth it? My primary suspect is my implementation of `submatrix`, which seems to be making a lot of different calls to functions for `ArrayBase`. Let's see if a more manual approach speeds things up. Success! Refactored `submatrix` to be a simple values copy operation into a new matrix and the speed improvements were dramatic! Go me! Don't know how I can avoid that super-slow allocation of a new matrix, though, (not without bringing static memory into the mix) so I think I've done what I can for now.

Back to our original task!

_Holy cow!_ Look at that sphere! Amaaaaazing!

That's a great stopping point for now. Let's see about cleaning up the code, especially in regard to `rustfmt`. `clippy` has some complaints, too!

# 11Feb2020

It's a new day! Time for new code!

Had a little fun playing around with transforms on the sphere but now it's time for the next step!

But even before that, refactoring time! Well, mostly moving things into their own modules so that the files are not so large. There! Extracted a few modules. I expect that to make things less confusing.

Oh no! `The trait "RaytracerObject" cannot be made into an object`. Here's hoping that I can fix this with enough uses of `Box` and `Rc`. (I can always simplify the implemenntation later.) No luck yet. There's a very real possibility that I'm overusing trait objects annd dynamic dispatch where I should be using enums. Yeah, let's try refactoring `RaytracerObject` into an enum and go from there. While that _would_ make it harder for others to add their own objects, that's not _actually_ a concern for this project? Priorities! Heck yeah that refactoring was easy! Much easier than I expected. No complaints here.

Interesting to see how the performance profile changed with the refactoring. It look likes matrix math has come to dominate the processing time, particularly calls to `ndarray` functions. I may well benefit from implementing my own matrix types. In the meantime, I'll eliminate some unnecessary dynamic dispatch, or at least some unnecessary code.

Hmm. `#submatrix` is expensive because it allocates a new, default `Array`. Since it's only called (so far) by `#determinant`, I wonder if I can get away with having `#submatrix` work with simpler memory models, like returning a simple `Vec` that only gets converted into an `Array` as needed. Something to think about when the rendering times get onerous again. Maybe I can use [Saruss's Rule](https://github.com/apanasara/Saruss_3x3_Determinant)?

Anyway, that's enough for today. I have work-related tasks to tackle!

Got a lot done! Going to treat myself to a bit more Rust work before I leave for the day.

# 12Feb2020

New day, new Cucumber scenarios! Let's get to work.

Oops! Good thing I was paying enough attention. Looks like I won't actually need to support multiple lights, at least not within the scope of this book. Goodbye, `Vec`! Hello, smaller scope! And all that was pretty straightforward! Plenty of refactoring opportunities, though. Wonder if I could eliminate `Rc` entirely with some effort.

And now, time for some _prepared computations_! Wish me luck!

I'm not sure if `PrecomputedHit` will be the right name for the struct but it seems correct after an initial review. We'll see how that bears out in practice.

Phew! Brain feels drained after that "precomputation" work. It's probably less a matter of the programming work and more a matter of having not had my afternoon coffee yet while deferred job tasks pull at my attention.

Everything was going swimmingly until the `Shading an intersection` scenario. At quite a loss for what could have caused the color values to converge. Something to sleep on and leave for tomorrow. More than enough other work tasks to focus on!

# 14Feb2020

Took a day off to focus on other work and only spending half an hour to debug today but let's see if I have any luck.

Wow, I'm totally stuck on this "Shading an intersection from the inside." How confounding. Might have to review the fundamental matrix math before I can diagnose this.

Oh my good grief. It was a faulty regex again! I _finally_ noticed that the world light looked odd. And fixed! Bleeping finally.

Still at the office, later than expected, so onto the next feature!

Hrm. Something seems to be wrong with my inside/outside calculations in regards to precomputed calculations. That doesn't seem to explain everything, though, since I still get incorrect color values when I try to correct for that. The `inside` part may be a symptom. As I thought! I wasn't selecting the earliest hit from among the intersections but rather the last hit. Yep! My first-hit detection math _was_ off! But my fix also broke something else ... And fixed! Just had to explicitly account for negative time values when calculating the first time. (No time travel allowed!) Progress!

# 17Feb2020

Back to work! A short dive into the code now since there's still research to do. Wanted a mental break to refresh myself.

Still feels like I'm overusing `Rc` and one scenario, `The color with an intersection behind the ray`, reinforced that notion. I probably just need to make better use of borrowing mutable references instead.

`view_transform` wasn't too bad, at least once I remembered to continue reading instead of trying to guess the solution on my own prematurely!

And next time: cameras!

# 18Feb2020

Time for a Rust/ray tracer break! Let's see how far I can get.

Yay! Initial `Camera` scenario was simple enough.

Huh. That seems like too few `pixel_size` scenarios. Well, done anyway. That wasn't too bad. Break time!

Almost camera time!! Normally I'd stop here to focus on more work stuff but (1) I'm staying a bit late tonight and (2) I'm really eager to see visible results of my work again!

Silly me! I already had a `Canvas` struct and didn't need `Image`! Oops. Amazing how I can forget about my own structs in my own small project. Bigger projects don't stand a chance.

HECK YEAH! It renders!! Go me!

# 19Feb2020

Alright! Let's set the sphere scene to match the book.

And success! It looks amazing.

But it's time to take a break. I have other priorities at work so I'll shelve this for now and save it for occasional breaks.

# 21Feb2020

I have a bit of downtime while waiting for a meeting (distracted by vague anxiety) so I'm going to take a quick stab at optimising the code.

So `RaytracerWorld#intersect` gets called a _lot_. Let's dig deeper. Something about an `Iterator`. So `#determinant` also gets implicated. Curious. I don't know if there's much we can do to optimize things at that level, though, aside from reimplementing matrices from scratch to avoid the overhead of the `Array` from `ndarray`, but let's keep looking. That does look like our best bet, though, and would certainly make the code less confusing in the end. We can get away with that now that we know we'll only work with matrices up to 4x4 and with only `f32` values.

But first, benchmarks! We can't optimize properly if we don't know where we're starting from:

Results from `bench "cargo run --example sphere --release"`:

With width(100) and height(50):

```
benchmarking cargo run --example sphere --release
time                 1.428 s    (1.352 s .. 1.529 s)
                     0.999 R²   (0.998 R² .. 1.000 R²)
mean                 1.397 s    (1.374 s .. 1.414 s)
std dev              23.62 ms   (11.09 ms .. 30.81 ms)
variance introduced by outliers: 19% (moderately inflated)
```

With width(200) and height(100) (note that this takes significantly longer to run than 100x50):

```
benchmarking cargo run --example sphere --release
time                 5.410 s    (5.201 s .. 5.788 s)
                     0.999 R²   (0.999 R² .. 1.000 R²)
mean                 5.368 s    (5.315 s .. 5.406 s)
std dev              52.88 ms   (21.11 ms .. 70.15 ms)
variance introduced by outliers: 19% (moderately inflated)
```

Oh! Time to parameterize `sphere.rs`. There's no reason to keep editing the image's dimensions in the code itself!

# 22Feb2020

Gasp! It's a Saturday and I'm still working on this! It's procrastination, I know, but fun. I just added command line argument support to `sphere.rs`!

Hrm. Took a stab at refactoring towards arrays but it doesn't feel right. I think I'll reprioritize around removing unnecessary uses of `Rc` first. Probably fast to just copy or something.

# 24Feb2020

Subsuming some anxiety into another attempt at optimization. Let's see if we can allocate fewer `Array` structs.

Okay, used some [manual math](https://www.mathsisfun.com/algebra/matrix-determinant.html) in order to avoid allocating an `Array`. I still allocate a lowercase A array of fixed size in order to handle the submatrix math. Something like that is probably unavoidable without dark magic code. Let's profile this code!

`bench "cargo run --example sphere --release -- 100 50"`:

```
benchmarking cargo run --example sphere --release -- 100 50
time                 241.8 ms   (233.7 ms .. 246.8 ms)
                     1.000 R²   (0.999 R² .. 1.000 R²)
mean                 254.0 ms   (248.3 ms .. 265.5 ms)
std dev              12.14 ms   (349.1 μs .. 15.25 ms)
variance introduced by outliers: 16% (moderately inflated)
```

Excuse my language but _daaaaaaaaaaaaaaaaaaa-yum_.

_16.9%_ of the original runtime. Almost _6x_ faster. Dang!

Let's try it with a larger image.

`bench "cargo run --example sphere --release -- 200 100"`:

```
time                 765.9 ms   (756.8 ms .. 771.7 ms)
                     1.000 R²   (1.000 R² .. 1.000 R²)
mean                 778.7 ms   (773.0 ms .. 788.5 ms)
std dev              9.874 ms   (655.3 μs .. 12.19 ms)
variance introduced by outliers: 19% (moderately inflated)
```

_14.2%_ Dang again! Not bad for maybe an hour's work!

# 25Feb2020

Retreating into ray tracer work for a bit, even though I'm working from home, because (positive but nonetheless rather anxiety-inducing) activities are killing my ability to focus on work tasks. Beats retreating into _Overwatch_, which is also a serious temptation.

Still torn between adding shadows vs. optimizing away `ndarray`. Since I'm at home, I think I'll try for the latter for at least a bit.

First things first: Pay for my tech debt "sins" by making some previously public fields private. Had I done that out of the gate then changing the underlying implementation would have proven much less painful. Well, it's time now!

Realization that I don't need to start with refactoring `Point` or `Vector`! Converting to/from them is relatively cheap compared to dealing with `ndarray::Array`! Time to refocus on `RaytracerMatrix`! Actually, `TransformationMatrix` looks like an even better target since it's used much less.

Struggled to make this compile for a while before I remembered `unimplemented!`. I don't need to make all the tests _pass_ on the first go, just _compile_.

Getting close! Amazed at how many of my tests _actually pass_! Just two sticky ones that probably indicate some faulty matrix math on my part.

Success! Finally! Took forever (and a break) for me to realize that I was generating too many values. Oops!
