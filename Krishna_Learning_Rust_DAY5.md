
## DAY-5 (Advanced Rust programming Practices )

## Imp Links to play ( https://play.rust-lang.org/) 
## Docs (https://doc.rust-lang.org/book/)

## Math

### arithmetic

```rust

let x=250u8;           // addition can crash, max value of u8 is 255
println!("{}",x+10);   // crashes, attempt to add with overflow
println!("{}",x.wrapping_add(10));   // wraps around, result: 5
println!("{}",x.saturating_add(10)); // stays at max, result: 255
println!("{}",x.wrapping_sub(u8::MAX)); // wraparound subtraction
std::f32::MIN;         // minimum binary floating point 32 bit value
std::i32::MAX;         // maximum 32 bit integer value

let b = -5;
let c = b.abs();       // error, abs not defined on generic integer
let b = -5i32;
let c = b.abs();       // ok, abs is defined on i32, 32 bit integer
42.25f32.round();      // can also call functions on float literals    
9.0f32.sqrt();         // square root approximation
9.sqrt()               // error, sqrt only for floats
let x = 3/4;           // 0
let y = 3.0/4;         // error, no implementation for `{float} / {integer}`
let z = 3.0/4.0;       // 0.75
```

Exponentials, exponentiation, power, raising to a power

```rust
let p1 = 2.pow(32) //err    // error[E0689]: can't call method `pow` on ambiguous numeric type `{integer}`
let p2 = 2_u8.pow(32) //err // thread 'main' panicked at 'attempt to multiply with overflow'
let p3 = 2_u8.checked_pow(32); // v becomes "None".
let p4 = match 2_u8.checked_pow(32) { Ok(n)=>n,None=>0 } // match a checked_pow b/c it returns Option
let p5 = 2u8.checked_pow(k).unwrap_or(0); // 0 if overflow, simpler than match 
let p6 = 2_u64.pow(32)                    // ok
```

### data conversion

