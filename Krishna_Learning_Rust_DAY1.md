
## DAY-1 (Basic Rust programming Practices )

## Imp Links to play ( https://play.rust-lang.org/) 
## Docs (https://doc.rust-lang.org/book/)


## Rust in a Nutshell

* Syntax tokens somewhat similar to C / C++ / Go
* Ownership of memory enforced at build time
* Statically linked
* Not so Object-Orientish, tends to be Functional-ish
* Control flow using pattern matching, Option+Result enums
* Packages: 'cargo add' command, https://crates.io
* Testing: 'cargo test' command, #[test] unit tests, integration tests
* Concurrency: ownership, mutability, channels, mutex, crossbeam + Rayon packages
* Auto formatter: 'rustfmt filename.rs' (see rust-lang.org for installation)
* compiler engine: LLVM, no non-LLVM compilers yet
* To use raw pointers, low level, call C/C++: unsafe{} keyword + ffi package
* smart pointers and reference counted: Box, Rc, Arc
* online playgrounds: https://play.rust-lang.org, https://tio.run/ 
* A survivial horror game where griefing is ... oops wrong Rust
* Polymorphism: Traits, Trait Extensions, Trait Objects
* Generic programming (a la C++ Templates): Generic Types
  
## Hello World

See https://www.rust-lang.org for installation details.

```rust
fn main() {
    println!("Hello World");
}
```

```bash
$ rustc main.rs   
$ ./main
Hello World
```

## Hello Packages, Hello Tests, Hello Dependencies

```bash
$ rustup.sh         # install rust(https://doc.rust-lang.org/book/ch01-01-installation.html)
$ cargo new myproj  # start new executable project under myproj path
$ cd myproj         # cd into the new directory
$ ls -lR            # list our skeleton of files
src/main.rs         # main.rs, has main() entry point
Cargo.toml          # Cargo.toml defines packaging
$ $EDITOR Cargo.toml  # add dependencies and other details(Use VS-code as Editor)
[package]
name = "helloworld"
version = "0.1.0"
authors = ["krishna krishna <krishna@krishna.krishna>"]

[dependencies]
serde = "1.0.80"
# edit main.rs to say "extern crate serde;" and "use serde::*;"
$ cargo add chrono # auto add dependency w/o wediting Cargo.toml
# edit main.rs to say "extern crate chrono;" and "use chrono::*;"
$ cargo build      # downloads dependencies + builds main.rs 
$ cargo run        # runs program created from main.rs
$ cargo test       # runs tests (in parallel by default)
$ cargo test -- --test-threads=1  # run tests one at a time
$ cargo test -- --nocapture       # run tests, show output
#$ cargo run --example fundemo -- --arg # run example with arg (./examples subdir)
#$ cargo build --feature blargh     # build, enable the blargh feature of the crate
```

## Mutability basics

```rust
 let _x = false;           // all variable bindings are immutable by default
_x = true; // err         // compile error. can't change a variable which has an immutable binding
let mut _p = false;       // "mut" designates a binding as mutable
_p = true;               // ok, a variable with mutable binding can change;
```

