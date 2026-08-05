## DAY-4 (Advanced Rust programming Practices )

## Imp Links to play ( https://play.rust-lang.org/) 
## Docs (https://doc.rust-lang.org/book/)


## Object-Oriented alternatives

Inheritance and Polymorphism - Rust doesn't have the object-oriented style of these things. Alternatives:

- "composition" (struct within struct),
- Traits (sort of like Interfaces), 
- Enums (which can do much more than C enums, they are more like Tagged Unions aka Algebraic Data Types aka Sum Types)
- "NewType" pattern, aka Tuple Structs where for example you say struct MyWrapper(u32) to wrap u32 and then impl your own methods on MyWrapper to mimic u32, along with derive_more and implement deref (u32 could be any other struct or type from another external crate).
- todo - describe Dynamic Trait Objects 
- todo - describe Generics

## Files

```rust
use std::fs::File;         // File handling module
use std::io::{Write,Read}  // read and write capabilities for File
use std::fs::OpenOptions;  // specialized version of File

// read file, non-crashing example
let filename = "test.txt";
match File::open(filename) {
	Err(why) => println!("failed to open file '{}': {}", filename, why),
        Ok(mut f) => {
        	println!("ok, opened {}",filename);
                let mut data = String::new();
                match f.read_to_string( &mut data ) {
                	Err(why) => println!("failed to read {}, {}",filename,why),
                        Ok(n) => println!("read {} bytes",n),
                };
	},
}

// crash-on-error examples ( unwrap() calls panic! on error, crashes the program )
let mut s = String::new();
File::open("test.txt").unwrap().read_to_string(&mut s).unwrap();    // read into s
File::create("output.txt").unwrap().write(s.to_bytes()).unwrap();   // write string
f = File::create("output.txt").unwrap();                            // create if doesnt exist
f = OpenOptions::new().write(true).truncate(true).create(true).open("out.txt").unwrap();       // create if doesnt exist
f = OpenOptions::new().append(true).open("out.txt").unwrap();       // append
f = OpenOptions::new().append(true).create(true).open("out.txt").unwrap(); // append + create
write!(f,"{}",s).unwrap(); // write formatted string, given file object f

let mut v = vec![];                                                 // read binary data, vec of u8
f = File::open("test.bin").unwrap().read_to_end(&mut v);            // without using buffering

use std::io::{self,BufRead};  // read from stdin aka standard input
let line = io::stdin().lock().lines().next().unwrap().unwrap();

let x = include!("datafile.txt"); // include external data file, example: vec![0,0,0];

// handle errors inside a io::Result-returning function with the question mark ?
fn readfunc()  -> std::io::Result<()> {
  let mut tmpbuf = vec![0u8;4096];
  let mut f = File::open("somefile.bin")?;   // instead of unwrap, just return error
  let count = f.read( &mut tmpbuf )?;        // instead of match, just return error
  Ok(())                                     // we got this far, so there's no error
}

// read binary data in a loop to a buffer
fn readfunc2()  -> std::io::Result<()> {
  let mut tmpbuf = vec![0u8;4096];
  let mut f = File::open("somefile.bin")?;   
  loop {
    let numbytes_read = f.read( &mut tmpbuf )?; 
    println!("read data {}",numbytes_read);
    if numbytes_read==0 { break Ok(()); }
  } 
}

// same loop as above, but with While Let
fn readfunc3()  -> std::io::Result<()> {
  let mut tmpbuf = vec![0u8;4096];
  let mut f = File::open("somefile.bin")?;   
  while let numbytes_read = f.read( &mut tmpbuf )? {
    println!("read # bytes: {}",numbytes_read);
    if numbytes_read==0 { break; }
  };
  Ok(())
}


// Passing File to function... 
pub fn zipread( mut rd:&File )  { let x = rd.read(&[buf])?; println!("{}",x); }
// .. or... pass any struct with read trait 
pub fn zipread( mut rd:impl Read )  { let x = rd.read(&[buf])?; println!("{}",x); }

// File position
// note this here is a u64 so y'all with a exabyte of data gone hafta finda workaroun'
// also for some reason f has to be mutable if you do this. 
use std::io::Seek;
println!("{:?}",f.stream_position()); 

// there is no 'close', files close automatically at the end of scope ( "}" )

```

