use std::marker::PhantomData;

trait Nat {}

#[derive(Debug)]
struct Zero;
impl Nat for Zero {}

#[derive(Debug)]
struct Suc<N: Nat> {
  _pred: PhantomData<N>
}
impl<N: Nat> Nat for Suc<N> {}

#[derive(Debug)]
struct Array<T, N: Nat> {
  list: Vec<T>,
  _size: PhantomData<N>
}

impl<T> Array<T, Zero> {
  fn new() -> Array<T, Zero> {
    Array { list: vec![], _size: PhantomData }
  }
}

impl<T, N: Nat> Array<T, N> {
  fn push(mut self, element: T) -> Array<T, Suc<N>> {
    self.list.push(element);
    Array {
      list: self.list,
      _size: PhantomData
    }
  }
}

trait PopWithoutFear<T, M: Nat, N: Nat> {
  fn pop_without_fear(self) -> Array<T, N>;
}

impl<T, N: Nat> PopWithoutFear<T, Suc<N>, N> for Array<T, Suc<N>> {
  fn pop_without_fear(mut self) -> Array<T, N> {
    self.list.pop().unwrap();
    Array {
      list: self.list,
      _size: PhantomData
    }
  }
}

/*
Probably useless, because in order to use it with loop constructs you need to implement
some sort of `runtime polymorphism` which will defeat the whole purpose of pop error
detection at compile time. Or maybe I don't know how to use it yet.
*/
pub fn run() {
  let list = Array::new();
  let list = list.push(10);
  let list = list.push(10);
  let list = list.pop_without_fear();

  println!("{:?}", list);
}