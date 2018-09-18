## Chapter 8 - Collections



* _Vector_
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
* _has map_
  * Allow you to associate value with a particular key
  * It's a particular implementation of the more general data structure called map



[Follow this link to access documenation on collections](https://doc.rust-lang.org/stable/std/collections/)

