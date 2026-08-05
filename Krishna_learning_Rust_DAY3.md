## DAY-3 (Intermediate Rust programming Practices )

## Imp Links to play ( https://play.rust-lang.org/) 
## Docs (https://doc.rust-lang.org/book/)


## If, conditionals, patterns, match, control flow

```rust

let oz = ounces_in_recipe();

// normal if else, like C, Pascal, etc
if oz == 1 {
	print!("1 ounce")
} else if oz == 8 {
	print!("1 cup")
} else if oz == 16 {
	print!("1 pint")
} else if oz == 128 {
	print!("1 gallon")
} else {
	print!("non-unit # of ounces");
}

match oz { // match, like case, works on literals not variables
	1 => print!("1 ounce"),  // "=>" signifies a branch or leg of the match
	8 => print!("1 cup"),    // have as many=> legs as you want
	16 => print!("1 pint"),    // end each leg with a comma ,
	128 => print!("1 gallon"),
	_ => print!("non-unit # of ounces") // underscore _ will match anything not previously matched
}

 // match can work on enums too, if you want to limit what values a variable can be assigned
pub enum UnitVolume{Cup,Pint,Gallon}

let ozv = UnitVolume::Cup;

match ozv {           
	UnitVolume::Cup => print!("cup"),    // "=>" signifies a branch or leg of the match
	UnitVolume::Pint => print!("pint"),    // have as many=> legs as you want
	UnitVolume::Gallon => print!("gallon"),    // end each leg with a comma ,
        // don't need underscore match-arm with Enums, it knows its complete
}

let x = [1i8,2i8];                    // match patterns can be structs, enums, arrays, etc
let y = match x {                     // match can 'return a result' to y
        [1,0] => "1,0",               // match a specific array
        [2,0]|[4,0] => "2,0 or 4,0",  // |, binary or, can be used in patterns
        [_,2] => "ends with 2",       // [_,2] will match [0,2] [1,2] [2,2], [3,2], etc
        _ => "other"
};
println!("{}",y);                     // "ends with 2"


let m = 3;	 		     // match patterns can only be constant, but you can
let n = 4;                           // do similar things with "match guard", an if statement 
let y = match (n,m) {                // which is inside of a => arm. First, we match tuple (n,m)
        (0,0) => "ok",               //  this leg matches any tuple with first element 0, return ok
        (3,_) if m%5==0 => "ok",     //  this leg matches when first element=3, and second divisible by 5
	(_,_) if m%3==0 => "ok",     //  this leg matches any tuple where the second element is divisble by 3
        _ => "stop",                 //  this leg matches anything else. 
};

let hour = get_24hr_time();          // patterns can be integer ranges (x..=y)
ampm = match hour {                  // however it has to be inclusive 
	0..=11 => "am"               // 1..11 is not ok, 1..=11 is ok.
	12..=23 => "pm"          
	_=> "unknown"            
};

let x = 5i32;                          // we can use the match value in match arms,
let y = match x-1 {                    // this is also called "binding with @", like so:
	0..=9 => 7,                    // since x is 5, x-1 is 4 and 4 is in 0..=9, so y=7 
	m @ 10..=19 => m*2,            // if x-1 was 14, m becomes 14, so y would be 28
	_=> -1,                        // if x-1 was >=20, y would become -1
};

let mut v = vec![0;4096];                // match also works with Result<>
match File::open("/dev/random") {     
	Ok(f)=>f.read(&v),
	Err(why)=>println!("file open failed, {}",why),
}

let v = vec![1,2,3];			// match works with Option too
let n = 1;
match v.get(n) {
	Some(x) => println!("nth item of v is {}",x),
	None => println!("v has no nth item for n={}",n),
};

```

See also: While let, if let. 

## Ownership, Borrowing, References, Lifetimes

Resources have exactly one owner. They can be 'moved' from one owner to another.
 