### Filesystem

```rust
use std::fs;
match std::fs::copy( "file.txt", "newfile.txt") {
	Ok => println!("copy successfull"),
	Err(why) => println!("copy failed"),
}
std::fs::remove_file("newfile.txt").unwrap(); // panic if failure

```

## System, arguments to main(), environment variables

```rust
std::env::args().for_each(|x| print!("{} ",x)); // main arguments as iterator, print each one 
for arg in std::env::args().collect::<Vec<String>>() { print!("{} ",arg); }; // same, as vector
if std::env::args().any(|x| x=="--help") {help()};            // if called with --help, run help()
let progname = std::env::args().nth(0)  // first argument. but this wont compile! its an Option()
let progname = std::env::args().nth(0).unwrap_or("yr system is very broken".to_string()); 
// on most systems, first argument = program name. but args is not guaranteed to exist, its an iterator
// that could be empty. so we can use unwrap_or() to deal with the Option if no arguments are there
```

### clap - command line argument parser

    # if you want your program 'mycompiler' to have args like this:
    mycompiler input.txt -o binary.exe -optimize -I./include -I/usr/include
    # install clap crate: 
    cargo add clap --features derive   # bash command to add clap dependency
 
```rust
mycompiler.rs:
extern crate clap;
// clap will parse the struct and automatically handle errors,
// detect the argument type based on the variable (Vec,Option,bool)
// with Option for optional args, no Option for Requred args, Vec for
// multiple args, bool for flag style args, etc. clap will also
// automatically provide --verison and --help using /// comments
#[derive(clap::Parser, Debug)]
pub struct Args {
    /// inputfile - filename to compile 
    inputfile: String,   
    
    /// outfile - filename for binary output
    #[arg(short = 'o', long)]
    outfile: Option(String), 
    
    /// paths to search for header files
    #[arg(short = 'I', long)]
    include: Vec<String>,

    /// use the optimizer during compilation
    #[arg(short='O', long)]
    optimize: bool       
}

fn main() {
    let clapargs = Args::parse_from(std::env::args.clone());
    println!("{:?}",clapargs);
    let mut output = "a.out";
    if let Some(outname) = clapargs.outfile.as_deref() { output = outname};
    if let Some(input) = clapargs.inputfile.as_deref() {
	compile(input,output,clapargs.optimize);
    } 
}

```

### Environment variables
```rust
std::env::var("IGNORE_CASE")  // environment variable IGNORE_CASE, returns Result or Err
if let Ok(v) = std::env::var("SHLVL") {println!("{:?}",v);} // print SHLVL if it's there, otherwise ignore
let sl = match std::env::var("SHLVL") { Ok(v)=>v,Err(e)=>"not set".to_string() }; // set s1 to SHLVL or "not set" if unset
println!("{:?}",std::env::vars().collect::<Vec<(String,String)>>()); // print all environment variables
for v in std::env::vars() {println!("{:?}",v);}  // print env vars line by line
```


## Reflection

For enums:
```
use strum::IntoEnumIterator;  
#[derive(strum_macros::EnumIter)]
enum Attachment { Avoidant, Anxious, Secure };
Attachment::iter().for_each(|d|print!("{d:?},")); // Avoidant, Anxious, Secure
```

For others: todo

## Traits

todo. Traits are like 'interfaces' in other languages.
You can see examples above where a Struct will Derive the Debug trait
so that gives it certain abilities. Traits are used extensively
when dealing with iterators, see below. 

## Iterators, functional style programming

