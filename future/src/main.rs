use std::{
    future::Future,
    mem,
    pin::Pin,
    rc::Rc,
    sync::{
        mpsc::{channel, Sender},
        Condvar,
    },
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
    thread::{self, JoinHandle},
    time::Duration,
};

use std::io::Read;

use future::reactor::Reactor;
use poll::{file::File, Token};

type ExecutorToken = usize;

fn main() {
    // This is just to make it easier for us to see when our Future was resolved
    let start = Instant::now();

    // Many runtimes create a global `reactor` we pass it as an argument
    let reactor = Reactor::new();

    // We create two tasks:
    // - first parameter is the `reactor`
    // - the second is a timeout in seconds
    // - the third is an `id` to identify the task
    let future1 = Task::new(reactor.clone(), 1, 1);
    let future2 = Task::new(reactor.clone(), 2, 2);

    // an `async` block works the same way as an `async fn` in that it compiles
    // our code into a state machine, `yielding` at every `await` point.
    let fut1 = async {
        let val = future1.await;
        println!("Got {} at time: {:.2}.", val, start.elapsed().as_secs_f32());
    };

    let fut2 = async {
        let val = future2.await;
        println!("Got {} at time: {:.2}.", val, start.elapsed().as_secs_f32());
    };

    // Our executor can only run one and one future, this is pretty normal
    // though. You have a set of operations containing many futures that
    // ends up as a single future that drives them all to completion.
    let mainfut = async {
        fut1.await;
        fut2.await;
    };

    // This executor will block the main thread until the futures are resolved
    block_on(mainfut);
}

// ============================= EXECUTOR ====================================
// ===========================================================================

struct Executor {
    receiver: Receiver<Token>,
    actions: Vec<(ExecutorToken, Box<dyn FnMut()>)>,
}

// Our executor takes any object which implements the `Future` trait
fn block_on<F: Future>(mut future: F) -> F::Output {
    // the first thing we do is to construct a `Waker` which we'll pass on to
    // the `reactor` so it can wake us up when an event is ready.
    let mywaker = Arc::new(MyWaker {
        thread: thread::current(),
    });
    let waker = mywaker_into_waker(Arc::into_raw(mywaker));

    // The context struct is just a wrapper for a `Waker` object. Maybe in the
    // future this will do more, but right now it's just a wrapper.
    let mut cx = Context::from_waker(&waker);

    // So, since we run this on one thread and run one future to completion
    // we can pin the `Future` to the stack. This is unsafe, but saves an
    // allocation. We could `Box::pin` it too if we wanted. This is however
    // safe since we shadow `future` so it can't be accessed again and will
    // not move until it's dropped.
    let mut future = unsafe { Pin::new_unchecked(&mut future) };

    // We poll in a loop, but it's not a busy loop. It will only run when
    // an event occurs, or a thread has a "spurious wakeup" (an unexpected wakeup
    // that can happen for no good reason).
    let val = loop {
        match Future::poll(future.as_mut(), &mut cx) {
            // when the Future is ready we're finished
            Poll::Ready(val) => break val,

            // If we get a `pending` future we just go to sleep...
            Poll::Pending => thread::park(),
        };
    };
    val
}

// ====================== FUTURE IMPLEMENTATION ==============================
// ===========================================================================

pub struct ReadNChars<'a> {
    reactor: &'a Reactor,
    file: File,
    token: Token,
    n: usize,
    buf: String,
}

impl<'a> ReadNChars<'a> {
    fn new(reactor: &'a Reactor, file: File, n: usize) -> ReadNChars<'a> {
        let token = file.file_descriptor as Token;
        let buf = String::new();

        reactor.poll_add(&file, token);

        ReadNChars {
            reactor,
            file,
            token,
            n,
            buf,
        }
    }
}

impl<'a> Future for ReadNChars<'a> {
    type Output = String;

    // Poll is the what drives the state machine forward and it's the only
    // method we'll need to call to drive futures to completion.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<String> {
        let this = self.get_mut();
        this.file.read_to_string(&mut this.buf);
        if this.buf.len() < self.n {
            this.reactor.add_waker(self.token, cx.waker().clone());
            Poll::Pending
        } else {
            Poll::Ready(self.buf.clone())
        }
    }
}