```rust
// stack memory, no moves, only copies
let a = 5;
let b = a;  // ok
let c = a;  // ok

// heap memory
let a = String::new(); 
let b = a;  // 'move' of ownership from a to b
let c = a;  // error. cannot "move" a again, b already owns it //need to clone() it 

// heap memory + function call
let a = String::new();
let b = a;  // 'move' of ownership from a to b
fn f(t:String) { } // function takes ownership of the variable passed as t
f(a); // error. f(a) would move a into f(), but a was already moved into b
```

Borrowing is an alternative to moving. It is done with References & (memory addresses)

```rust
// heap memory, using borrows and references instead of moves
let a = String::from("☪☯ॐ✡γ⚕✝");
let b = &a;  // this is borrowing, not moving, a to b
let c = &a;  // it is OK to have more than one borrower of non-mutable variables
println!("{}",a);    // ☪☯ॐ✡γ⚕✝
println!("{:p}",&a); // 0x7ffcffb6b278
println!("{:p}",b);  // 0x7ffcffb6b278  // b and c hold the address of a
println!("{:p}",c);  // 0x7ffcffb6b278
println!("{:p}",&b); // 0x7ffcffb6b290  // b and c are distinct variables
println!("{:p}",&c); // 0x7ffcffb6b298  // with their own addresses
```

However borrowing has special rules regarding mutability. 

In a block, only one of these statements can be true for a given resource R
* The program has one or more immutable references to R
* The program has exactly one mutable reference to R 

```rust

// example error[E0502]: cannot borrow `a` as mutable because `a` is also borrowed as immutable
let mut a = 5;
let b = &a;
let c = &mut a;

// example error[E0499]: cannot borrow `a` as mutable more than once at a time
let mut a = 5;
let b = &mut a;
let c = &mut a;

```
Another way to think about it is Shared vs Exclusive
* A plain old & borrow is a Shared Reference, since multiple people can share at once
* A &mut reference is an Exclusive reference, only one can have it.  

Lifetime in Rust: Resources are destroyed, (their heap memory is freed), at the end of a 'scope'. Their owners are also destroyed. That is the point of ownership - so that resources won't be accessed after they are destroyed, which is the source of a huge number of errors in C programs.

Borrowed resources are not destroyed when the borrowed reference itself goes out of scope. However the borrow cannot "outlive" the destruction of the original resource nor it's owner.

Take, for example, the case where we borrow a variable via ```&```. The borrow has a lifetime that is determined by where it is declared. As a result, the borrow is valid as long as it ends before the lender is destroyed. However, the scope of the borrow is determined by where the reference is used.

```rust 
fn main() { // main block starts 
    let i = 3; // Lifetime for `i` starts. ────────────────┐
    //                                                     │
    { // new block starts                                  │
        let borrow1 = &i; // `borrow1` lifetime starts. ──┐│
        //                                                ││
        println!("borrow1: {}", borrow1); //              ││
    } // block for `borrow1 ends. ─────────-──────────────┘│
    //                                                     │
    //                                                     │
    { // new block starts                                  │
        let borrow2 = &i; // `borrow2` lifetime starts. ──┐│
        //                                                ││
        println!("borrow2: {}", borrow2); //              ││
    } // block for `borrow2` ends. ───────────────────────┘│
    //                                                     │
}   // main block ends, so does Lifetime ends. ────────────┘

```


## Structs

```rust
struct Wheel{ r:i8, s:i8};  // basic struct, like C, pascal, etc. r = radius, s = number of spokes 
struct badWheel{ mut r: i8, mut h: i8, }; // error, mut keyword doesnt work inside struct
let w = Wheel{r:5,s:7};   // new wheel, radius 5, spokes 7, immutable binding
w.r = 6; // error, cannot mutably borrow field of immutable binding
let mut mw = Wheel{r:5,s:7}; //  new mutable wheel. fields inherit mutability of struct;
mw.r = 6;  // ok

impl Wheel {              // impl -> implement methods for struct, kinda like a C++ class
        fn new(r: isize) -> Wheel { Wheel { r:r, s:4 } }  // our own default
        fn dump(    &self) { println!("{} {}",self.r,self.s); }  // immutable self
	fn badgrow(    &self) { self.s += 4; } // error, cannot mutably borrow field of immutable binding
        fn  okgrow(&mut self) { self.s += 4; } // ok, mutable self
};
w.dump();    // ok , w is immutable, self inside dump() is immutable
w.okgrow();  // error, w is immutable, self inside okgrow() is mutable
             //  cannot borrow immutable local variable `w` as mutable