```rust
let x: u16 = 42;           // integer conversion. start with u16
let y: u8 = x as u8;       // cast to unsigned 8 bit integer
let z: i32 = x as i32;     // cast to signed 32 bit integer
let bez = z.to_le_bytes(); // get individual 8 bit bytes in the 32 bit int 
let bbz = z.to_be_bytes(); // same, but in big endian order  
println!("{:?},{:?},{:?},{:?}",bez[0],bez[1],bez[2],bez[3]); // 42,0,0,0
println!("{:?},{:?},{:?},{:?}",bbz[0],bbz[1],bbz[2],bbz[3]); // 0,0,0,42

let a = u32::from('A')              // get integer value of char. all chars are 4 bytes.
let a = char::from_u32(68)          // get char value of an integer (utf8/ascii)
let a = char::from_digit(4,10)      // get char '4' from integer 4, base 10

let s = [0,1,2,3];                  // u8 to u32, start with array of four u8
let ls = u32::from_le_bytes(s);     // convert to u32, little endian
let bs = u32::from_be_bytes(s);     // convert. to u32, big endian
println!("{:?} {:?}",ls,bs);        // 50462976 66051
let s2 = vec![0,0,0,1,2,3,4,5,6,7]; // vector of ten u8
let ars2 = [s2[2],s2[3],s2[4],s2[5]]; // array of four u8
let be = u32::from_be_bytes(ars2);    // create u32 from 3rd-6th bytes of ars2
println!("{:?}",be);                // 66051

// converting vectors of integers
let mut v1 = vec![0u8;2];                // 2 u8, each with '0'
let v2 = vec![0xffeeaa00u32,0xff0000dd]; // two u32 in a vector
v2.extend(v2);  // error the trait bound `Vec<u8>: Extend<u32>` is not satisfied
for d in v2 { v1.extend(d.to_le_bytes()) }; // convert each u32 by itself
println!("{:?}",v1) // [0, 0, 0, 170, 238, 255, 221, 0, 0, 255]

// integer conversion using byteorder crate
extern crate byteorder; // modify your Cargo.toml to add byteorder crate. then:
use byteorder::{BigEndian, ReadBytesExt, NativeEndian, ByteOrder, LittleEndian, WriteBytesExt};
let arr = [0,1,2,3];
let s = NativeEndian::read_u32(&arr[0..4]);   // array of four bytes into u32.
let mut v = vec![0u8];
s.write_u8( v );

let vx = vec![0u8,1,2,0xff,4,5,6,0xff];     // vector of u8
//let mut vy = vec![0u32];                  // vector of u32
//LittleEndian::read_u32_into(&vx,&mut vy); // panic, vy cant hold vx
let mut vy = vec![0u32;2];                  // vector of u32
LittleEndian::read_u32_into(&vx,&mut vy);   // convert u8s to u32s
println!("{:x?}",vy);                       // [ff020100, ff060504]
let mut vx2 = vec![0u8;8];                  // new vector of u8
LittleEndian::write_u32_into(&vy,&mut vx2); // convert back to u8s
println!("{:02x?}",vx);                     // [00, 01, 02, ff, 04, 05, 06, ff]

// converting vectors of integers using unsafe mem::transmute
let v = vec![0u8;80]; let i = 0;
let n:i32 = {unsafe { std::mem::transmute::<&[u8],&[i32]>(&v[i..i+4])}}[0]; 
let x:f32 = {unsafe { std::mem::transmute::<&[u8],&[f32]>(&v[i..i+4])}}[0];
let mut block = vec![0u8;64];  // convert entire block at once
let mut X = unsafe { mem::transmute::<&mut [u8], &mut [u32]>(&mut block) }; 
#[cfg(target_endian = "big")]   // deal with endian issues if needed
for j in 0..16 { X[j] = X[j].swap_bytes(); }




let s = format!("{:e}",0.0f32);    // convert float32 to base-10 decimal string (scientific format)
let n = s.parse::<f32>().unwrap(); // parse float from string, panic/crash if theres an error
let mut n = 0.0f32;                // parse float from string, without panic/crashing
match s.parse::<f32>() {
	Err(e)=>println!("bad parse of {}, because {}",s,e),
	Ok(x)=>n=x
}
let a = "537629.886026485"                        // base-10 decimal string to binary float point
print!("{:.50}",a.to_string().parse::<f32>().unwrap());// 537629.875000000000000000000000000000000 
let b = 537629.886026485;    print!("{:.50}",b);      // 537629.886026485008187592029571533203125
let c = 537629.886026485f32; print!("{:.50}",c);      // 537629.875000000000000000000000000000000
let d = 537629.886026485f64; print!("{:.50}",d);      // 537629.886026485008187592029571533203125

let ch='e';
ch.is_digit(16) // true, 'e' is a numerical digit in base 16
ch.is_digit(10) // false, 'e' is not a numerical digit in base 10, only 0-9 are.
let y:u32 = ch.to_digit(16).unwrap_or(0);  // convert 'e' into 15, since base=16. if fail, make y 0
let n = 'x'.is_alphabetic(); // detect whether letter
println!("𝄠 is hex 0x{:06x}, or decimal {}",'𝄠' as u32,'𝄠' as u32); // unicode codepoint
println!("a is hex 0x{:06x}, or decimal {}",'a' as u32,'a' as u32); // for <128 this is the ascii code

let m = 0b0001; // binary literal
let m = 0o0007; // octal literal
let m = 0x000f; // hexadecimal literal
let (a,b,c) = (0b00_01,0o00_07,0x00_0f); // literals with underscores for ease of reading

println!("{:x}",0x12345678u32.swap_bytes());  // 0x78563412 32-bit byteswap 



```

#### string conversion

```rust

use std::i64;
let z = i64::from_str_radix("0x1f".trim_start_matches("0x"), 16).unwrap(); // hex string to integer
let q = i64::from_str_radix("0b10001".trim_start_matches("0b"), 2).unwrap(); // binary string to integer

println!("{:?}","abc".as_bytes()); // &[u8] slice, [97, 98, 99] ( utf8 string into bytes )
println!("{:?}","abc".as_bytes().to_vec()); // Vec<u8> [97, 98, 99] string into slice into Vector of bytes
println!("{:?}","ᏣᎳᎩ".as_bytes()); // [225, 143, 163, 225, 142, 179, 225, 142, 169] ( Cherokee utf8 )
let s = b"\x61\x62\x63"; // b precedes sequence of bytes, hexadecimal, pseudo-string form
println!("{}",std::str::from_utf8( s ).unwrap()); // abc // decodes utf8 to string, crash if invalid utf8

println!("{:?}", "सूर्य नमस्कार्कार".as_bytes()); // [224, 164, 184,...] 
let s = [224, 164, 184, 224, 165, 130, 224, 164, 176, 224, 165, 
         141, 224, 164, 175, 32, 224, 164, 168, 224, 164, 174, 224, 
	 164, 184, 224, 165, 141, 224, 164, 149, 224, 164, 190, 224, 164, 176];
println!("{}",std::str::from_utf8( &s ).unwrap()); // सूर्य नमस्कार   
println!("{:?}",std::str::from_utf8( &s ).unwrap()); // "स\u{942}र\u{94d}य नमस\u{94d}कार" different decoding
```