## types, variables, declarations, initialization
```rust

let x: bool = false;    // let keyword
let k = false;          // rustc can determine some types automatically
let y: char = '上';     // all chars are 4 bytes
let 上 = 5; //err       // error. identifiers must be ASCII characters 
let a: i8 = -2;         // 8 bit signed integers, also i16, i32, i64  
let b: u8 = 200;        // 8 bit unsigned integers, also u16, u32, u64
let n: f32 = 0.45;      // 32 bit float (automatcally converted+rounded from base-10 decimal to binary)
let n2 = 42.01f64;      // 64 bit float literal of the number 42.01 (approximately)


let r: [u8;3] = [3,4,5];          // array of 3 int, immutable, cannot grow or change values
let mut rm: [u8;3] = [3,4,5];     // same as r but mutable. cannot grow, but values can change.
let mut s1 = [0;500];             // array of 500 integers, each initialized to 0.
s1[0..200].fill(7);               // set the first 200 integers to the value 7
s1[400..].fill(5);                // set the last 100 integers to the value 5
let s2 = &r[0..2];                // slice of array, s==&[3,4]
let s3 = &r[0..2][0];             // index into slice, s==3
let s4 = &r[1..];                 // slice from index 1 to end
let s5 = &r[..2];                 // slice from beginning to index 2


let mut u:Vec<u8> = Vec::new();   // create empty vector of unsigned 8 bit int, can grow
let mut v = vec![3,4,5];          // initialize mutable vector using vec! macro
let w = vec![1,12,13];            // vectors can be immutable too
let z = (0..999).collect::<Vec<u32>>(); // init Vector from Range (0..999)
u.push( 2 );                  // add item to vector
v.rotate_right( 1 );          // in-place rotate of mutable vector [5,3,4]
v.rotate_left( 1 );           // in-place rotate [3,4,5]
let s6 = &mut v[1..];         // mutable slice, allows vector-like operations on a...
s6.rotate_right( 1 );         // ...subslice of the vector [3,5,4] 
u.pop();                      // vectors can pop, return+remove last input (like a stack)
v.contains(&3);               // true if vector contains value
v.remove(1);                  // remove the nth item from a vector...
v.append(&mut u);             // append v with u (u becomes empty ([]), both mutable)
v.extend(w);                  // extend v with w (v owns w, w can be immutable)
v.resize(200,0);              // make vector have 200 elements, set them to 0
v.resize(4398046511104,0);    // memory allocation of 4398046511104 bytes failed. core dumped.
                              // There is no malloc() style checking for success, Rust just crashes
v[0..100].fill(7);            // set the first 100 elements in v to the value 7
v[150..].fill(9);             // set the last 50 elements in v to the value 7
v.fill(9);                    // set all elements of v to have the value 9
let x = &w[1..];              // get a slice of a vector (a view into it's elements)
print("{:?}",x);              // [12,13]; 
let vs = v.len();             // length of vector


let (p,d,q) = (4,5,6);        // tuple() can assign multiple variables at once
print("{}",p);                // you can use them alone after tuple assignment
let m = (4,5,"a");            // tuples can have multiple different types as elements
let (a,b) = (m.1, m.2);         // tuple dereference with .1, .2, (output is : a:5,b:a)
let (c,d) = (m.p, m.q); //err   // error, cannot index into a tuple using a variable


let s = String::from("上善若水 "); // String is a heap variable. Strings are UTF8 encoded.
let s2 = "水善利萬物而不爭";       // "" literals are type &str, different from String
let s3 = &s;                    // & when prefixed to String gives &str
let s4 = s2.to_string();        // create String from &str
let s5 = format!("{}{}",s2,s3); // concatenate &str to &str
let s6 = s + s2;                // concatenate String to &str
for i in "말 한마디에 천냥 빚을 갚는다".split(" ") {print!("{i}");} // split &str
s.chars().nth(4);               // get nth char. 
s.get(2..).unwrap(); // ERROR   // get substring failed because 上 was 3 bytes
s.get(3..).unwrap(); 		// 善若水
s.trim();                       // 上善若水, no trailing space
s.starts_with("上善");          // true
s.ends_with("水");              // true

let i4 = s.find('水').unwrap_or(-1); // index of character (not always a byte offset, b/c utf8)
let hellomsg = r###"                 // Multi-line &str with embedded quotes
 "Hello" in Chinese is 你好 ('Ni Hao')
 "Hello" in Hindi is नमस्ते ('Namaste')
"###;


let x = vec![5,12,13];           // indices into slices/arrays/vectors must be of type usize
let y = 2 as u8;                 // using u8, u16, etc will not work
print!("{}",x[y]);               // error[E0277]: the type `[{integer}]` cannot be indexed by `u8`
let z = 2 as usize;              // usize is the pointer size. used in loops, vector length, etc
print!("{}",x[z]);               // ok.   there is also "isize" if usize needs to be signed



const BILBOG: i32 = 10;         // constant
static ORGOG: &str = "zormpf";  // static, global-ish variable
static FOOBY: i32 = 5;          // statics are only mutable inside unsafe{} blocks. 
static Z_ERRMSG : [&str;2] = ["need input","need more input"]; // static strings
type Valid = bool;              // typedef ( make your own type names ) 
let mut v = vec![1u8,2u8,3u8];  // determine the type of expression expr by looking at rustc error
println!("{}",v.iter_mut());    // for example, if we want to know the type of v, build an error
println!("{}",v.iter_mut());    // type of v.iter_mut() is std::slice::IterMut<'_, u8>`
 ```


## Operators

```rust
1 + 2 - 3 * 4 / 5  // arithmetic add, minus, multiply, divide
7 % 5              // modulo (remainder)
& | ^              // bitwise and, or, xor
<< >>              // leftshift, rightshift, will crash on overflow
// note that in C, overflowing << is actually undefined. 
// Rust has multiple versions, each defined. 
let a:u8 = 0b10110011; // print!("{:08b}",a); // 0b10110011 padded binary output 
//a.rotate_left(1)    // 01100111 circular bit rotation, out of left -> in at right
//a.wrapping_shl(1)   // 01100110 this destroys the left-most bits that would cause overflow
//a.overflowing_shl(1)// 01100110,false returns tuple (value,did it overflow the number type)  
//a.rotate_right(4)   // 11011001 circular bit rotation to the right
!a                  // 01001100 bitwise not
a == b != c < d <= e > f >= g  // logical comparison
a && b || c ! d    // logical boolean, and, or, not