mw.dump();   // ok, mw is mutable, self inside dump is immutable.  
mw.okgrow(); // ok, mw is mutable, self inside grow() is mutable.

#[derive(Default,Copy,Clone,Debug,PartialEq)] // automatic implementations 
struct Moo {x:u8,y:u8,z:u8,}
let m1:Moo=Default::default();            // Default, x=0 y=0 z=0
let m2:Moo=Moo{x:3,..Default::default()}; // Default, x=3 y=0 z=0
let (mut n,mut k)=(M{x:-1,y:-1,z:-1},Default::default());
n=k;                          // Copy
vec![M{x:0,y:1,z:2};42];      // Clone
println!("{:?}",n);           // Debug, formatter
if n==k {println!("hi");};    // PartialEq, operator overloading 

// customized operator overloading
impl PartialEq for Wheel{ fn eq(&self,o:&Wheel)->bool {self.r==o.r&&self.s==o.s} }
if mw == w { print!("equal wheels"); }

// default values for members of struct
#[derive(Debug,Default)]
struct Wheel1{ r:i8, s:i8};
println!("{:?}",Wheel1{..Default::default()});        // Wheel1 { r: 0, s: 0 } 

// custom default values for members of struct
#[derive(Debug)]
struct Wheel2{ r:i8, s:i8};
impl Default for Wheel2 { fn default() -> Wheel2{Wheel2 { r: 8, s: 8 }}  }
println!("{:?}",Wheel2{..Default::default()});        // Wheel2 { r: 8, s: 8 } 
println!("{:?}",Wheel2{r:10,..Default::default()});   // Wheel2 { r: 10, s: 8 }

#[derive(Debug)]                       // Initialize one struct from another
struct Apple {color:(u8,u8,u8),price:f32};  
let a = Apple{color:(100,0,0),price:0.2};
let b = Apple{color:(9,12,38),..a };      // this is called "struct update"

```

Structs with function pointers as members

```
struct ThingDoer {
    name: String,
    do_thing: fn(x:&Vec<u8>) -> u8,
}
fn baseball(v:&Vec<u8>)->u8{ 42 }
fn main (){
    let adder = ThingDoer{name:"addthing".to_string(), do_thing:|n| n.iter().fold(0,|a,b|a+b)};
    let multer = ThingDoer{name:"multhing".to_string(), do_thing:|n| n.iter().fold(1,|a,b|a*b)};
    let nother = ThingDoer{name:"nothing".to_string(), do_thing:|n|baseball(&n)};
    let (f1,f2,f3) = (adder.do_thing,multer.do_thing,nother.do_thing);
    let v = vec![1,2,3,4];
    println!("{} {} {}",f1(&v),f2(&v),f3(&v));
}
```


## Enums - not like C

Enums in Rust do more than Enums in C/C++. They are actually more like 
Tagged Unions or ADT / Algebraic Data Types from Functional programming languages.
Other places call them Sum Types

```rust
enum Fruit { Apple, Banana, Pear }
let x = call_some_function( Fruit::Apple );  
enum Errs { ErrOK = 3, ErrFile = 5, ErrFire = 12 } // custom Discriminats (integers)
enum Planet { Earth = 3, Mars, Jupiter } // mars will be 4, Jupiter 5. 

enum Blorg {     // enums can have different types as members
 Flarg(u8),
 Blarg(u32),
 Norg(String),
 Florg(bool)
}

let x = Blorg::Flarg(1); // enums can be detected with a match
match x {
    Blorg::Flarg(1) => println!("x is a Flarg with value of 1!"),
    Blorg::Flarg(_) => println!("x is a Flarg with non-1 value"),
    Blorg::Norg(_) => println!("x is a Norg"),
    _ => println!("neither Flarg nor Norg"),
}

