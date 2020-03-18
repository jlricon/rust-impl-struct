# Various ways of structuring code in Rust


The overall idea here is to look at a problem involving a function doing some transformations on data depending on some configuration, and where those functions can act at the beginning and at the end (In various steps).
Within a step, the ordering doesn't matter.
1. Bad.rs has a naive way of doing it, where everything is a function with a bunch of if statements.
 There is nothing wrong with inlining it all when it is appropriate, as [John Carmack](http://number-none.com/blow/john_carmack_on_inlined_code.html) suggests
 but here the problem we are facing is that something that we end up with a forest of if statements and 
 2. Traits.rs uses structs and traits to bunch operations into logical units. The idea is that for each entry in `Config` we define a handler that implementas `Operation` and then we run over a list of operations. This is nice but we have to do dynamic dispatch.
 3. Enum.rs uses instead a big enum and puts the operations in its `impl`. Now there is no dynamic dispatch, but we have decoupled `Config` from the behaviour of each option. Also, the `impl` could grow quite big.
 4. Tidy_enum.rs just bundles the `impl` into smaller subfunctions; but now we end up with an entry in the enum and another entry in a function. But then again, in the Struct case we had a struct and then an impl for the struct.
 5. Tidy_enum_float.rs removes the string and then I look at the assembly to confirm that all these layers of abstraction are zero cost; indeed they are, at least for floats :=)
 
 The _1.rs files are just the base file but extending it to see how it would be done and see how much is the compiler helpful in ensuring correctness.
 Of course in the Bad case it doesn't help but in the others it is equally nice.
 
 However, one more way is to change the way we think about the program