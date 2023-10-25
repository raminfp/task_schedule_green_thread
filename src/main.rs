use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};


struct Task {
    task: Box<dyn FnMut() + 'static>,
    interval: Duration,
}

struct GreenThread {
    task: Task,
    last_run: Instant,
}

impl GreenThread {
    fn new(task: Task) -> GreenThread {
        GreenThread {
            task,
            last_run: Instant::now(),
        }
    }

    fn run_if_due(&mut self) {

        let elapsed = self.last_run.elapsed();
        if elapsed >= self.task.interval {
            (self.task.task)();
            self.last_run = Instant::now();
        }

    }
}

struct Scheduler {
    threads: Vec<Rc<RefCell<GreenThread>>>,
}

impl Scheduler {
    fn new() -> Scheduler {
        Scheduler { threads: Vec::new() }
    }

    fn add_thread(&mut self, task: Task) {
        let thread = Rc::new(RefCell::new(GreenThread::new(task)));
        self.threads.push(thread);
    }

    fn run(&mut self) {
        loop {
            for thread in &self.threads {
                thread.borrow_mut().run_if_due();
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    }
}

fn schedule_tasks(tasks: Vec<Task>) {
    let mut scheduler = Scheduler::new();

    for task in tasks {
        scheduler.add_thread(task);
    }

    scheduler.run();
}

fn task_without_params() {
    // loop {
        println!("I am running 10 sec task.");
    // }

}

fn task_with_params(param1: i32, param2: &str) {
    println!("I am running task with params: param1 = {}, param2 = {}", param1, param2);
}

fn main() {
    let interval1 = Duration::from_secs(10);  // 1 second for task1
    let interval2 = Duration::from_secs(1); // 60 seconds (1 minute) for task2

    let task1 = Task {
        task: Box::new(task_without_params),
        interval: interval1,
    };

    let task2 = Task {
        task: Box::new(|| println!("Task 2")),
        interval: interval2,
    };

    // Create a new task for task_without_params
    let task3 = Task {
        task: Box::new(task_without_params),
        interval: Duration::from_secs(10), // Interval for task_without_params
    };

    // Create a new task for task_with_params
    let task4 = Task {
        task: Box::new(|| task_with_params(42, "Hello")),
        interval: Duration::from_secs(5), // Interval for task_with_params
    };

    let tasks = vec![task1, task2, task3, task4];

    schedule_tasks(tasks);
}