```rust
let v = vec![3,4,5];
let it = v.iter(); // iterator as object
println!("{:?} {:?} {:?}",it.next(),it.next(),it.next()); // consumed
println!("{:?}",it.next()); // None

let v = vec![3];
let mut i = v.iter().peekable();
println!("{:?}",i.peek());  // Some(3)
println!("{:?}",i.peek());  // Some(3)
println!("{:?}",i.next());  // 3
println!("{:?}",i.next());  // None

let mut i = v.iter();
print!("{:?}",i.collect::<Vec<_>>(); // iterator back to vector 

// basic iteration
for i in v.iter() {print!("{i} ");} // 3 4 5 
vec![3,4,5].iter().for_each(|x| print!("{x} ")); // 3 4 5
let biggest = v.iter().max();        // 5
let hasle2 = v.iter().any(|x| x<=4); // true if any element is less than or equal to 2
let biggest = v[0..1].iter().max();  // will return 4, not 5, b/c we took a slice of the vector
for i in vec![3,4,5].iter().take(2) {print("{i}");} // 3, 4
for (i,n) in vec![3,4,5].iter().enumerate() {print!("{i}:{n} ");} // 0:3 1:4 2:5
vec![3,4,5,3].iter().find(|&&x| x == 3) // Some(&3), first three
vec![3,4,5].iter().position(|&&x| x == 5) // 2 // kind of like indexOf() in other langs

// combine multiple iterators
(3..=5).chain(9..=11).for_each(|i|print!("{i:?} ")); // 3 4 5 9 10 11
(3..=5).zip(9..=11).for_each(|i|print!("{i:?} ")); // (3, 9) (4, 10) (5, 11) 

// skipping elements
for i in v.iter().step_by(2) {print!("{i} ");} // 3 5 vec![
for i in vec![3,4,5].iter().skip(1) {print("{i}");} // 4, 5
print!("{:?}",vec![3,4,5].into_iter().map(|x| 2 * x).collect::<Vec<u8>>()); // 6 8 10
for i in vec![3,4,5].iter().filter(|x| x<=4) {print!("{i}");} // 3 4
for i in vec![Some(3),None,Some(4)].iter().fuse() {print!("{i}");} // Some(3), None
vec![4,5,3,3].iter().skip_while(|x| **x>=4).for_each(|x|print!("{},",x));  // 3,3, 
for i in vec![3,4,5,3].iter().skip_while(|x| **x>=4) {print!("{i},");} // 3,4,5,3
for i in vec![4,5,3,3,9,6].iter().take_while(|x| **x>=4) {print!("{i},");} // 4,5
for i in vec![3,4,5,3].iter().take_while(|x| **x>=4) {print!("{i},");} //   (nothing) 
vec![3,4,5].iter().cycle().take(6).for_each(|x|print!("{},",x)); // 3,4,5,3,4,5 cycle allows wraparound

// modify elements
v.iter_mut().for_each(|v|*v=*v+5);

// mathematical operations
for i in vec![3,4,5].iter().scan(0,|a,&x| {*a=*a+x;Some(*a)}) {print!("{i}")} // 3,7,12
print!("{}",vec![3,4,5].iter().fold(0, |a, x| a + x)); // 12
vec![3,4,5].iter().sum() // 3+4+5, 12
vec![3,4,5].iter().product() // 12*5, 60
(1..4).reduce(|x,y| x*y).unwrap_or(0); // 6   // 6 is 1*2*3
println!("{}",vec![1.,2.,3.].iter().cloned().fold(std::f64::MIN, f64::max)); // max, 3
println!("{}",vec![1.,2.,3.].iter().cloned().fold(std::f64::MAX, f64::min)); // min, 1
print!("{:?}",vec![['a','b','c'], ['d','e','f']].iter().flatten().collect::<String>() ); // "abcdef"
print!("{:?}",vec![vec![3,4],vec![5,1]].iter().flatten().collect::<Vec<_>>()); // 3,4,5,1
let (a,b): (Vec<_>, Vec<_>) = vec![3,4,5].into_iter().partition(|x| x>&4);
println!("{:?} {:?}",a,b); // [5] [3,4]
vec![3,4,5].iter().all(|x| x>2) // true
vec![3,4,5].iter().all(|x| x>4) // false
vec![3,4,5].iter().any(|x| x<4) // true
vec![3,4,5].iter().any(|x| x<2) // false
// comparisons: cmp, le, eq, ne, lt, etc
print!("{}",vec![3,4,5].iter().eq(vec![1,3,4,5].iter().skip(1))); // true

// misc
permutations(n) // n-based permutations of each element of the iterator
unique() // remove duplicates from iterators
cloned() // clones each element
unzip() // backwards of zip()
rev() // reverse iterator
rposition() // combine reverse and position
max_by_key() // max using single func on each item
max_by()    // max using compare closure |x,y| f(x,y)
v.min_by(|a,b| a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal)); //min val,float
find_map() // combine find and map 
flat_map() // combine flatten and map 
filter_map() // combine filter and map

into_iter() // can help you collect
let (v,u)=(vec![1,2,3],vec![4,5,6]);
print!("{:?}", v.into_iter().zip(u.into_iter()).collect<Vec<(i8,i8)>>()); // E0277
// value of type `Vec<(i8, i8)>` cannot be built from `std::iter::Iterator<Item=(&{integer}...
print!("{:?}", v.into_iter().zip(u.into_iter()).collect<Vec<(i8,i8)>>()); // [(1, 4), (2, 5), (3, 6)]
```



Itertools library: more adapters
```rust
    use itertools::Itertools;
    // these are periodically integrated into Rust core, and/or may change
    
    // sort related
    vec![13,1,12].into_iter().sorted(); // [1,12,13]
    vec![(3,13),(4,1),(5,12)].into_iter().sorted_by(|x,y| Ord::cmp(&x.1,&y.1)); // [(4,1),(5,12),(3,13)]            
    "Mississippi".chars().dedup().collect::<String>(); // Misisipi
    "agcatcagcta".chars().unique().collect::<String>(); // agct

    // mingling single items into others
    "四四".chars().join("十")); // 四十四
    (1..4).interleave(6..9).collect_vec(); // 1,6,2,7,3,8
    " 十是十 ".chars().intersperse('四').collect::<String>(); // "四十四是四十四"
    "愛".chars().pad_using(4,|_| '.').collect::<String>(); // "愛..."

    // multiple iterators
    "az".chars().merge("lm".chars()).collect::<String>(); // "almz"
    "za".chars().merge_by("ml".chars(),|x,y| x>y ).collect::<String>(); // "zmla"
    vec!["az".chars(),"lm".chars()].into_iter().kmerge().collect::<String>(); //"almz"
    for c in &vec![1,2,3,4,5].into_iter().chunks(2) {  // chunk - split into new iterator groups of 2
      for x in c { print!("{:?}",x); } print!(","); } // each group is 2 items. output: 12,34,5,

    // mathematical operations
    "ab".chars().cartesian_product("cd".chars()).collect_vec() ; // [(a,b),(a,d),(b,c),(b,d)]
    (1..=3).combinations(2).collect_vec(); // [[1,2], [1,3], [2,3]]
    "Go raibh maith agat".chars().positions(|c| c=='a').collect_vec();// [4, 10, 15, 17]
    ("四十四是四十四".chars().all_equal(), "謝謝".chars().all_equal()); // (false, true)
    [1,5,10].iter().minmax().into_option().unwrap() // (1,10) // faster than min() then max()
    (0..3).zip_eq("abc".chars()).collect_vec(); // [(0,'a'), (1,'b'), (2,'c')]

    // modifying and removing items
    "bananas".chars().coalesce(|x,y| if x=='a'{Ok('z')}else{Err((x,y))}).collect::<String>(); 
    // result = bzzz - coalesce will replace a pair of items with a new single item, if the item matches
    vec![1,2,3].into_iter().update(|n| *n *= *n).collect_vec(); // [1,4,9]
    let mut s=['.';3];s.iter_mut().set_from("abcdefgh".chars()); s.iter().collect::<String>(); // "abc"
