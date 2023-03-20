
pub struct SparseVector {
    items: Vec<(i32, i32)>
}

impl SparseVector {
    
    fn new(nums: Vec<i32>) -> Self {
        
        let mut items: Vec<(i32, i32)> = Vec::new();
        
        for (i, n) in nums.into_iter().enumerate() {
            if n != 0 {
                items.push((i as i32, n));
            }
        }
        
        Self {items}
    }
    
    fn dot_product(&self, vec: SparseVector) -> i32 {
        let mut i = 0;
        let mut j = 0;
        let mut res = 0;
        
        while i < self.items.len() && j < vec.items.len() {
            let item1 = self.items[i];
            let item2 = vec.items[j];
            if item1.0 == item2.0 {
                res += item1.1 * item2.1;
                i += 1;
                j += 1;
            } else if item1.0 > item2.0 {
                j += 1;
            } else {
                i += 1;
            }
        }
        
        return res;
    }
}

pub fn test() {
    let spv1 = SparseVector::new(vec![1, 0, 0, 2, 3]);
    let spv2 = SparseVector::new(vec![0, 3, 0, 4, 0]);
    
    let res = spv1.dot_product(spv2);
    println!("Result is: {}", res);
}