let a = 5;         // pointer + dereference example
let b = &a;        // &a is 'address of a' in computers memory
let c = *b;        // *b is contents of memory at address in b (dereference)
print!("{c}");     // 5

overloading: see struct
```

## Run time errors, Crashing, panic, except, unwrap, Option, Result

```rust
panic!("oops");             // panic!() instantly crashes program
let v = vec![3,4,5];
let a = v[0];               // ok, normal lookup, a is 3
let b = v[12];              // will call panic! at runtime, v[12] doesn't exist
                            
```

```bash
$ export RUST_BACKTRACE=1  #index out of bounds: the len is 3 but the index is 12
$ cargo run    # will tell you exact line where panic occured, with call stack trace
```

### Option - a type for functions that may return Some thing, or None thing 

```rust
let v = vec![3,4,5];        // create Vector with three elements. then try to get() the element at index 12. 
let c = v.get(12);          // there is no element at index 12, but this will not crash, get returns an Option
print!("{:?}",v.get(12));   // prints the word "None", since there is no 12th element in v
print!("{:?}",v.get(0));    // prints the word "Some(3)", because there is an element at index 0
                            // Options have a value of either None, or Some(item), i.e. they are "Enums"
let e = v.get(0).unwrap();  // ok, 'unwrap' the Option returned by get(0), e is now 3
let d = v.get(12).unwrap(); // this crashes. 'unwrap' of a None Option will call panic!
let f = v.get(5).unwrap_or(&0); // unwrap_or gives a value if get() is None. f = 0
```
Option and Match - a control flow similar to **if else** but with more error checking at compile time
```
let x = v.get(12);
match x { Some(x)=>println!("OK! {x}"),  // print OK if v has 13th item
	  None=>println!("sad face"), }   // otherwise print sad face
// you will get a compile-time error if you forget to handle both Some and None
```

### Result - a type like Option but with Ok() and Err() instead of Some() and None. 

```rust

match std::env::var("SHLVL") {  // env::var() returns a std::result::Result(<T,E>) enum type. 
	Ok(v)=>println!("SHLVL = {v:?}"), // if OK, std::env returns value of Environment variable
	Err(e)=>println!("error message: {:?}",e.to_string()) }; // if not, error message

