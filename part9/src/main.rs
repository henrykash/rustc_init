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

impl<'a> Drop for OurIterator<'a> {
    fn drop(&mut self) {
        println!("Dropping OurIterator");
    }
}
fn main() {
    let mut nums = vec![1, 2, 3];
    let mut nums2 = OurIterator::new(&nums, 2);
    let mut nums3 = OurIterator::new(&nums, 3);

    

    while let Some(num) = nums2.next() {
        println!("{}", num);
    }

    // nums2 is dropped here
    drop(nums2);

    while let Some(num) = nums3.next() {
        println!("{}", num);
    }

    drop(nums3);
    
   let _push = nums.push(4);

    print!("nums: {:?}", nums);
}