```rust
let s = "hello" ;                   // s = &str
let s = "hello".to_string();        // s = String
let m = s.replace("hello","new");   // m = "new"
let y = s.to_uppercase();           // y = "NEW"

// str.as_bytes() converts to slice, which implements Read
let s = "Reedeth Senek, and redeth eek Boece";
let s2= "Ther shul ye seen expres that it no drede is"
let buf = &mut vec![0u8; 64];
s.as_bytes().read(buf).unwrap();
s2.as_bytes().read(&buf[30]).unwrap();
```

Regular expressions typically use an external crate.
Syntax for regex: https://docs.rs/regex/1.1.2/regex/index.html#syntax

In cargo.toml,
[dependencies]
regex = "1.1.0"

In main.rs:
```rust
use regex::Regex;
let text = r###"The country is named France, I think the biggest city is Paris;
the Boulangerie are beautiful on la rue du Cherche-Midi"###;              // multiline string

let re = Regex::new(r"country is named (?P<country>.*?),.*?biggest city is (?P<biggestcity>.*?);").unwrap();
let caps = re.captures(text).unwrap();
println!("{}, {}",&caps["biggestcity"],&caps["country"]);                  // Paris, France

let re = Regex::new(r"(?ms)city is (?P<citname>.*?).$.*?beautiful on (?P<streetname>.*?)$").unwrap();
let caps = re.captures(text).unwrap();
println!("{}, {}",&caps["streetname"],&caps["citname"]);               // la Rue de Cherche Midi, Paris
```


### comparison and sorting

```rust

use std::cmp           			// max/min of values
let a = cmp::max(5,3); 			// maximum value of 2 integers
let b = cmp::max(5.0,3.0); 		// build error, floats cant compare b/c of NaN,Inf
let v = vec![1,2,3,4,5];   		// list, to find max of
let m = v.iter.max().unwrap(); 		// max of numbers in a list, crash on error
match v.iter().max() { 			// max of numbers in a list, don't crash
    Some(n)=>println!("max {}",n), 	// do stuff with max value n
    None=>println!("vector was empty")
}
let m = v.iter.max().unwrap_or(0);      // max of numbers in a list, or 0 if list empty

v = vec![1,3,2];                               
v.sort();                                      // sort integers
v = vec![1.0,3.0,2.0];                         // sort floats
v.sort();                                      // error, float's NaN can't be compared
v.sort_by(|a, b| b.cmp(a));                    // sort integers using closure
v.sort_by(|a, b| a.partial_cmp(b).unwrap());   // sort floating point using closure
v.min_by(|a,b| a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal)); // minimum value
println!("{}",vec![1.,2.,3.].iter().cloned().fold(std::f64::MIN, f64::max)); // 3
println!("{}",vec![1.,2.,3.].iter().cloned().fold(std::f64::MAX, f64::min)); // 1

struct Wheel{ r:i8, s:i8};                     // sort a vector that holds a struct
let mut v = vec![Wheel{r:1,s:2},Wheel{r:3,s:2},Wheel{r:-2,s:2}];
v.sort_by(|a, b| a.r.cmp(&b.r));               // sort using closure, based on value of field 'r'
v.sort_by(|a, b| a.r.cmp(&(&b.r.abs())));      // sort using closure, based on abs value of r

fn compare_s( a:&Wheel, b:&Wheel ) -> std::cmp::Ordering {      // sort using a function
     a.s.partial_cmp(&b.s).unwrap_or(std::cmp::Ordering::Equal)
}
v.sort_by( compare_s );                        
```

### Pseudo randoms

Pseudo Random number generators, aka PRNGs