note there is also std::io::Result used a lot in file operations. 

```

**If Let** - A control statemnt like match but with a no-op for None or Err() 

```rust

if let Some(x) = v.get(12) { println("OK! {x}"); } 
// we don't have to type out a handler for None. it does nothing. 

if let Ok(x) = std::env::var("SHLVL") { println!("SHLVL = {x:?}"); }
// if the Result is Err, then nothing is done. 
```

Option in particular can prevent the use of null pointers, preventing crashes one might see in C/C++.
Say we are keeping tack of Owls, we know each Owls name, but not necessarily their last location.
	
```rust
struct Owl { name: String, 
	last_location: Option<String>  }   // in C++ location might be a *char which could init as NULL 

let owls = [Owl{name:"Opv".to_string(),last_location:None},
            Owl{name:"Marty".to_string(),last_location:Some("Barn".to_string())}];

for owl in &owls {
	match &owl.last_location {
		None=>println!("Owl named {} - location is not being tracked",owl.name),
		Some(loc)=>println!("Owl named {} - last known location was {}",owl.name,loc),
}}
// note that we did not have to check for null pointers, nor derefernece any
// pointer. if we forgot to check for None, the compiler would give an error at compile time.
// there are no null-related runtime errors possible.

```

Note that there are no Exceptions. Panic/Option/Result/multi-value-return are used instead. 

## Printing

```rust
println!("Hello, 你好, नमस्ते, Привет, ᎣᏏᏲ");    // unicode text is OK 
print!("Hi, is {}x{}={} ?",6,9,42);           // curly braces {} get replaced by arguments
                                              //Hi, is 6x9=42 ?
let (meepo,beepo) = (5,6);                    // print variables without using arguments..
print!("{meepo}:{beepo}");                    // .. by putting the names inside {}

let v = vec![1,2,3];
println!( "v[0] from {:?} = {}", v, v[0] )    // {:?} can print lots of special types
println!("{:02x?}",v);                        // {:02x?} can print 2 digit hex of vector
//let s = format!( "x coord={}", p.X )          // print to string 
//s2 := fmt.Sprintf( "{e}", 17.0 )              // another way to print to string
use std::fmt::Write;                          // yet another way - like C++ stringstream
let mut s3 = String::new();                   // String implements fmt::Write so we can
match writeln!(s3,"Hello There") {            // write to a String like a file.
   Ok(_)=>(), Err(e)=>println!("error writing to string {} {:?}",s3,e),} 
writeln!(s3,"Hello Too").unwrap_or(());       // the concise version w/o any error msg

// C / printf style formatted output: 
println!("hex:{:x} bin:{:b} sci:{:e}",17,17,17.0); // hexadecimal, binary, etc. 
//   "hex:11 bin:10001 sci:1.7e1"
// Pad with zeros: 
println!("dec:{:#04} hex:{:#06x} bin:{:08b} sci:{:09e}",17,17,17,17.0); 
//   "dec:0017 hex:0x0011 bin:00010001 sci:00001.7e1"
println!(" {:.40} ", 1.0f32/3.0f32 );  // print 40 digits of precision for floating point
//   "0.3333333432674407958984375000000000000000" 
println!(" {:>4} {:>4} ", 232, 8 );    // pad as columns, width 4 spaces, align right
//   " 232    8"
let mut s=String::new();               // build string, concatenate over lines 
s.push_str(&format!("{} {} ",1,2));
s.push_str(&format!("{} {}\n",3,4));    // "1 2 3 4\n"
let mut s2=String::new();              // alternate version, same goal  
write!(s2,"{} {}",1,2).unwrap_or(()); 
writeln!(s2,"{} {}",3,4).unwrap_or(()); // "1 2 3 4\n"

println!("\u{2766}");                  // ❦  unicode floral heart, character hex 2766 

