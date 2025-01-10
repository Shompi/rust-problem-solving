struct MinStack {
    min: Option<i32>,
    high: Option<i32>,    
    length: usize,
    values: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        Self {
            high: None,
            min: None,
            length: 0,
            values: vec![]
        }
    }
    
    fn push(&mut self, val: i32) {
        self.values.push(val);

        if self.high == None {
            self.high = Some(val);
        } else if self.high < Some(val) {
            self.high = Some(val);
        }

        if self.min == None {
            self.min = Some(val);
        } else if self.min > Some(val) {
            self.min = Some(val);
        }

        self.length += 1;
    }
    
    fn pop(&mut self) {
        let v = self.values.pop().unwrap();

        if self.high != None && self.high == Some(v) {
            
            if

        }
        if self.min != None && self.min == Some(v) {
            self.min = None;
        }

        self.length -= 1;
    }
    
    fn top(&self) -> i32 {
        self.values[self.length - 1]
    }
    
    fn get_min(&self) -> i32 {
        self.min.unwrap()
    }
}

fn main() {
    println!("Hello, world!");

    let mut stack = MinStack::new();

    stack.
}
