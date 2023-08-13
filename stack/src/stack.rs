pub struct Stack<T> {
    cont: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { cont: Vec::new() }
    }
    fn top(&self) -> Option<&T> {
        self.cont.last()
    }

    fn top_mut(&mut self) -> Option<&mut T> {
        self.cont.last_mut()
    }
}
