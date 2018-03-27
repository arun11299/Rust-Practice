use std::thread;
use std::sync::{mpsc,Arc,Mutex};

trait FnBox {
    fn call_box(self : Box<Self>);
}

impl<F : FnOnce()> FnBox for F {
    ///
    fn call_box(self : Box<F>) {
        (*self)()
    }
}

type ThreadHandle = thread::JoinHandle<()>;

struct ExecutionUnit
{
    job : Box<FnBox + 'static + Send>,
}

pub struct Work
{
    id : u32,
    thread : ThreadHandle,
}

pub struct ThreadPool
{
    workers : Vec<Work>,
    sender : mpsc::Sender<ExecutionUnit>,
}

impl Work {
    ///
    fn new(id : u32, receiver : Arc<Mutex<mpsc::Receiver<ExecutionUnit>>>) -> Work
    {
        let thread = thread::spawn(move || {
            loop {
                let exec_unit = receiver.lock().unwrap().recv().unwrap();
                exec_unit.job.call_box();
            }
        });
        Work {
            id,
            thread,
        }
    }
}

impl ThreadPool {
    ///
    pub fn new(num_workers : usize) -> ThreadPool
    {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut threads = Vec::with_capacity(num_workers);
        for i in 0..num_workers {
            threads.push(Work::new(i as u32, Arc::clone(&receiver)));
        }

        ThreadPool{
            workers : threads,
            sender,
        }
    }

    ///
    pub fn execute<F>(&mut self, func : F)
        where F : FnOnce() + 'static + Send
    {
        //select a work indexed by the indexer
        self.sender.send(ExecutionUnit{ job : Box::new(func) }).unwrap();
    }
}