```rust
extern crate rand;   /// add rand to dependencies in Cargo.toml
use rand::prelude::*; 
let x = rand::random(); // boolean
let y = rand::random::<int>(); // integer
let x = rand::thread_rng().gen_range(0, 100); // between
let mut rng = rand::thread_rng();
let z: f64 = rng.gen(); // float 0...1
let mut nums: Vec<i32> = (1..100).collect();
nums.shuffle(&mut rng);
```
```rust
// alternative, without needing external crate, based on codeproject.com by Dr John D Cook
// https://www.codeproject.com/Articles/25172/Simple-Random-Number-Generation
// License: BSD, see https://opensource.org/licenses/bsd-license.php
use std::time::{SystemTime, UNIX_EPOCH};
let mut m_z = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time reversed").as_millis();
let mut m_w = m_z.wrapping_add( 1 );
let mut prng = || {  m_z = 36969 * (m_z & 65535) + (m_z >> 16);
                     m_w = 18000 * (m_w & 65535) + (m_w >> 16);
                     m_z.rotate_left( 16 ) + m_w };
let buf = (0..1000).map(|_| prng() as u8).collect::<Vec<u8>>();
// creates 1000 pseudo random bytes in buf, using Closure named prng
```

### Hashing

```rust
use std::collections::hash_map::DefaultHasher; 
use std::hash::{Hash, Hasher};
let mut hasher = DefaultHasher::new(); // hashing, via a Hasher object, which holds state
let x = 1729u32;
let y = 137u32;
hasher.write_u32( x );                 // input x to hash func, store output in hasher 
hasher.write_u32( y );                 // input y, combine w existing state, store in hasher
println!("{:x}", hasher.finish());     // .finish() function Hasher gives current state
345.hash(&mut hasher);                 // update hasher's state using '.hash()' trait
println!("{:x}", hasher.finish());     // .finish() does not 'reset' hasher objects

```

### Date and Time

```rust
use std::time::{Instant};
let t = Instant::now();
// do something for 5.3 seonds
println!("{}",t.elapsed().as_secs());          // 5, seconds, rounded
println!("{}",t.elapsed().subsec_millis());    // 300. remainder in milliseconds

use chrono::prelude::*;
println!("{:?} ", Utc::now().to_rfc2822());
println!("{:?} ", Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());
println!("{:?} ", Local::now().to_rfc2822());
println!("{:?} ",DateTime::parse_from_str("2014-11-28 21:00:09 +09:00", "%Y-%m-%d %H:%M:%S %z"));
println!("{:?} ",Utc.with_ymd_and_hms(2014, 11, 28, 12, 0, 9).unwrap());
println!("{:?} ",Utc.timestamp(1500000000, 0)); // Epoch time seconds, aka mtime on Unix files.
"Tue, 17 Jan 2023 03:12:07 +0000" 
"2023-01-17 03:12:07" 
"Mon, 16 Jan 2023 21:12:07 -0600"
Ok(2014-11-28T21:00:09+09:00)
2014-11-28T12:00:09Z
2017-07-14T02:40:00Z
```

## Annotations

Aka hashtags aka warning thingies. These statements typically affect compilation and begin with a hashtag, and square brackets. They are sort of like a mix of #pragma and #ifdef from C. 

```rust
#![allow(dead_code)]             // stop compiler from printing warnings on unused funcs/vars,   
#![allow(unused_variables)]      // this is good when you are working on a library
//  = note: #[warn(unused_assignments)] on by default
#![allow(unused_assignments)]    // you can take most warnings, and disable by changing 'warn' to 'allow'

// by removing ! and placing on line before a fn, you can disbale warning only for the function
#[allow(unused_assignments)]     
fn zoogle_poof( n:u8 )->u8 { let a = 0; 5+n };

let mut a = 0x00ffee22;          // modify numbers for big endian machines.
#[cfg(target_endian = "big")]    // target = machine the code will be run on
a = a.swap_bytes();              // the line (or block) immediately following the # gets affected.
```

## Linked Lists

Textbook C/Java-style implementations of linked lists often involve ownership that is not allowed by the Rust borrow checker. However it can be accomplished. There is builting "LinkedList" type in std::collections, also some resources from A. Beinges :

https://cglab.ca/~abeinges/blah/too-many-lists/book/

You can also implement a linked list as a Vector of structs, using integer indexes into the vector instead of pointers.

## FFI, Calling C functions, porting C code

foreign function interface, aka calling code from other languages. 

layout
```bash
   src/lib.rs
   src/ccode.c
   build.rs
   Cargo.toml
```

