use std::{thread::JoinHandle, sync::{Arc, atomic::AtomicUsize, mpsc::{channel, Sender, Receiver}, Mutex}};

use crate::{Script, Camera};

#[derive(Clone)]
pub enum Job {
    Close,
    Update(Arc<Box<dyn Script>>),
    UpdateCamera(Arc<Box<dyn Camera>>)
}

pub struct Threads {
    workers: Vec<Worker>,
    jobs_running: Arc<AtomicUsize>,
    tx: Sender<Job>,
    park_main_thread_rx: Receiver<()>
}
impl Threads {
    pub fn new() -> Self {
        let logical_cores = num_cpus::get();
        
        let (tx, rx): (Sender<Job>, Receiver<Job>) = channel();
        let rx = Arc::new(Mutex::new(rx));

        let (park_main_thread_tx, park_main_thread_rx): (Sender<()>, Receiver<()>) = channel();
        let park_main_thread_tx = Arc::new(Mutex::new(park_main_thread_tx));

        let jobs_running = Arc::new(AtomicUsize::new(0));
        
        let mut workers = Vec::with_capacity(logical_cores);
        for id in 0..logical_cores {
            let rx = rx.clone();
            let jobs_running = jobs_running.clone();
            let park_main_thread_tx = park_main_thread_tx.clone();
            workers.push(Worker {
                id,
                handle: std::thread::spawn(move || {
                    loop {
                        let rx = rx.lock().unwrap();
                        let recv = rx.recv();
                        drop(rx);
                        match recv.unwrap() {
                            Job::Close => {
                                jobs_running.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
                                break
                            },
                            Job::Update(s) => s.update(),
                            Job::UpdateCamera(c) => c.update()
                        }
                        jobs_running.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
                        if jobs_running.load(std::sync::atomic::Ordering::Relaxed) == 0 {
                            park_main_thread_tx.lock().unwrap().send(()).unwrap();
                        }
                    }
                })
            })
        }
        
        Self {
            workers,
            jobs_running,
            tx,
            park_main_thread_rx
        }
    }
    pub fn send(&self, job: Job) {
        self.jobs_running.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.tx.send(job).unwrap()
    }
    pub fn send_jobs(&self, jobs: &[Job]) {
        self.jobs_running.fetch_add(jobs.len(), std::sync::atomic::Ordering::Relaxed);
        for job in jobs {
            self.tx.send(job.clone()).unwrap()
        }
    }
    pub fn send_to_all(&self, job: Job) {
        self.send_jobs(&vec![job;self.workers.len()])
    }
    pub fn wait(&self) {
        if self.jobs_running.load(std::sync::atomic::Ordering::Relaxed) > 0 {
            self.park_main_thread_rx.recv().unwrap();
        }
    }
}

pub struct Worker {
    pub id: usize,
    pub handle: JoinHandle<()>
}

impl Drop for Threads {
    fn drop(&mut self) {
        self.send_to_all(Job::Close);
        while let Some(worker) = self.workers.pop() {
            worker.handle.join().unwrap();
        }
    }
}