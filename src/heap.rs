use std::cmp::Ordering;
use std::fmt::Debug;
use std::iter::IntoIterator;

struct Heap<T> where T: PartialOrd+Debug{
  heap : Vec<T>
}

impl<T> Heap<T> where T: PartialOrd+Debug{
    pub fn new() -> Self {
        Heap {
            heap: Vec::new(),
        }
    }

    pub fn push(&mut self, v:T){
      self.heap.push(v);
      let mut index = self.heap.len() -1;
      if index == 0{
        return;
      }
      while let Some(order) = self.heap[(index+1)/2-1].partial_cmp(&self.heap[index]){
        if order == Ordering::Greater{
          self.heap.swap(index, (index+1)/2 -1);
          index = (index+1)/2 -1;
          if index!=0{
            continue;
          }
        }
        break;
      }
    }

    pub fn pop(&mut self)->Option<T>{

      match self.heap.get(0){
        None => None,
        _ =>{
          let smallest = self.heap.swap_remove(0);
          let mut index =0;
          loop{
            let left_index = 2 *index+1;
            let right_index = left_index+1;
            match (self.heap.get(index),self.heap.get(left_index),self.heap.get(right_index)){
              (None,_,_)=> break,
              (Some(top),Some(left),Some(right)) if left.partial_cmp(right) == Some(Ordering::Greater) && right.partial_cmp(top) == Some(Ordering::Less)=>{
                self.heap.swap(right_index,index);
                index = right_index;  
              },
              (Some(top),Some(left),_) if left.partial_cmp(top) == Some(Ordering::Less)=>{
                self.heap.swap(left_index, index);
                index = left_index;
              },
              _ =>{
                break;
              }
            }
          }
          Some(smallest)
        }
      }
    }
}

impl <T> IntoIterator for Heap<T> where T: PartialOrd+Debug{
  type Item = T;
  type IntoIter = std::vec::IntoIter<T>;
  fn into_iter(self) -> Self::IntoIter{
    self.heap.into_iter()
  }
}


#[cfg(test)]
pub mod test{
    use crate::heap::Heap;

  #[test]
    fn it_works() {
        let mut heap = Heap::new();
        heap.push(5);
        heap.push(4);
        heap.push(3);
        heap.push(2);
        heap.push(1);
        
        assert_eq!(heap.pop(),Some(1));
        assert_eq!(heap.pop(),Some(2));
        assert_eq!(heap.pop(),Some(3));
        assert_eq!(heap.pop(),Some(4));
        assert_eq!(heap.pop(),Some(5));
        assert_eq!(heap.pop(),None);
    }
}