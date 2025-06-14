use core::fmt;
use std::{fmt::Debug, ops::Index, vec::IntoIter};

/// A sorted list data structure
///
/// # Example
///
/// ```
/// use sortedlist_rs::SortedList;
///
/// let array = vec![90, 19, 25];
/// let mut sorted_list = SortedList::from(array);
///
/// println!("{:?}", sorted_list);
/// // [19, 25, 90]
///
/// sorted_list.insert(100);
/// sorted_list.insert(1);
/// sorted_list.insert(20);
/// println!("{:?}", sorted_list);
/// // [1, 19, 20, 25, 90, 100]
///
/// let x = sorted_list.remove(3);
/// assert_eq!(25, x);
/// // removed the 3-rd smallest (0-indexed) element.
///
/// assert_eq!(&20, sorted_list.kth_smallest(2));
///
/// assert_eq!(20, sorted_list[2]);
///
/// println!("{:?}", sorted_list);
/// // [1, 19, 20, 90, 100]
/// ```
pub struct SortedList<T>
where
    T: Ord,
{
    _lists: Vec<Vec<T>>,
    _index_tree: Vec<usize>,
    _index_tree_offset: usize,
    _load_factor: usize,
    _upper_load_factor: usize,
    _lower_load_factor: usize,
    _len: usize,
}