```

## Implementing your own iterator for your own struct

If you want to be able to use all the cool adapters like filter(), map(), fold(), etc, 
on your own cusom data structure you can implement the Iterator trait for your 
struct. Basically you need to create another struct called a "XIterator" that implements 
a 'next()' method. Then you create a method in your own struct called 'iter()' that returns 
a brand new XIterator struct.

Here is a super simple example, all it does is iterate over a data inside of custom 
data struct Mine. The data is stored in a vector so it's pretty straightforward to access it.


```rust

#[derive(Debug)]
struct Mine{
    v:Vec<u8>
}

impl Mine{
    fn iter(self:&Mine) -> MineIterator {
        MineIterator {
            m:&self, count:0,
        }
    }
}

struct MineIterator<'a> {
    m: &'a Mine,
    count: usize,
}

impl<'a> Iterator for MineIterator<'a> {
    type Item = &'a u8;
    fn next(&mut self) -> Option<Self::Item> {
        self.count+=1;
        if self.count<=self.m.v.len() {
		Some(&self.m.v[self.count-1])
	} else {
		None
	}
    }
}

fn main() {
    let m=Mine{v:vec![3,4,5]};
    m.iter().for_each(|i| print!("{:?} ",i));println!(""); // 3 4 5
    m.iter().filter(|x|x>3).fold(0,|a,x| a+x); // 9 
}

