#[derive(Debug, Clone)]
pub struct Data {
    a: String,
    b: i32,
}

fn main() {
    // let data: Data = Data::new().a("123".to_owned()).b(12);
    let data: Data = DataBuilder::new().a("123".to_owned()).b(12).build();
    println!("{:?}", data);
}

struct DataBuilder {
    data: Data,
}

impl DataBuilder {
    fn new() -> Self {
        DataBuilder {
            data: Data {
                a: "123".to_owned(),
                b: 2,
            },
        }
    }
    fn a(mut self, a: String) -> Self {
        self.data.a = a;
        self
    }
    fn b(mut self, b: i32) -> Self {
        self.data.b = b;
        self
    }
    fn build(self) -> Data {
        self.data
    }
}

trait Builder {
    fn new() -> Self;
    fn a(self, a: String) -> Self;
    fn b(self, b: i32) -> Self;
    // fn build_mut(&mut self) -> &mut Self;
}

impl Builder for Data {
    fn a(mut self, a: String) -> Self {
        self.a = a;
        self
    }

    fn b(mut self, b: i32) -> Self {
        self.b = b;
        self
    }

    // fn build_mut(&mut self) -> &mut Self {
    //     self
    // }

    fn new() -> Self {
        Data {
            a: "abc".to_owned(),
            b: 1,
        }
    }
}
