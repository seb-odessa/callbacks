struct Callbacks {
    callbacks : Vec<Box<FnMut(i32)>>,
}
impl Callbacks {
    pub fn new() -> Self {
        Callbacks{ callbacks : Vec::new() }
    }
    pub fn register(&mut self, callback : Box<FnMut(i32)>) {
        self.callbacks.push(callback);
    }
    pub fn register_generic<F: FnMut(i32)+'static>(&mut self, callback: F) {
            self.callbacks.push(Box::new(callback));
    }
    pub fn call(&mut self, val: i32) {
        for callback in &mut self.callbacks {
            (&mut *callback)(val);
        }
    }
}



fn main() {
    let mut c = Callbacks::new();
    c.register(Box::new(|val| println!("Callback fn1({})", val)));
    c.call(0);
    c.call(1);
    {
        let mut cnt : usize = 0;
        c.register_generic(move |val| { cnt = cnt + 1; println!("Closure2({}) cnt = {}", val, cnt)});
    }
    c.call(2);
    c.call(3);
}