Cargo.toml
```rust
[package]
name = "duh"
version = "0.1.0"
authors = ["akhmatova"]
build = "build.rs"
[build-dependencies]
cc = "1.0"
libc = "0.2.0"
```

build.rs
```rust
extern crate cc;

fn main() {
    cc::Build::new().file("src/ccode.c").compile("ccode");
}
```

src/ccode.c
```C
#include <stdio.h>
int quadrance( int x1, int y1, int x2, int y2) {
	return (x1-x2)*(x1-x2)+(y1-y2)*(y1-y2);
}
char * blerg( *char txt ) {
	printf("This is C. Here is text from Rust: %s", txt);
	int fd = open("/tmp/blerg",0);
	return "blerg";
}
void printerrs() {
        int code = errno;
        printf("C: errno: %i strerror: %s\n",code,strerror(code));
}
```

src/main.rs
```rust
use std::os::raw::{c_int,c_char,c_void};
use std::ffi::{CString,CStr};  
extern "C" {
    fn quadrance(x1: c_int, y1: c_int, x2: c_int, y2: c_int) -> c_int;
    fn blerg(v: *const char)->*const c_char;
    fn printerrs()->c_void;
}
fn main() {
    unsafe {
        println!("4^2+3^2={:?}", quadrance(0, 0, 4, 3));
	let tmp = CString::new("blarg").unwrap(); // CString must be assigned
	let msg = blerg(tmp.as_ptr()); // so as_ptr() will have something to point at 
	println!("This is Rust. Here's text from C: {:?}",CStr::from_ptr(msg));
	printerrs(); // was there an error?
    }
}
```

Run:
```bash
don@oysters:~/duh$ cargo run
   Compiling duh v0.1.0 (/home/don/duh)                                         
    Finished dev [unoptimized + debuginfo] target(s) in 1.95s                   
     Running `target/debug/duh`
4^2+3^2=25
This is C. Here is text from Rust: blarg
This is Rust. Here's text from C: blerg
```

See also: https://s3.amazonaws.com/temp.michaelfbryan.com/index.html

c++ - https://hsivonen.fi/modern-cpp-in-rust/

https://doc.rust-lang.org/nomicon/ffi.html

Michael Bryan's unofficial guide
https://s3.amazonaws.com/temp.michaelfbryan.com/index.html

Using char** C functions:
https://stackoverflow.com/questions/42717473/passing-vecstring-from-rust-to-char-in-c

Note that much C code does not initialize structs or memory, which may be forgotten
when one is used to Rust
http://www.ex-parrot.com/~chris/random/initialise.html

Structs and bindgen auto header conversion
https://medium.com/dwelo-r-d/using-c-libraries-in-rust-13961948c72a


Setting CFLAGS examples:

modify build.rs:

```rust
extern crate cc;

fn main() {
    std::env::set_var("CFLAGS","-w");  // turn off all warnings
    std::env::set_var("CFLAGS","-v");  // run compiler in verbose mode
    std::env::set_var("CFLAGS","-v -O2");  // optimize level 2 + verbose mode
    cc::Build::new().file("src/ccode.c").compile("ccode");
}
```

Unions:

In Rust, Enums are typically preferred over union, as a traditional C-style union is
only available by using unsafe{}. But unions can help talking to / porting C code:
``` rust
#[repr(C)]
union Borgle { dorgle: u32, porgle: u8[4] }
let mut a=Borgle{dorgle:1234};
unsafe{ a.dorgle = 0xa0ca0c };
```

Goto vs labels, loops, and breaks:

Rust does not have Goto, however sometimes a similar result 
can be achieved using labelled loops and breaks. 

```rust
    'label1: loop {
    	if zimblatz == 5 {
	    break; // breaks labe1 loop
	}
        'label2: for frobnoz in 0..4 {
            if blorg==KUMQUAT {break;} // breaks label2 only
	    while j<5 {
	    	if snog==7 { 
			// goto end of label1 loop immediately
			break 'label1; 
		}
		j+=1;
	    }
        }	
    } // end of 'label1 loop
    do_something();
```

Pointer arithmetic

```rust
use std::raw::c_uchar;
unsafe{
	let mut x = some_c_func() as *const c_uchar;
	x = x.offset(1);  // instead of x++
}
```

## source code layout and modules

Here is an example source code organization layout for a simple library, which has a handful of modules, some examples, integration tests, benchmarking, and two executable binaries.
 