```

If you want to create a Mutable Iterator, you will (as of writing, 2018)
have to use an unsafe code with a raw pointer. This is alot like what the
standard library does. If you are not extremely careful, your program may
exhibit bizarre behavior and have security bugs, which defeats the purpose
of using Rust in the first place. 

* https://gitlab.com/Boiethios/blog/snippets/1742789
* https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#dereferencing-a-raw-pointer

```rust


impl Mine{
    fn iter_mut(self:&mut Mine) -> MineIterMut {
            MineIterMut {
            m:self, count:0,//_marker:std::marker::PhantomData,
        }
    }
}

pub struct MineIterMut<'a> {
    m: &'a mut Mine,
    count: usize,
}

impl<'a> Iterator for MineIterMut<'a> {
    type Item = &'a mut u8;
    fn next(&mut self) -> Option<&'a mut u8> {
            if self.count == self.m.v.len() {
                None
            } else {
                let ptr: *mut u8 = &mut self.m.v[self.count];	    
                self.count += 1;
		unsafe{ Some(&mut *ptr) }
            }
    }
}

fn main() {
    let mut m=Mine{v:vec![3,4,5]};
    m.iter().for_each(|i| print!("{:?} ",i));println!(""); // 3 4 5
    m.iter_mut().for_each(|i| *i += 1);
    m.iter().for_each(|i| print!("{:?} ",i));println!(""); // 4 5 6
}
```

### make your own Iterator with impl Iterator 

"impl Iterator" can help build iterators without creating your own custom struct.
For example you can return an Iterator from a function that modifies your data. Examples
follows for Itertools iproduct! macro and cartesian_product adapter.

```rust
fn f2() -> impl Iterator<Item = (u8,u8)> { iproduct!(0..2,0..3) }
fn f3(mut v:Vec<u8>) -> impl Iterator<Item = (u8,u8)> {
    v.push(1); v.into_iter().cartesian_product(0..3) }

for vertex in f2() {println!("{:?}", vertex);} // (0,0) (0,1) (0,2) (1,0) (1,1) (1,2)
for vertex in f3(vec![0]) {println!("{:?}", vertex);} // (0,0) (0,1) (0,2) (1,0) (1,1) (1,2)
```