/// Private method implementations
impl<T> SortedList<T>
where
    T: Ord,
{
    const DEFAULT_INDEX_TREE_OFFSET: usize = 1 << 5;
    const DEFAULT_LOAD_FACTOR: usize = 1_024;
    const DEFAULT_UPPER_LOAD_FACTOR: usize = 2_048;
    const DEFAULT_LOWER_LOAD_FACTOR: usize = 512;

    /// Instantiate an empty SortedList.
    fn _default() -> Self {
        Self {
            _lists: vec![],
            _index_tree: vec![0; 2 * Self::DEFAULT_INDEX_TREE_OFFSET],
            _index_tree_offset: Self::DEFAULT_INDEX_TREE_OFFSET,
            _load_factor: Self::DEFAULT_LOAD_FACTOR,
            _upper_load_factor: Self::DEFAULT_UPPER_LOAD_FACTOR,
            _lower_load_factor: Self::DEFAULT_LOWER_LOAD_FACTOR,
            _len: 0,
        }
    }

    /// Collapse self._lists\[i]. self._lists\[i].len() must be > 1.
    fn _collapse(&mut self, i: usize) {
        if self._lists.len() <= 1 {
            panic!(
                "Attempting to collapse while self._lists contains only {} lists.",
                self._lists.len()
            );
        }

        let left = match i >= 1 {
            true => self._lists[i - 1].len(),
            false => usize::MAX,
        };

        let right = match i + 1 < self._lists.len() {
            true => self._lists[i + 1].len(),
            false => usize::MAX,
        };

        assert!(left.min(right) < usize::MAX);
        if left < right {
            // collapse to k-1
            let mut removed = self._lists.remove(i);
            self._lists[i - 1].append(&mut removed);
        } else {
            let mut removed = self._lists.remove(i + 1);
            self._lists[i].append(&mut removed);
        }

        self._rebuild_index_tree();
    }

    /// Expand self._lists\[i]. self._lists\[i].len() must be > self._upper_load_factor in order for worthy expansion.
    fn _expand(&mut self, i: usize) {
        if self._lists[i].len() < self._upper_load_factor {
            panic!("Unnecessary expand at self._lists[{}]", i);
        }

        let size = self._lists[i].len();
        let removed: Vec<T> = self._lists[i].drain(size / 2..).collect();
        self._lists.insert(i + 1, removed);

        // instead of rebuilding the index segment tree, we should check whether we can "shift" the suffix to the right
        // then update tree at position k and k+1
        // and rebuild the first half of the index tree
        self._rebuild_index_tree();
    }

    /// Rebuild the index segment tree.
    fn _rebuild_index_tree(&mut self) {
        self._index_tree_offset = Self::DEFAULT_INDEX_TREE_OFFSET; // minimal size lower bound
        while self._index_tree_offset < self._lists.len() {
            self._index_tree_offset *= 2;
        }

        self._index_tree.fill(0);
        self._index_tree.resize(2 * self._index_tree_offset, 0);

        (0..self._lists.len()).for_each(|node| {
            self._index_tree[node + self._index_tree_offset] = self._lists[node].len();
        });

        (1..self._index_tree_offset).rev().for_each(|node| {
            self._index_tree[node] = self._index_tree[2 * node] + self._index_tree[2 * node + 1];
        });
    }

    /// Query the range sum of the index tree
    /// It computes the number of elements stored in self._lists\[ql..qr+1].
    fn _index_tree_sum(
        &self,
        ql: usize,
        qr: usize,
        opt_node: Option<usize>,
        opt_l: Option<usize>,
        opt_r: Option<usize>,
    ) -> usize {
        let node = opt_node.unwrap_or(1);
        let l = opt_l.unwrap_or(0);
        let r = opt_r.unwrap_or(self._index_tree_offset - 1);

        if ql <= l && r <= qr {
            return self._index_tree[node];
        }

        if qr < l || r < ql {
            return 0;
        }

        let m = (l + r) / 2;
        self._index_tree_sum(ql, qr, Some(2 * node), Some(l), Some(m))
            + self._index_tree_sum(ql, qr, Some(2 * node + 1), Some(m + 1), Some(r))
    }

    /// add val to position k of the underlying array of the segment tree
    fn _index_tree_add(&mut self, i: usize, val: i32) {
        let mut node = self._index_tree_offset + i;
        if val >= 0 {
            self._index_tree[node] += val as usize;
        } else {
            self._index_tree[node] -= (-val) as usize;
        }
        node /= 2;

        while node > 0 {
            self._index_tree[node] = self._index_tree[2 * node] + self._index_tree[2 * node + 1];
            node /= 2;
        }
    }

    /// Remove self._lists\[i]\[j]. It is assumed that self._lists\[i]\[j] will not go out of bound.
    fn _lists_remove(&mut self, i: usize, j: usize) -> T {
        if i >= self._lists.len() || j >= self._lists[i].len() {
            panic!(
                "List index out of range. Attempting to remove self._lists[{}][{}]",
                i, j
            );
        }

        let removed = self._lists[i].remove(j);
        self._len -= 1;

        if self._lists.len() > 1 && self._lists[i].len() < self._lower_load_factor {
            self._collapse(i);
        } else {
            self._index_tree_add(i, -1);
        }

        removed
    }

    /// Insert `element` into self._lists\[i]. It is assumed that self._lists\[i] is the correct insert position.
    fn _lists_insert(&mut self, i: usize, element: T) {
        // insert ele into self._lists[i]
        // assumptions:
        // 1. self._lists[i] must exist
        // 2. i is the correct position for inserting ele
        let pos = match self._lists[i].binary_search(&element) {
            Ok(p) => p,
            Err(p) => p,
        };

        self._lists[i].insert(pos, element);
        self._len += 1;

        if self._lists[i].len() > self._upper_load_factor {
            self._expand(i);
        } else {
            self._index_tree_add(i, 1);
        }
    }

    /// Find the position in self._lists which element should be inserted.
    fn _bisect_right_lists(&self, element: &T) -> usize {
        if &self._lists[0][0] > element {
            return 0;
        }

        let mut lo = 0;
        let mut hi = self._lists.len() - 1;
        if &self._lists[hi][0] <= element {
            return hi;
        }

        // self._lists[lo][0] <= element
        // self._lists[hi][0] > element
        let mut mid;
        while lo + 1 < hi {
            mid = (lo + hi) / 2;
            if &self._lists[mid][0] <= element {
                lo = mid;
            } else {
                hi = mid;
            }
        }

        lo
    }

    /// Returns (i,j) such that self._lists\[i]\[j] is the k-th element (0-indexed) of the SortedList.
    fn _locate_kth_element(&self, k: usize) -> (usize, usize) {
        // input k is 0-indexed
        if k >= self._len {
            panic!("SortedList: Index out of range.");
        }

        let is_leaf_node = |u| u >= self._index_tree_offset;
        let mut cnt = k + 1;

        let mut node: usize = 1;
        while !is_leaf_node(node) {
            if self._index_tree[2 * node] >= cnt {
                node *= 2;
            } else {
                cnt -= self._index_tree[2 * node];
                node = 2 * node + 1;
            }
        }

        // return values are 0-indexed
        (node - self._index_tree_offset, cnt - 1)
    }

    /// Retrieve an immutable reference of self._lists\[i]\[j].
    fn _at(&self, i: usize, j: usize) -> &T {
        &self._lists[i][j]
    }

    /// Returns a flattened view of the SortedList.
    fn _flat(&self) -> Vec<&T> {
        self._lists.iter().fold(Vec::new(), |mut cur, list| {
            list.iter().for_each(|element| {
                cur.push(element);
            });
            cur
        })
    }
}

