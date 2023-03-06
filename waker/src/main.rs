use std::{mem::align_of, mem::size_of};

// A reference to a trait object is a fat pointer: (data_ptr, vtable_ptr)
trait Test {
    fn add(&self) -> i32;
    fn sub(&self) -> i32;
    fn mul(&self) -> i32;
}

/// Our home-brewed fat pointer to a trait object( mind the similarity to std::task::RawWakerVTable)
#[repr(C)] // https://doc.rust-lang.org/nomicon/other-reprs.html#reprc
struct FatPointer<'a> {
    /// A reference is a pointer to an instantiated `Data`instance.
    data: &'a mut Data,
    /// Since we need to pass in literal values like length and alignment it's
    /// easiest for us to convert pointers to usize-integers instead of the other way around.
    vtable: *const usize,
}

// This is the data in our trait object. It's just two numbers we want to operate on.
struct Data {
    a: i32,
    b: i32,
}

// ====== function definitions ======
fn add(s: &Data) -> i32 {
    s.a + s.b
}
fn sub(s: &Data) -> i32 {
    s.a - s.b
}
fn mul(s: &Data) -> i32 {
    s.a * s.b
}

fn main() {
    let mut data = Data { a: 3, b: 2 };

    let vtable = vec![
        0,
        size_of::<Data>(),
        align_of::<Data>(),
        add as usize,
        sub as usize,
        mul as usize,
    ];

    let fat_pointer = FatPointer {
        data: &mut data,
        vtable: vtable.as_ptr(),
    };

    let test = unsafe { std::mem::transmute::<FatPointer, &dyn Test>(fat_pointer) };

    // And voalá, it's now a trait object we can call methods on
    println!("Add: 3 + 2 = {}", test.add());
    println!("Sub: 3 - 2 = {}", test.sub());
    println!("Mul: 3 * 2 = {}", test.mul());
}
