## Chapter 8 - Collections



* _Vector_
  * It has a macro to build it
    * `vec![1,2,3]`
  * Allow you to store a variable number of values next to each other
    * Create Vector
      * `let v:Vector<i32> = Vec::new();` 
      * ` let v = vec![1,2,3]` 
        * Inferred i32 since it has i32 values
    * Update Vector
      * `v.push(1)`
        * or the type inferred to the vector
    * Dropping a vector
      * A vector is freed when it goes out of scope
        * Just like most structs
        * Can get complicated when using references
  * [For more information on the vector](https://doc.rust-lang.org/stable/nomicon/vec.html)
* _String_
  * A collection of characters
  * Information about concat
    * let s3 = s1 + &s2
      * The way it concats
        * s1 is going to be copied to s3
          * Loose ownership
            * Unable to use it afterwards
        * then concat s2 to s1
          * Which create concat string
          * s2 can still be used since it has been borrowed (referenced)
  * ___CAREFUL___
    * Not all the character are represented on 1 byte!
    * It is possible that some letters ( Russian and hindi alphabet) are represented on 2 bytes!
      * If you want to iterate through a string
        * You should do it with `chars()`
          * `for c in "ThisGuy".chars() { println!("{}",c) }`
* _has map_
  * Has no macro to build it
  * Has to be imported
    * `use std::collection::HashMap`
  * Allow you to associate value with a particular key
  * It's a particular implementation of the more general data structure called map
  * __How to insert when no key__
    * `scores.entry(String::from("Yellow")).or_insert(50);`



[Follow this link to access documenation on collections](https://doc.rust-lang.org/stable/std/collections/)