/// Public method implementations
impl<T> SortedList<T>
where
    T: Ord,
{
    /// Creates an empty SortedList.
    ///
    /// # Example
    ///
    /// ```
    /// use sortedlist_rs::SortedList;
    ///
    /// let sorted_list: SortedList<i32> = SortedList::new();
    /// ```
    pub fn new() -> Self {
        Self::_default()
    }

    /// Find the k-th smallest (0-indexed) element in the SortedList.
    ///
    /// # Example
    ///
    /// ```
    /// use sortedlist_rs::SortedList;
    ///
    /// let sorted_list = SortedList::from([10, 2, 3]);
    /// assert_eq!(&3, sorted_list.kth_smallest(1));
    /// ```
    pub fn kth_smallest(&self, k: usize) -> &T {
        // k is 0-indexed
        let (i, j) = self._locate_kth_element(k);
        return self._at(i, j);
    }

    /// Clears the SortedList.
    ///
    /// # Example
    ///
    /// ```
    /// use sortedlist_rs::SortedList;
    ///
    /// let mut sorted_list = SortedList::from([10, 2, 3]);
    /// sorted_list.clear();
    ///
    /// assert_eq!(0, sorted_list.len());
    /// assert_eq!(true, sorted_list.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self._lists.clear();
        self._index_tree.clear();
        self._index_tree
            .resize(2 * Self::DEFAULT_INDEX_TREE_OFFSET, 0);
        self._index_tree_offset = Self::DEFAULT_INDEX_TREE_OFFSET;
        self._len = 0;
    }

    /// Insert `element` into the SortedList.
    ///
    /// # Example
    ///
    /// ```
    /// use sortedlist_rs::SortedList;
    ///
    /// let mut sorted_list = SortedList::new();
    /// sorted_list.insert(10);
    /// sorted_list.insert(6);
    /// sorted_list.insert(99);
    ///
    /// assert_eq!(3, sorted_list.len());
    /// assert_eq!(6, sorted_list[0]);
    /// assert_eq!(10, sorted_list[1]);
    /// ```
    pub fn insert(&mut self, element: T) {
        if self._len == 0 {
            self._lists.clear();
            self._lists.push(vec![]);
            self._lists_insert(0, element);
            return;
        }

        let k = self._bisect_right_lists(&element);
        self._lists_insert(k, element);
    }

    /// Pops the k-th smallest (0-indexed) element from the SortedList.
    ///
    /// # Example
    ///
    /// ```
    /// use sortedlist_rs::SortedList;
    ///
    /// let mut sorted_list = SortedList::from([10, 2, 99, 20, 30]);
    /// let popped = sorted_list.remove(3);
    ///
    /// assert_eq!(30, popped);
    /// ```
    pub fn remove(&mut self, k: usize) -> T {
        let (i, j) = self._locate_kth_element(k);
        self._lists_remove(i, j)
    }

    /// Binary searches the given element in the SortedList.
    /// Returns Ok(i) for exact match, Err(i) otherwise.
    ///
    /// # Example
    ///
    /// ```
    /// use sortedlist_rs::SortedList;
    ///
    /// let mut sorted_list = SortedList::from([10, 2, 99, 20, 30]);
    ///
    /// let result = sorted_list.binary_search(&30);
    /// assert_eq!(Ok(3), result);
    ///
    /// let result = sorted_list.binary_search(&90);
    /// assert_eq!(Err(4), result);
    /// ```
    pub fn binary_search(&self, element: &T) -> Result<usize, usize> {
        if self._len == 0 {
            return Err(0);
        }

        let i: usize = self._bisect_right_lists(element);
        if i == 0 {
            return self._lists[i].binary_search(element);
        }

        match self._lists[i].binary_search(element) {
            Ok(pos) => Ok(pos + self._index_tree_sum(0, i - 1, None, None, None)),
            Err(pos) => Err(pos + self._index_tree_sum(0, i - 1, None, None, None)),
        }
    }

    /// Returns whether the SortedList contains a specific element.
    ///
    /// # Example
    ///
    /// ```
    /// use sortedlist_rs::SortedList;
    ///
    /// let sorted_list = SortedList::from([10, 2, 99, 20]);
    ///
    /// assert_eq!(true, sorted_list.contains(&10));
    /// assert_eq!(false, sorted_list.contains(&90));
    /// ```
    pub fn contains(&self, element: &T) -> bool {
        self.binary_search(element).is_ok()
    }

    /// Returns the number of elements stored in the SortedList.
    ///
    /// # Example
    ///
    /// ```
    /// use sortedlist_rs::SortedList;
    ///
    /// let sorted_list = SortedList::from([10, 2, 99, 20]);
    ///
    /// assert_eq!(4, sorted_list.len());
    /// ```
    pub fn len(&self) -> usize {
        self._len
    }

    /// Returns whether the SortedList is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use sortedlist_rs::SortedList;
    ///
    /// let mut sorted_list: SortedList<i32> = SortedList::new();
    /// assert_eq!(true, sorted_list.is_empty());
    ///
    /// sorted_list.insert(1);
    /// assert_eq!(false, sorted_list.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the last element of the SortedList, i.e. the largest element.
    ///
    /// # Example
    ///
    /// ```
    /// use sortedlist_rs::SortedList;
    ///
    /// let sorted_list = SortedList::from([10, 2, 99, 20]);
    ///
    /// assert_eq!(Some(&99), sorted_list.last());
    /// ```
    pub fn last(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        return self._lists.last().unwrap().last();
    }

    /// Returns the first element of the SortedList, i.e. the smallest element.
    ///
    /// # Example
    ///
    /// ```
    /// use sortedlist_rs::SortedList;
    ///
    /// let sorted_list = SortedList::from([10, 2, 99, 20]);
    ///
    /// assert_eq!(Some(&2), sorted_list.first());
    /// ```
    pub fn first(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        return self._lists.first().unwrap().first();
    }

    /// Returns the element for the given index in the SortedList.
    ///
    /// # Example
    ///
    /// ```
    /// use sortedlist_rs::SortedList;
    ///
    /// let sorted_list = SortedList::from([10, 2, 99, 20]);
    ///
    /// assert_eq!(Some(&20), sorted_list.get(2));
    /// ```
    pub fn get(&self, index: usize) -> Option<&T> {
        if self.is_empty() || self.len() <= index {
            return None;
        }
        return Some(self.kth_smallest(index));
    }

    /// Returns a flattened view of the SortedList.
    ///
    /// # Example
    ///
    /// ```
    /// use sortedlist_rs::SortedList;
    ///
    /// let sorted_list = SortedList::from([10, 2, 99, 20]);
    /// let flattened = sorted_list.flatten();
    ///
    /// assert_eq!(vec![&2, &10, &20, &99], flattened);
    /// ```
    pub fn flatten(&self) -> Vec<&T> {
        self._flat()
    }

    /// Convert `self` into a new `Vec`.
    ///
    /// # Example
    ///
    /// ```
    /// use sortedlist_rs::SortedList;
    ///
    /// let sorted_list = SortedList::from([10, 2, 99, 20]);
    /// let v = sorted_list.to_vec();
    ///
    /// assert_eq!(vec![2, 10, 20, 99], v);
    /// ```
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        self._lists.iter().fold(vec![], |mut cur, list| {
            cur.append(&mut list.to_vec());
            cur
        })
    }

    /// Returns an iterator over the elements of the SortedList.
    ///
    /// # Example
    ///
    /// ```
    /// use sortedlist_rs::SortedList;
    ///
    /// let sorted_list = SortedList::from([10, 2, 99, 20]);
    /// let mut iterator = sorted_list.iter();
    /// 
    /// assert_eq!(Some(&2), iterator.next());
    /// assert_eq!(Some(&10), iterator.next());
    /// assert_eq!(Some(&20), iterator.next());
    /// assert_eq!(Some(&99), iterator.next());
    /// assert_eq!(None, iterator.next());
    /// 
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self._lists.iter().flat_map(|list| list.iter())
    }
}

