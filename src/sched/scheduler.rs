use std::{fmt, time, thread};
use std::sync::{Arc};
use std::sync::atomic::{AtomicBool, Ordering};

/*******************************************************
******************* ACTION **************************
********************************************************/

pub struct Action {
    name: String,
    state: String
    //state: sched::action_state::ActionState
}

impl Action {
    pub fn init() -> Action {
        Action{name: "init_name".to_string(), state: "init_state".to_string()}
    }

    pub fn new(n: String, s: String) -> Action {
        Action{name: n, state: s}
    }

    pub fn run(self){

    }
}


impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.name, self.state)
    }
}


/*******************************************************
******************* SCHEDULER **************************
********************************************************/

/// The `Scheduler` type. Used to run actions lists
pub struct Scheduler {
    pub actions: Vec<Action>,
    pub is_running: Arc<AtomicBool>,
    pub iteration: i32,
}

impl Scheduler {

    pub fn init(actions: Vec<Action>) -> Scheduler {
        Scheduler{actions: actions, is_running: Arc::new(AtomicBool::new(true)), iteration: 0}
    }

    pub fn run(&self){
        let alive = self.is_running.clone();
        let child = thread::spawn(move || {

            println!("Thread running ...");
            while alive.load(Ordering::Relaxed){
                let three_secs = time::Duration::from_secs(3);
                thread::sleep(three_secs);
                //self.iteration();
            }

        });
        println!("Iteration completed !")
    }

    pub fn single_iteration(&self){
        println!("Iterate !")
    }

    pub fn stop(&self){
        self.is_running.store(false, Ordering::SeqCst);
        println!("Thread STOP !!");
    }
}


impl fmt::Display for Scheduler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut my_str: String = "ACTIONS".to_string();
        let mut count: i32 = 0;
        for v in &self.actions {
            my_str = format!("{} \n{} - {}, {}", my_str, count, v.name, v.state);
            count += 1;
        }
        write!(f, "{}", my_str)
    }
}