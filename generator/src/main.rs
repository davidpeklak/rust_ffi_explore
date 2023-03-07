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

fn main() {
    let mut gen = GeneratorA::start(4);

    if let GeneratorState::Yielded(n) = gen.resume() {
        println!("Got value {}", n);
    }

    if let GeneratorState::Complete(()) = gen.resume() {
        ()
    }
}