impl<T> Default for SortedList<T>
where
    T: Ord,
{
    /// Creates an empty SortedList.
    fn default() -> Self {
        Self::_default()
    }
}

impl<T> Index<usize> for SortedList<T>
where
    T: Ord,
{
    type Output = T;

    /// Access the SortedList for the given index.
    ///
    /// # Example
    ///
    /// ```
    /// use sortedlist_rs::SortedList;
    ///
    /// let sorted_list = SortedList::from([20, 2, 10, 99, 50, 32]);
    ///
    /// assert_eq!(32, sorted_list[3]);
    /// ```
    fn index(&self, index: usize) -> &Self::Output {
        self.kth_smallest(index)
    }
}

impl<T> From<IntoIter<T>> for SortedList<T>
where
    T: Ord,
{
    /// Creates a SortedList from an IntoIter
    fn from(iter: IntoIter<T>) -> Self {
        let mut array: Vec<T> = iter.collect();
        array.sort();

        // directly construct sorted_list's internals, i.e. _lists, _len
        // This method is way faster than inserting elements one by one
        let sorted_iter = array.into_iter();
        let mut sorted_list = Self::default();
        sorted_list._len = sorted_iter.len();

        sorted_list._lists.push(vec![]);
        let mut last_list_size = 0;

        for element in sorted_iter {
            sorted_list._lists.last_mut().unwrap().push(element);
            last_list_size += 1;
            if last_list_size == sorted_list._load_factor {
                last_list_size = 0;
                sorted_list._lists.push(vec![]);
            }
        }

        sorted_list._rebuild_index_tree();
        sorted_list
    }
}