```bash
$ ls ..
./mycrate                  	  # main crate folder
./mycrate/Cargo.toml       	  # cargo file, lists dependencies etc
./mycrate/src/lib.rs        	  # only one library is allowed per crate
./mycrate/src/mod1.rs      	  # module 1. a library can use multiple modules
./mycrate/src/mod2.rs       	  # module 2 , a second module
./mycrate/src/bin/program1.rs     # source code for an executable
./mycrate/src/bin/program2.rs     # source code for another executable
./mycrate/tests             	  # integration tests (as opposed to unit tests at end of every .rs file)
./mycrate/tests/integration_test.rs  # big test that tests big part of library
./mycrate/tests/test_data_file       # integration tests often use test files
./mycrate/examples          	  # easy to follow examples   
./mycrate/examples/helloworld.rs  # simple example, "use mycrate::*"
./mycrate/examples/zoogbnoz.rs    # more complicated example
./mycrate/benches           	  # benchmarking code and files
./mycrate/benches/benchtest.rs    # program to run benchmarking speed tests
./mycrate/build.rs                # optional program to run before build
./mycrate/target		  # binaries are generated under target
./mycrate/target/debug/program1   # debug build executable
./mycrate/target/release/program1 # release build executable (optimized for speed) 
$ cargo build                     # this builds all files in crate
```


src/lib.rs: (our main library file which the world will use)
```rust
mod mod1;      // we must add each modules in lib.rs with 'mod' before we can use them within each other
use mod1::*;   // import symbols from mod1.rs
pub mod mod2;  // this module is public, anyone using this crate can access its functions too
use mod2::*;   // import symbols from mod2.rs
pub fn mycrate_init() { 
  let x = do_stuff1(9);  // call function from mod1.rs
  mycrate_monster(x);
}
pub fn mycrate_monster(y:u8) -> Monster { // Monster - type from mod1.rs
  let mut a = Monster{ true,y };
  do_stuff2( &mut a );   // call function from mod2.rs
  a
}
pub fn mycrate_calcteeth()-> { 23 }
```

src/mod1.rs: (our 1st module that does stuff)
```rust
pub fn do_stuff1(x:u8)->u8 { x+5 } // public function, available to other modules
pub struct Monster1 {  // public struct, available to other modules
    pub is_happy:bool, // public struct member, available to others
    pub num_teeth:i8,
}
```

src/mod2.rs: 
```rust
use super::*; // use functions from lib.rs
use mod1::*;  // use functions/types from mod1.rs, like Monster struct 
              // note: this only works if "mod mod1" line is in lib.rs
	      // otherwise you get E0432 unresolved import , maybe a missing crate
pub fn do_stuff2( a: &mut Monster ) { 
	a.is_happy = calc_happy();           // call our own private function
	a.num_teeth = mycrate_calcteeth();   // call a function from lib.rs
}
fn calc_happy() -> bool { // private function only available within mod2.rs
  match today { Tuesday => true ,_=>false } }
```

See also
https://doc.rust-lang.org/book/ch07-02-modules-and-use-to-control-scope-and-privacy.html


## ANSI colors

In C, ansi colors for terminal text output are typically done with the escape code
in octal format, such as printf("\033]0;31m test red"); In Rust, you can use
the unicode escape sequence, instead of 033 octal, we can use unicode with 001b hexadecimal:

```rust
fn main() {
    println!("\u{001b}[0;31m{}", "test red");
    println!("\u{001b}[0;32m{}", "test green");
    println!("\u{001b}[0;33m{}", "test orange");
    println!("\u{001b}[0;34m{}", "test blue");
    println!("\u{001b}[0;35m{}", "test magenta");
    println!("\u{001b}[0;36m{}", "test cyan");
    println!("\u{001b}[0;37m{}", "test white");
    println!("\u{001b}[0;91m{}", "test bright red");
    println!("\u{001b}[0;92m{}", "test bright green");
    println!("\u{001b}[0;93m{}", "test yellow");
    println!("\u{001b}[0;94m{}", "test bright blue");
    println!("\u{001b}[0;95m{}", "test bright magenta");
    println!("\u{001b}[0;96m{}", "test bright cyan");
    println!("\u{001b}[0;97m{}", "test bright white");
    println!("\u{001b}[0m{}", "restored to default");
}
```

There are also several crates that do this. Search the web for additional ANSI color features.