// Enums can also derive traits
#[derive(Clone,Debug,PartialEq)]  // cannot derive Default on enum
enum ColorMapData {
    OneByteColors(Vec<u8>),
    FourByteColors(Vec<u32>),
}

// Enums can also have methods / functions, using impl
// the key is that inside the function, pattern matching is used
// to determine which variant of the Enum the function receives
impl ColorMapData {
  fn description(&self)->String {
    let (numcolors,numbytes) = match self {
      ColorMapData::OneByteColors(x) => (x.len(),"1"),
      ColorMapData::FourByteColors(x) => (x.len(),"4"),
    };
    format!("ColorMap with {} colors, {} bytes per color",numcolors,numbytes)
}}

// functions can take Enum as arguments
fn examinecm( c:ColorMapData ) { println!("{}",c.description()); }
let ca = ColorMapData::FourByteColors(vec![0xFFAA32FFu32,0x00AA0011,0x0000AA00]);
let cb = ColorMapData::OneByteColors(vec![0,1,3,9,16]);
examinecm(ca);
// ColorMap with 3 colors, 4 bytes per color
examinecm(cb);
// ColorMap with 5 colors, 1 bytes per color
```

Using strum, a reflection package, you can iterate an enum's values

```
$ cargo add strum strum_macros
use strum::IntoEnumIterator;  
#[derive(Debug, strum_macros::EnumIter)]
enum Tofu { Smooth, Firm, ExtraFirm }
Tofu::iter().for_each(|d|print!("{d:?},")); // Smooth,Firm,ExtraFirm
```

## Collections, Key-value pairs, Sets

HashMap, aka associative array / key-value store / map 

```rust
use std::collections::{HashMap};
let mut m = HashMap<char,i32>::new();   
m.insert('a', 1);                   // key is 'a', value is 1
let b = m[&'a'];                    // [] lookup, this crashes at runtime if 'a' is not in map
let c = m.get(&'a').unwrap_or(&-1); // .get(), c == -1 if a is not in map. no crash.
match m.get(&'a') {                 // deal with map get() lookup using Match + Option
    Some(x)=>println!("a found in map, value is {}",x),
    None=>println!("a not found in map"),
}
if let Some(x) = m.get(&'a') {     // deal with map get() lookup using if let + Option
    println!("a found in map, value is {}",x);
}  // if 'a' is not in map, do nothing

*m.get_mut(&'a').unwrap() += 2;  // change a value inside a map

```

Concise initialization with from and from_iter using tuples of (key,value)

```rust
let mut phonecodes = HashMap::from(  // from uses a fixed size array
  [("Afghanistan",93),("American Samoa",1),("Ukraine",380)]   );
let mut squares:HashMap<u32,u32> = HashMap::from_iter(
  (0..24).map(|i| (i, i*i))   );     // from_iter uses an iterator
println!("{:?}",phonecodes); //{"Afghanistan: 93, Ukraine: 380...
println!("{:?}",squares); //{10: 100, 3: 9, 1: 1, 13: 169, ...
```

HashSet

```rust
use std::collections::HashSet;
let mut squares = HashSet::new();
squares.insert(0);
squares.insert(4);
let b = squares.contains(&4);   // b==true
```

## Macros

Does not act like a preprocessor. It replaces items in the abstract syntax tree

```rust
macro_rules! hello {
    ($a:ident) => ($a = 5)
}
let mut a = 0;
println!("{}",a); // 0
hello!(a);
println!("{}",a); // 5
    
macro_rules! bellana {
    ($a:expr,$b:expr) => ($a + $b)
}
println!("{:?}",bellana!(5,(9*2))); // 23

macro_rules! maximum {   
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => ( std::cmp::max($x, maximum!($($y),+)) )
}
maximum!(1,2,3,4);   // 4

macro_rules! dlog {
    ($loglevel:expr, $($s:expr),*) => (
        if DLOG_LEVEL>=$loglevel { println!($($s),+); }
    )
}
let DLOG_LEVEL=5;
dlog!(4,"program is running, dlog:{}",DLOG_LEVEL);  // "program is running, dlog:5"

