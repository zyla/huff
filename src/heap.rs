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

fn pop<T>(heap: &mut Vec<T>) -> Option<T> where T: Ord {
  if heap.is_empty() {
    return None;
  }

  let item = heap.swap_remove(0);

  let mut i = 0;
  while left_child(i) < heap.len() {
    let next_index =
      if right_child(i) < heap.len() && heap[right_child(i)] <= heap[left_child(i)] {
        right_child(i)
      } else {
        left_child(i)
      };

    // Not strictly necessary, and may slow us down (less swaps, but more comparisons!)
    if heap[i] <= heap[next_index] {
      break;
    }
    heap.swap(i, next_index);
    i = next_index;
  }

  Some(item)
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

    fn pop_all<T>(heap: &mut Vec<T>) -> Vec<T> where T: Ord {
        let mut result = Vec::with_capacity(heap.len());
        while let Some(item) = pop(heap) {
            result.push(item);
        }
        result
    }

    fn heapsort<T>(items: Vec<T>) -> Vec<T> where T: Ord {
        pop_all(&mut make_heap(items))
    }

    #[test]
    fn make_heap_examples() {
        assert_eq!(make_heap(vec![1]), vec![1]);
        assert_eq!(make_heap(vec![1, 0]), vec![0, 1]);
        assert_eq!(make_heap(vec![0, 0]), vec![0, 0]);
        assert_eq!(make_heap(vec![1, 9, 8, 2, 7, 6, 3, 4, 5, 0]),
                             vec![0, 1, 3, 4, 2, 8, 6, 9, 5, 7]);
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

        fn qc_heapsort(items: Vec<u8>) -> TestResult {
            let sorted_items = heapsort(items.clone());
            if sorted_items != sorted(&items) {
                return TestResult::error(format!("Not sorted correctly.\nitems:  {:?}\nsorted: {:?}", items, sorted_items));
            }
            return TestResult::passed();
        }

        fn qc_pop_preserves_heap_invariant(items: Vec<u8>) -> TestResult {
            let mut heap = make_heap(items.clone());
            pop(&mut heap);
            if let Err(info) = invariant_holds(&heap) {
                return TestResult::error(format!("Heap invariant not satisfied at {:?}.\nitems: {:?}\nheap:  {:?}", info, items, heap));
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
