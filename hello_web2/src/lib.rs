use std::{
    sync::{
        mpsc,
        mpsc::{Receiver, RecvError, Sender},
        Arc, Mutex,
    },
    thread,
    thread::JoinHandle,
};

/// 线程池结构体
pub struct ThreadPool {
    /// 工作线程队列
    threads: Vec<Worker>,
    // 任务发送者
    sender: Option<Sender<Job>>,
}

// 任务，为简化代码将Box<dyn FnOnce() + Send + 'static>取别名为 Job
type Job = Box<dyn FnOnce() + Send + 'static>;

/// 任务线程结构
struct Worker {
    // 线程id
    id: usize,
    // 任务内容
    thread: Option<JoinHandle<()>>,
}

/// 工作线程实现
impl Worker {
    /// 新建工作线程并返回
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        // 构建工作内容，返回类型为一个新线程，move关键字，将主线程变量所有权捕获入闭包中
        let thread: JoinHandle<()> = thread::spawn(move || loop {
            // loop 循环表示该线程一直运行和程序共存亡
            // 接收器接收消息
            let message: Result<Job, RecvError> =
                // 先lock住，n个线程同时接收消息，先到先得，没有拿到值的在这里阻塞
                // recv()函数转换为Result<Job, RecvError>类型
                receiver.lock().unwrap().recv();
            match message {
                // message内容就是一个一次性函数
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    // 执行函数
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        // 构建worker返回
        Worker {
            id: id,
            thread: Some(thread),
        }
    }
}

impl ThreadPool {
    /// 线程池初始化函数
    pub fn new(size: usize) -> ThreadPool {
        // 线程池线程数必须为正整数
        assert!(size > 0);
        // 创建发送者与接收者
        let (sender, receiver) = mpsc::channel();
        // 多线程环境下，需要Arc(多线程计数器) 和 Mutex(互斥锁) 包装发送者，可能被多个线程引用且要保证线程安全
        let receiver: Arc<Mutex<Receiver<Job>>> = Arc::new(Mutex::new(receiver));
        // 初始化线程池列表
        let mut workers: Vec<Worker> = Vec::with_capacity(size);
        // 给线程池添加工作线程
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        // 构建线程池返回
        ThreadPool {
            threads: workers,
            sender: Some(sender),
        }
    }

    /// 线程池执行函数
    pub fn execute<F>(&self, f: F)
    where
        // 闭包作为任务只需被线程执行一次，所以选择FnOnce类型
        // Send是因为闭包需要从一个线程传递到另一个线程
        // 因为不知道闭包什么时候执行完成，所以设置为'static生命周期
        F: FnOnce() + Send + 'static,
    {
        // 将传入的闭包包装成一个job
        let job: Box<F> = Box::new(f);
        // 用发送者将job发送出去
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.threads {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
