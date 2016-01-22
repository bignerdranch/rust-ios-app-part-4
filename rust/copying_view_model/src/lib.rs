extern crate libc;
extern crate rand;

use rand::Rng;
use rand::distributions::{IndependentSample, Range};

use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

mod for_c;
pub use for_c::*;

#[derive(Clone)]
pub struct ViewModel {
    values: Vec<String>,
}

impl ViewModel {
    fn new() -> ViewModel {
        ViewModel { values: Vec::new() }
    }

    fn push(&mut self, value: String) {
        self.values.push(value);
    }

    fn remove(&mut self, index: usize) {
        self.values.remove(index);
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn value_at_index(&self, index: usize) -> &str {
        &self.values[index]
    }
}

pub trait ViewModelObserver: Send + 'static {
    fn inserted_item(&self, view_model: ViewModel, index: usize);
    fn removed_item(&self, view_model: ViewModel, index: usize);
    fn modified_item(&self, view_model: ViewModel, index: usize);
}

pub struct ViewModelHandle(Vec<mpsc::Sender<()>>);

impl ViewModelHandle {
    pub fn new<Observer>(num_threads: usize, observer: Observer) -> (ViewModelHandle, ViewModel)
        where Observer: ViewModelObserver
    {
        let starting_vm = ViewModel::new();
        let inner = Arc::new(Mutex::new(Inner::new(starting_vm.clone(), observer)));
        let mut worker_channels = Vec::with_capacity(num_threads);

        for i in 0..num_threads {
            let (tx, rx) = mpsc::channel();
            worker_channels.push(tx);

            let worker = ThreadWorker::new(i, inner.clone(), rx);
            thread::spawn(move || worker.main());
        }
        (ViewModelHandle(worker_channels), starting_vm)
    }
}

struct ThreadWorker<Observer: ViewModelObserver> {
    inner: Arc<Mutex<Inner<Observer>>>,
    thread_id: usize,
    shutdown: mpsc::Receiver<()>,
}

struct Inner<Observer: ViewModelObserver> {
    view_model: ViewModel,
    observer: Observer,
}

impl<Observer: ViewModelObserver> Inner<Observer> {
    fn new(vm: ViewModel, observer: Observer) -> Inner<Observer> {
        Inner {
            view_model: vm,
            observer: observer,
        }
    }
}

impl<Observer: ViewModelObserver> ThreadWorker<Observer> {
    fn new(thread_id: usize,
           inner: Arc<Mutex<Inner<Observer>>>,
           shutdown: mpsc::Receiver<()>)
           -> ThreadWorker<Observer> {
        ThreadWorker {
            inner: inner,
            thread_id: thread_id,
            shutdown: shutdown,
        }
    }

    fn main(&self) {
        let mut rng = rand::thread_rng();
        let between = Range::new(0i32, 10);

        // add a new item immediately
        self.add_new_item();

        loop {
            thread::sleep(Duration::from_millis(1000 + rng.gen_range(0, 3000)));

            if self.should_shutdown() {
                println!("thread {} exiting", self.thread_id);
                return;
            }

            // 20% of the time, add a new item.
            // 10% of the time, remove an item.
            // 70% of the time, modify an existing item.
            match between.ind_sample(&mut rng) {
                0 | 1 => self.add_new_item(),
                2 => self.remove_existing_item(&mut rng),
                _ => self.modify_existing_item(&mut rng),
            }
        }
    }

    fn should_shutdown(&self) -> bool {
        match self.shutdown.try_recv() {
            Err(mpsc::TryRecvError::Disconnected) => true,
            Err(mpsc::TryRecvError::Empty) => false,
            Ok(()) => unreachable!("thread worker channels should not be used directly"),
        }
    }

    fn add_new_item(&self) {
        let mut inner = self.inner.lock().unwrap();
        inner.view_model.push(format!("rust-thread-{}", self.thread_id));
        inner.observer.inserted_item(inner.view_model.clone(), inner.view_model.len() - 1);
    }

    fn remove_existing_item<R: Rng>(&self, rng: &mut R) {
        let mut inner = self.inner.lock().unwrap();
        if inner.view_model.len() > 0 {
            let i = rng.gen_range(0, inner.view_model.len());
            inner.view_model.remove(i);
            inner.observer.removed_item(inner.view_model.clone(), i);
        }
    }

    fn modify_existing_item<R: Rng>(&self, rng: &mut R) {
        let mut inner = self.inner.lock().unwrap();
        if inner.view_model.len() > 0 {
            let i = rng.gen_range(0, inner.view_model.len());
            inner.view_model.values[i].push_str(&format!("-{}", self.thread_id));
            inner.observer.modified_item(inner.view_model.clone(), i);
        }
    }
}
