// you can make things unprintable (sensitive secrets?) with structs like
#[allow(dead_code)]
struct UnPrintable(i32);

// and do the opposite through a derive attribute :O
#[derive(Debug)]
struct Structure(i32);

// what if we print a nested structure, hmmm?
#[derive(Debug)]
struct Deep(Structure);

fn main() {
  println!("{} days", 31);

  // cool, positional arguments!
  println!("{0}, this is {1}, {1} this is {0}", "alice", "bob");

  // special formatting is available with keywords after `:`
  println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

  // you can right align text with a specified width. also, check out this dope named variable thing
  println!("{number:>width$}", number=1, width=6);
  //just checking if weird names work.
  println!("{diggy:>nonsense$}", diggy=1, nonsense=6); // it do!

  // formatting numbers!
  println!("{number:>0width$}", number=1, width=6);
  //hol up could this be an arbitrary fill?
  // println!("{number:>6width$}", number=1, width=3); //nope

  // detects bad argument number.
  // println!("My name is {0}, {1} {0}", "Bond");
  // FIXME ^ Add the missing argument: "James"

  // Activity for printing pi with only three digits!
  let pi = 3.141592;
  
  println!("Hello! Pi with 3 decimals is {0:.3}", pi);


  ////////// DEBUG /////////// https://doc.rust-lang.org/stable/rust-by-example/hello/print/print_debug.html
  println!("Now {:?} will print!", Structure(3));

  // there's no "nice formatting" here. For example, you cannot print what's deep in this structure without manually de-structuring it.
  println!("Now {:?} will print!", Deep(Structure(7)));
  
  //pretty print!
  println!("now {:#?} will print!", Deep(Structure(7)))


}