/*
designators: 
block   // rust block, like {}.        expr    // expressions
ident   // variable/function names.    item    // 
pat     // pattern.                    path    // rust path
stmt    // statement.                  tt      // token tree
ty      // type.                       vis     // visibility qualifier
*/
```




## Arrays, Slices, Ranges

Arrays

```rust
let arr: [u8; 4] = [1, 2, 3, 4]; // immutable array. cant change size.
let mut arrm: [u8; 4] = [1,2,3,4]; // mutable array. cant change size.
let n = arr.len();     // length of array = 4 items 
let s1 = &arr[0..2];   // slice of underlying array
let n2 = s1.len();     // length of slice = 2 items
println!("{:?}",s1);   // 1 2, contents of slice
let s2 = &arr[1..];    // slice until end
println!("{:?}",s2);   // 2 3 4 contents of slice until end
let sm = &mut arrm[0..2];  // mutable slice
sm[0] = 11;                // change element of mutable slice,
println!("{:?}",sm);       // 11 2 3 4  
                           // underlying array was changed
println!("{:?}",arrm);     // 11 2 3 4 

let z = [1,6,1,8,0,0,3];
println!("{:?}",z[0..4]);   // error - not a slice
^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
println!("{:?}",&z[0..4]);  // OK to take a slice   
// 1,6,1,8

// pass array slice to function
fn dostuff(x:&mut [u8]) {
	x[0] = 5;
	println!("{:?}  {}",x,x.len()); // 5 2 3 4   4
}

fn main() {
	let mut arr: [u8; 4] = [1, 2, 3, 4];
	dostuff( &mut arr );
}

```

Get(), a byte index into a UTF8 String

```rust
let sa = String::from("a"); // get() - byte index slice from a String
let sga = sa.get(0..1); // Some("a"). get returns Option, but not a Option<char> 
let s = String::from("上善若水"); // get() with a UTF8 String where char.len()!=1
let sg0 = s.get(0..0);  // Some("") , the empty string.
let sg1 = s.get(0..1);  // None, because the first byte of 上 is not a utf8 char
let sg2 = s.get(0..2);  // None, same reason
let sg3 = s.get(0..3);  // Some("上"), this works because 上 in utf8 is 3 bytes
let sg4 = s.get(0..4);  // None again, because we now have 上 + 1 other byte
// see 上 at http://www.unicode.org/cgi-bin/GetUnihanData.pl?codepoint=4E0A
```

## Split_At_Mut - Mutability and References into a Vector

For the special case for getting references to items within a vector:

Imagine we have three Wheels (struct W) with radius (W.r) and they are inside
a vector v. We start with wheels of radius 3,4,5 and want to change 
it to be radiuses of 2, 4, 8.

```rust
    #[derive(Debug)]
    struct W{ r:u32 }       // wheel struct
    let mut v = vec![W{r:3},W{r:4},W{r:5}];   // vector of structs
    let wheela = &mut v[0];      // mutable reference to Wheel with radius 3
    let wheelb = &mut v[2];      // compile error! two mutables for one variable!
    //  error[E0499]: cannot borrow `v` as mutable more than once at a time
    wheela.r = 2;  // nope
    wheelb.r = 8;  // nope
```
The borrow checker treats the entire vector as one gigantic variable, so
you can't edit part of it with a mutable reference and then edit another part,
because the parts aren't considered parts. They are considered as if you are
editing the variable v. 

But there is a workaround built into the language. It is called..

split_at_mut

It can create mutable slices, which allow mutable access to the vector.

```rust
     #[derive(Debug)]
    struct W{ r:u32 }       // wheel struct
    let mut v = vec![W{r:3},W{r:4},W{r:5}];   // vector of structs
    let (l, r) = v.split_at_mut(2);  // split after the wheel with r4
    let wheela = &mut l[0];  // first item of left part of split
    let wheelb = &mut r[0];  // first item of right part of split
    wheela.r = 2;       // no problem
    wheelb.r = 8;       // no problem
    println!("{:?} {:?}",l,r); // [W { r: 2 }, W { r: 4 }] [W { r: 8 }]
    println!("{:?}",v); // [W { r: 2 }, W { r: 4 }, W { r: 8 }]
```
