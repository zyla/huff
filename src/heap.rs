pub fn insert<T>(heap: &mut Vec<T>, x: T) where T: Ord {
  let mut i = heap.len();
  heap.push(x);
  while i > 0 {
    // To maintain heap invariant, enforce heap[parent] <= heap[i].
    if !(heap[parent(i)] <= heap[i]) {
      heap.swap(i, parent(i));
    }
    i = parent(i);
  }
}

fn parent(i: usize) -> usize { (i - 1) / 2 }
fn left_child(i: usize) -> usize { i * 2 + 1 }
fn right_child(i: usize) -> usize { i * 2 + 2 }

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck::*;

    fn invariant_holds<T>(heap: &Vec<T>) -> Result<(), (usize, &T, usize, &T)> where T: Ord {
      for i in 1..heap.len() {
        if !(heap[parent(i)] <= heap[i]) {
          return Err((parent(i), &heap[parent(i)], i, &heap[i]));
        }
      }
      Ok(())
    }

    fn insert_all<T>(heap: &mut Vec<T>, items: Vec<T>) where T: Ord {
        for item in items.into_iter() {
            insert(heap, item);
        }
    }

    fn make_heap<T>(items: Vec<T>) -> Vec<T> where T: Ord {
        let mut heap = Vec::with_capacity(items.len());
        insert_all(&mut heap, items);
        heap
    }

    #[test]
    fn it_works() {
        assert_eq!(make_heap(vec![1]), vec![1]);
        assert_eq!(make_heap(vec![1, 0]), vec![0, 1]);
        assert_eq!(make_heap(vec![0, 0]), vec![0, 0]);
    }

    fn sorted<T>(xs: &[T]) -> Vec<T> where T: Clone + Ord {
        let mut sorted = xs.to_vec();
        sorted.sort();
        sorted
    }

    quickcheck! {
        fn qc_make_heap(items: Vec<u8>) -> TestResult {
            let heap = make_heap(items.clone());
            if let Err(info) = invariant_holds(&heap) {
                return TestResult::error(format!("Heap invariant not satisfied at {:?}.\nitems: {:?}\nheap:  {:?}", info, items, heap));
            }
            if sorted(&heap) != sorted(&items) {
                return TestResult::error(format!("Item set not preserved.\nitems: {:?}\nheap:  {:?}", items, heap));
            }
            return TestResult::passed();
        }

        fn qc_left_child_parent_id(i: usize) -> bool {
            parent(left_child(i)) == i
        }

        fn qc_right_child_parent_id(i: usize) -> bool {
            parent(right_child(i)) == i
        }
    }
}
