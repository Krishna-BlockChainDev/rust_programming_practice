## DAY-2 (Advanced Rust programming Practices )

## Imp Links to play ( https://play.rust-lang.org/) 
## Docs (https://doc.rust-lang.org/book/)


## Parallel processing

```rust
extern crate rayon;
use rayon::prelude::*;

fn main() {
    let mut v = Vec::new();  // create a vector of floats, to multiply each by 0.9
    for i in 0..1024*1280 { v.push(i as f32); }
    v.iter_mut().for_each(     |x| *x = *x * 0.9 ); // single thread version //may issue warning
    v.par_iter_mut().for_each( |x| *x = *x * 0.9 ); // multiple threads version
    
    let v = (0..999).collect::<Vec<u32>>(); // par_iter is slightly different
    let tot = v.par_iter().map(|i|i*i).reduce(||0,|a,x|a+x); // parallel sum of squares
    // see https://docs.rs/rayon/latest/rayon/iter/trait.ParallelIterator.html#method.fold
    // for info on fold, reduce, map, etc, in Rayon
    
    very_slow_function1(); // two single threaded functions that take a long time
    very_slow_function2(); 
    
    rayon::join( || very_slow_function1()    // run them in parallel if appropriate
    		 || very_slow_function2() );
    
    s = "VeryLargeString ...pretend goes on for 10Mb "; // imagine 10Mb string
    s.chars().par_iter().do_stuff() // error // par_iter() cant work on string because
                                  // it doesnt know where to cut the byte boundaries
    //let v = s.iter().collect::<Vec<char>>9; // so convert to vector of char each 4 bytes
   // v.par_iter().map(|ch| ch.to_ascii_lowercase()).do_stuff() // now you can
		 
}
```

```bash
$ cargo add rayon  # add rayon dependency
$ cargo run        # installs rayon, runs program
```

```rust 
channels: todo
mutex: todo
concurrency: todo
```

## Smart pointers

Box is a smart pointer. It is not possible to become null or to point to invalid memory.

As an example, consider a tree data structure. In C you might have nodes that have
pointers to child nodes, and then at the leaf nodes , the child node pointers are NULL. 

In Rust, the nodes have Boxed pointers to child nodes. The Boxes are all wrapped with an Option.
So if there is no child, the Option is None. If there is a child, it is Some(Box(childnode)) 

```rust
struct Node { data: char, leftchild: Option<Box<Node>>, rightchild: Option<Box<Node>> }
let mut x = Node{data:'x',leftchild:None,rightchild:None};
let y = Node{data:'y',leftchild:None,rightchild:None};
let z = Node{data:'z',leftchild:None,rightchild:None};
(x.leftchild,x.rightchild) = (Some(Box::new(y)),Some(Box::new(z)));
// Node { data: 'x', 
//        leftchild: Some(Node { data: 'y', leftchild: None, rightchild: None }), 
//        rightchild: Some(Node { data: 'z', leftchild: None, rightchild: None }) }
```

Another way is to use Enum variants for the different node types, then you dont
even need to use Option at all. Each node is actually a different Variant so the leaf
nodes do not even have children they only have data, while the inner branch nodes have
children but do not have data.

See below section on "Enums - not like C" for more on how Enums work in Rust. 

```rust
enum Node { Branch(Box<Node>,Box<Node>), Leaf(char) }
let y = Node::Leaf('y');
let z = Node::Leaf('z');
let x = Node::Branch(Box::new(y),Box::new(z));
// print!("{:?}",x); // Branch(Leaf('y'), Leaf('z'))
```

https://doc.rust-lang.org/std/boxed/index.html

https://rosettacode.org/wiki/Huffman_coding#Rust

Arc, RC - reference counted pointers. Todo

## Functions and closures

