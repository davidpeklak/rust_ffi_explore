enum GeneratorState<Y, R> {
    Yielded(Y),
    Complete(R),
}

trait Generator {
    type Yield;
    type Return;
    fn resume(&mut self) -> GeneratorState<Self::Yield, Self::Return>;
}

enum GeneratorA {
    Enter(i32),
    Yield1(i32),
    Exit,
}

impl GeneratorA {
    fn start(a1: i32) -> Self {
        GeneratorA::Enter(a1)
    }
}

impl Generator for GeneratorA {
    type Yield = i32;
    type Return = ();

    fn resume(&mut self) -> GeneratorState<i32, ()> {
        match self {
            GeneratorA::Enter(a) => {
                println!("Hello");
                let a = *a * 2;
                *self = GeneratorA::Yield1(a);
                GeneratorState::Yielded(a)
            },
            GeneratorA::Yield1(_) => {
                println!("World");
                *self = GeneratorA::Exit;
                GeneratorState::Complete(())
            },
            GeneratorA::Exit => {
                panic!("Cannot resume exited generator");
            }
        }
    }
}

enum GeneratorB {
    Enter,
    Yield1 {
        to_borrow: String,
        borrowed: *const String,
    },
    Exit,
}

impl GeneratorB {
    fn start() -> Self {
        GeneratorB::Enter
    }
}

impl Generator for GeneratorB {
    type Yield = usize;
    type Return = ();

    fn resume(&mut self) -> GeneratorState<usize, ()> {
        match self {
            GeneratorB::Enter => {
                let to_borrow = String::from("Hello");
                let borrowed = &to_borrow;
                let res = borrowed.len();

                *self = GeneratorB::Yield1 { to_borrow, borrowed: std::ptr::null() };

                // NB! And we set the pointer to reference the to_borrow string here
                if let GeneratorB::Yield1 {to_borrow, borrowed} = self {
                    *borrowed = to_borrow;
                }

                GeneratorState::Yielded(res)
            },
            GeneratorB::Yield1 { to_borrow: _, borrowed } => {
                let borrowed: &String = unsafe {&**borrowed};
                println!("Hello {}", borrowed);
                *self = GeneratorB::Exit;
                GeneratorState::Complete(())
            },
            GeneratorB::Exit => {
                panic!("Cannot resume exited generator");
            }
        }
    }
}

fn main() {
    let mut gen = GeneratorA::start(4);

    if let GeneratorState::Yielded(n) = gen.resume() {
        println!("Got value {}", n);
    }

    if let GeneratorState::Complete(()) = gen.resume() {
        ()
    }

    let mut gen = GeneratorB::start();

    if let GeneratorState::Yielded(n) = gen.resume() {
        println!("Got value {}", n);
    }

    let mut gen2 = GeneratorB::start();
    std::mem::swap(&mut gen, &mut gen2);

    if let GeneratorState::Complete(()) = gen2.resume() {
        ()
    }
}
