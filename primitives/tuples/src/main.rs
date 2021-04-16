use std::fmt;

fn reverse (pair: (i32, bool)) -> (bool, i32) {
  // destructure into not-a-tuple then make it a tuple again, woah!
  let (integer, boolean) = pair;

  (boolean, integer)
}

fn transpose (matrix: Matrix) -> Matrix {
  // shift em around
  Matrix(matrix.0,matrix.2,matrix.1,matrix.3)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({} {})\n({} {})", self.0, self.1, self.2, self.3)
  }
}

fn main() {
  // a tuple with a bunch of different types
  let long_tuple = (1u8, 2u16, 3u32, 4u64,
                    -1i8, -2i16, -3i32, -4i64,
                    0.1f32, 0.2f64,
                    'a', true);
  
  // read the parts of the tuples dawg
  println!("long tuple first: {}, second {}", long_tuple.0, long_tuple.1);

  // tuples can be inside tuples :O
  let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
  
  // tuples are printable
  println!("tuple of tuples {:?}", tuple_of_tuples);

  // long tuples can't be printed doe.
  // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
  // println!("too long tuple: {:?}", too_long_tuple);

  let pair = (1, true);
  println!("pair is {:?} and reversed is {:?}", pair, reverse(pair));

  let matrix = Matrix(1.1,1.2,2.1,2.2);
  println!("{:?}", matrix);
  println!("{}", matrix);

  println!("transposed like:\n{}", transpose(matrix));
}