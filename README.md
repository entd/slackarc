# slackarc
Lazy on-demand static types for creating global shared pointers, 
that are droped when no longer needed.

example:

```rust
use slackarc::GlobalWeak;
use std::{fmt::Debug, sync::Arc};

#[derive(Debug)]
struct Foo {
    a: String,
    b: u64,
}

static REFERENCE: GlobalWeak<Foo> = GlobalWeak::new(|| {
    println!("Building First");
    Foo {
        a: "Bar1".to_owned(),
        b: 75,
    }
});

static REFERENCE2: GlobalWeak<Foo> = GlobalWeak::new(|| {
    println!("Building Second");
    Foo {
        a: "Bar2".to_owned(),
        b: 45,
    }
});

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Bye {0}", self.a);
    }
}

fn main() {
    let fooarc2: Arc<Foo> = REFERENCE2.upgrade().unwrap();
    let fooarc1: Arc<Foo> = REFERENCE.upgrade().unwrap();
    println!("Hello, world! {fooarc1:?}");
    println!("Hello, world! {fooarc2:?}");
    std::mem::drop(fooarc2);
    let _: Arc<Foo> = REFERENCE2.upgrade().unwrap();
}
```
Allocate and share global objects. 
When the last reference is lost entire object is lost.
Next call will reinitialize object.
