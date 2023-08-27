use std::marker::PhantomData;

pub trait Stack {
    type DataType;
    type Container;

    fn new() -> Self;
    // fn top(&self) -> Option<Self::T>;
    // fn top_mut(&mut self) -> Option<&mut Self::T>;
}

pub struct VecStack<T, U> {
    data_type: PhantomData<T>,
    data: U,
}

impl<T, U> Stack for VecStack<T, U>
where
    T: i32,
    U: Vec<T>,
{
    type DataType = T;
    type Container = U;

    fn new() -> Self {
        VecStack {
            data_type: 1,
            data: Self::Container::new(),
        }
    }
}
