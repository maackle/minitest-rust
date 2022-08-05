use squash::multitest;

struct Foo(pub u8);

struct Bar {
    x: u16,
    s: String,
}

trait Common {
    fn num(&self) -> u8;
}

impl Common for Foo {
    fn num(&self) -> u8 {
        self.0
    }
}

impl Common for Bar {
    fn num(&self) -> u8 {
        self.x as u8
    }
}

fn do_stuff<T: Common>(a: T, v: Vec<T>) -> u8 {
    a.num() + v.into_iter().map(|x| x.num()).sum::<u8>()
}

#[multitest(td, Foo, Bar)]
fn testtest() {
    let a = td(Foo(0));
    let v = vec![td(Foo(1)), td(Foo(2))];
    assert_eq!(do_stuff(a, v), 3);
}