```rust
fn add( a:i8, b:i8 ) -> i8 { b + a }     // 'return' keyword optional
fn getcodes(a:i8)->(i8,i32){ (9,a as i32*99) }   // multi return via tuples
let (x, s) = getcodes( 3 );           // assign multi-return w tuples
fn multy(a:i8,b:i8=5)->i8{a*b}   //error // Rust has no default parameters.
fn multy(a:i8,b:Option<i8>)->i8 {        // but you can fake it with Option unwrap_or
  a * b.unwrap_or(5) }                    // unwrap_or(x) means x is the default value
fn main(){ print!("{}",multy(3,None));}   // pass None to the func, result here=15
fm main(){ print!("{}",multy(3,Some(4)));} // awkward, tho, as normal calls require Some()
fn f(t:i8) {                              // nesting functions is OK
  fn g(u:i8) { u*5 };                     // g nested inside f
  let a = t + g(2);  }                    // g can be called from inside f
fn f2(t:i8) {             // however, nested functs cannot access outside variables 
  let mut m = 2;          // m is declared outside the scope of g2 block // to access m need to pass as a argument in g2 function
  fn g2(u:i8){u*5 + m};}  // error[E0434]: can't capture dynamic environment in a fn item
fn f(t:i8) {
  struct W{a:int}; t+1 }  // structs can be declared inside functions too


// function pointers
fn addtwo(t:i8)->i8{t+2}; // simple function, adds 2 to argument. 
println!("{}",addtwo(5)); // prints 7
let fp = addtwo;          // fp = function pointer to addtwo function
println!("{}",fp(5));     // now we can call fp() just like we called addtwo
fn f<F>(fp: F) where F: Fn(i8)->i8 { println!("{}",fp(1)) } 
// 'where F:Fn' lets us build a function that can accept another function as an argument
f(fp);  // call function f, passing a pointer to the 'addtwo' function. result=3

type ZFillCallback = fn(bottom:u32,top:u32)->u32;  // typedef of a function

fn maximum(t:i8,...) {} // error, can't have variable number of arguments.
                        // only macros! can be Variadic in Rust (see below)

// closures
let c = |x| x + 2;    // define a closure, which is kinda like a lambda function
let a = c(5);         // closures can be called, like functions. result = 7
let value = 5;        // a closure can also read values outside its scope
let d = |x| value + x;// d(n) will now add 'value' to any input n. (in this case,5) 
fn f<F>(fp: F) where F: Fn(i8)->i8 { println!("{}",fp(1)) }  // f takes a function as an argument 
f(c);                 // a closure can be passed, like a function pointer, result = 3
f(|x| x * value);     // and a closure can be anonymous, without a name. result = 5

for i in 0..4.filter(|x| x>1) // anonymous closures are used often with iterators (see below) 
print!("{} ",i)               // 2 3  (0 1 2 3 filtered to only values greater than 1)


```

## Unit tests, integration tests

Unit tests, placed in the same file as the code being tested

```rust
./src/lib.rs:

pub fn process(v:&mut Vec<u8>)->&Vec<u8>{ v.update(|x| f(x)) } // main function called by users
fn f(x:u8)->u8 { x*x }   // small piece of our code, to test in unit testing

#[cfg(test)]        // cfg -> section will only compiled during 'cargo test'
mod tests {         // namespace helper
    use super::*;   // bring in our functions above
    #[test]         // next function will be a single test
    fn test_f() { assert!(f(4)==16); } // test f() by itself (unit)
}
```

Integration tests, for overall crate, lives under ./tests/*.rs

```rust
./tests/file.rs:         // will only be built dring 'cargo test'
extern crate mypackage;  // include package we are testing
#test                    // treat next function as a test
fn bigtest() {           // not a unit test. instead, test overall code
	let mut d = vec![1,2,3];               // set up some typical data users would have
	let expected_results = vec![1,4,9];    // some results we expect
	assert!(process(d)==expected_results); // test what a user would typically call, process()
}
```

```bash
$ cargo test           # test build, will include cfg(test) sections
-> test_f passed       # cargo reports on passed tests
-> test_bigtest failed # cargo reports on failed tests
```

## Documentation

rust-doc and cargo doc allow automatic building of html documentation
for code. precede documentation of your code with three slashmarks
instead of the normal two slashmarks, like so:

```rust
/// blorg() returns the blorgification of input x
/// # Details
/// this code implements the krishna procedure
/// for blimfication of zorgonautic primes 
/// # Arguments
/// * `x` - typically a square number
/// # Safety
/// Cannot panic unless x overflows u64
/// # Example
///     let n = blorg(36); 
fn blorg(x:u64)->u64 {
   x+x*x
}
```

Then run rust-doc or cargo doc and view the result.

```bash
$ cargo doc
$ firefox target/doc/cratename/index.html
```

Good examples of the results are on https://crates.io