impl<T> From<Vec<T>> for SortedList<T>
where
    T: Ord,
{
    /// Creates a SortedList from a Vec
    fn from(array: Vec<T>) -> Self {
        Self::from(array.into_iter())
    }
}

impl<T> From<&[T]> for SortedList<T>
where
    T: Ord + Clone,
{
    /// Allocate a SortedList and fill it by cloning `array`'s items.
    fn from(array: &[T]) -> Self {
        Self::from(Vec::from(array))
    }
}

impl<T> From<&mut [T]> for SortedList<T>
where
    T: Ord + Clone,
{
    /// Allocate a SortedList and fill it by cloning `array`'s items.
    fn from(array: &mut [T]) -> Self {
        Self::from(Vec::from(array))
    }
}

impl<T, const N: usize> From<[T; N]> for SortedList<T>
where
    T: Ord,
{
    /// Allocate a SortedList and move `array`'s item into it.
    fn from(array: [T; N]) -> Self {
        Self::from(Vec::from(array))
    }
}

impl<T> fmt::Debug for SortedList<T>
where
    T: Ord + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self._flat(), f)
    }
}

#[cfg(test)]
mod tests {
    use rand::{seq::SliceRandom, thread_rng, Rng};

    use crate::SortedList;

    #[test]
    fn it_works() {
        let x = vec![1, 2, 3, 4, 9, 8, 7, 6, 5, 4];
        let y = vec![1, 2, 3, 4, 4, 5, 6, 7, 8, 9];
        let arr = SortedList::from(x);

        for i in 0..10 {
            assert_eq!(y[i], arr[i]);
        }
    }

    #[test]
    fn random_tests() {
        // Run tests with randomized inputs

        let test_size = 100_000;
        let test_op = 5_000;

        let mut rng = thread_rng();
        let mut array = vec![];
        for _ in 0..test_size {
            let x = rng.gen::<i32>();
            array.push(x);
        }

        // reference sorted list
        let mut copy = array.clone();
        copy.sort();

        // actual sorted list
        let mut sorted_list = SortedList::from(array);

        // Data setup
        for _ in 0..test_op {
            let x = rng.gen::<i32>();

            // insert into copy
            let k = match copy.binary_search(&x) {
                Ok(i) => i,
                Err(i) => i,
            };
            copy.insert(k, x);

            // insert into sorted list
            sorted_list.insert(x);
        }

        // Acutal tests
        for _ in 0..test_op + test_size {
            // Test: remove a random index, sorted order is maintained
            let idx = rng.gen_range(0..copy.len());
            let expect = copy.remove(idx);
            let actual = sorted_list.remove(idx);
            assert_eq!(expect, actual);

            // Test: first
            let expect = copy.first();
            let actual = sorted_list.first();
            assert_eq!(expect, actual);

            // Test: last
            let expect = copy.last();
            let actual = sorted_list.last();
            assert_eq!(expect, actual);

            // Test: binary_search
            let x = rng.gen::<i32>();
            let actual = sorted_list.binary_search(&x);
            let expect = copy.binary_search(&x);
            assert_eq!(expect, actual);

            // Test: get
            let index = rng.gen_range(0..copy.len() + 2000);
            let actual = sorted_list.get(index);
            let expect = copy.get(index);
            assert_eq!(expect, actual);

            // Test: len
            let actual = sorted_list.len();
            let expect = copy.len();
            assert_eq!(expect, actual);

            // Test: is_empty
            let actual = sorted_list.is_empty();
            let expect = copy.is_empty();
            assert_eq!(expect, actual);
        }
    }