// derive Debug can make your own structs, enums, and unions printable by {:?}
#[derive(Debug)]
struct Wheel{radius:i8}                
println!("{:?}",vec![Wheel{radius:4},Wheel{radius:3},Wheel{radius:2}]);
// [Wheel { radius: 4 }, Wheel { radius: 3 }, Wheel { radius: 2 }]

// If you want to customize your debug output, you can implement Debug yourself
use std::fmt;
struct Wheel{radius:i8}     
impl std::fmt::Debug for Wheel {
  fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result{
    write!(f, "輪:徑[{}]", self.radius)
}}
println!("{:?}",vec![Wheel{radius:4},Wheel{radius:3},Wheel{radius:2}]);
// [輪:徑[4], 輪:徑[3], 輪:徑[2]]

// fmt::Display makes your own structs and enums printable with ordinary {} symbol
impl std::fmt::Display for Wheel{
  fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result{
    write!(f, "W[{}]", self.radius)
   }
}

// Display for enums
pub enum Apple{PinkLady,HoneyCrisp}  
impl std::fmt::Display for Apple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self { Apple::PinkLady=>write!(f, "Ap:PLad"),
                     Apple::HoneyCrisp=>write!(f, "Ap:HonCr"),
		     }
    }
}
```

### loop, for, while
```rust

for i in 0..10 {print!("{},",i)};          // 0,1,2,3,4,5,6,7,8,9
for i in (0..10).rev() {print!("{},",i)};    // 9,8,7,6,5,4,3,2,1,0
for i in (0..10).step_by(2) {print!("{},",i)};              // 0 2 4 6 8 (step_by is used as iterator)
for i in (0..10).skip(1).step_by(2) {print!("{},",i)};      // 1 3 5 7 9
for i in (0..10).rev().step_by(2){print!("{},",i)};         // 9 7 5 3 1 
for i in (0..=10).rev().step_by(2){print!("{},",i)};        // 10 8 6 4 2 0 
for i in (0..=10).step_by(2){print!("{},",i)}     ;         // 0 2 4 6 8 10 
for i in (0..9).rev().step_by(2){print!("{},",i)} ;         // 8 6 4 2 0 
for i in (0..9).step_by(2){print!("{},",i)}       ;         // 0 2 4 6 8 
for i in (0..10).cycle().skip(5).take(10){print!("{},",i)}  // 5 6 7 8 9 0 1 2 3 4 

let v = vec![3,5,7];
for n in &v { println!("{}",n) }             // for loop over vector
for (i, n) in v.iter().enumerate() {        // iterate with (index, item) tuple 
	println!("{},{} ", i, n);}          // 0,3 \n 1,5 \n 2,7

let mut i = 0;				    // while loop
while i < 10 {print!("{}",i); i += 2;}      // 0 2 4 6 8
let mut j:u8 = 9;                           
while j > 0 {print!("{}",j); j -= 2;}       // 9 7 5 3 1 Panic! j is unsigned but goes below 0 

let mut i = 0;                              // loop
loop { i=i+1; if i<10 { break; } };	    // plain loop, exit with break;
let x = loop { i=i+1; if i>=10 {break i;} } // loop that returns value, x = 10
```

Iterators as an alternative to loops

```rust
let v = vec![3,5,7,11];                    // vector to iterate
v.iter().for_each(|x| print!("{} ",x));    // for_each over iterator, 3 5 7 11
print!("{}",v.iter().fold(0,|a,i| a+i));   // adds [0]+[1]+[2]+[3], prints 26. 
// for more info , see iterators/functional section below
```

```rust
// While Let, can be used in situations where we expect Option::Some() for several iterations,
// but we expect a Option::None() to be at the end of the iterations. For example:
let mut x = (0..12).filter(|x| x%2==0);          // x is an iterator
while let Some(i) = x.next() {print!("{}:",i);}  // While loop over the iterator
// 0:2:4:6:8:10:   // prints the even numbers between 0 and 12
```
