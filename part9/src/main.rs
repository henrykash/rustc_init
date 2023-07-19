struct OurIterator<'a> {
    nums: &'a [i32],
    index: usize,
    x: i32,
}

impl<'a> OurIterator<'a> {
    //explicit lifetime annotation required
    fn new(nums: &'a [i32], x: i32) -> Self {
        Self {
            nums,
            index: 0,
            x,
        }
    }
}

impl<'a> Iterator for OurIterator<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.nums.len() {
            return None;
        } else {
            let num = self.nums[self.index];
            self.index += 1;
            Some(num * self.x)
        }
    }
}

fn main() {
    let nums = vec![1, 2, 3];
    let mut iter = OurIterator::new(&nums, 2);
    while let Some(num) = iter.next() {
        println!("{}", num);
    }
}