    #[test]
    fn example() {
        let array = vec![90, 19, 25];
        let mut sorted_list = SortedList::from(array);

        sorted_list.insert(100);
        sorted_list.insert(1);
        sorted_list.insert(20);

        let x = sorted_list.remove(3);
        assert_eq!(25, x);
        assert_eq!(&20, sorted_list.kth_smallest(2));
        assert_eq!(20, sorted_list[2]);
    }

    #[test]
    fn binary_search_test() {
        let array = vec![20; 100_000];
        let sorted_list = SortedList::from(array);
        let x = 50;

        let actual = sorted_list.binary_search(&x);
        let expected: Result<usize, usize> = Err(100_000);

        assert_eq!(actual, expected);
    }

    #[test]
    fn contains_test() {
        let mut sorted_list = SortedList::from([10; 10_000]);
        assert!(sorted_list.contains(&10));
        assert!(!sorted_list.contains(&9));

        sorted_list.insert(9);
        assert!(sorted_list.contains(&9));

        sorted_list.insert(11);
        assert!(sorted_list.contains(&11));
    }

    #[test]
    fn clear_test() {
        // arrange
        let mut sorted_list_1 = SortedList::from([10; 10_000]);
        let mut sorted_list_2 = SortedList::from([1, 3, 4, 2, 0]);

        // act
        sorted_list_1.clear();
        sorted_list_2.clear();

        // assert
        for sorted_list in [sorted_list_1, sorted_list_2] {
            assert!(sorted_list._lists.is_empty());
            for val in &sorted_list._index_tree {
                assert!(val == &0);
            }
            assert!(sorted_list._index_tree.len() == 2 * sorted_list._index_tree_offset);
        }
    }

    #[test]
    fn to_vec_test() {
        // arrange
        let mut rng = thread_rng();
        let mut array: Vec<usize> = (0..5_000).collect();
        array.shuffle(&mut rng);

        let sorted_list = SortedList::from(array);

        // act
        let to_vec = sorted_list.to_vec();

        // assert
        assert_eq!((0..5_000).collect::<Vec<usize>>(), to_vec);
    }

    #[test]
    fn flatten_test() {
        // arrange
        let mut rng = thread_rng();
        let mut array: Vec<usize> = (0..5_000).collect();
        array.shuffle(&mut rng);

        let sorted_list = SortedList::from(array);

        // act
        let flatten = sorted_list.flatten();

        // assert
        for i in 0..5_000 {
            assert_eq!(flatten[i], &i);
        }
    }

    #[test]
    fn complex_data_structure_test() {
        // arrange
        let mut array: Vec<Vec<i32>> = vec![];
        for i in (0..2000).rev() {
            array.push((0..i + 1).collect());
        }

        // act
        let sorted_list = SortedList::from(array);

        // assert
        for i in 0..2000 {
            let actual: &Vec<i32> = &sorted_list[i];
            let expected = (0..i + 1).map(|x| x as i32).collect::<Vec<i32>>();

            assert_eq!(i + 1, actual.len());
            for j in 0..actual.len() {
                assert_eq!(actual[j], expected[j]);
            }
        }
    }
    
    #[test]
    fn break_case_insert_after_lst_has_been_clean() {
        let mut lst = SortedList::<usize>::new();
        lst.insert(3);
        lst.remove(0);
        lst.insert(1);
        lst.insert(5);
    }
}
