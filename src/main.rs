use std::io;

mod sched;

fn main() {
    println!("Scheduler start");

    // Initalize actions
    let action1 = sched::scheduler::Action::init();
    let action2 = sched::scheduler::Action::new("n2".to_string(), "s2".to_string());

    let mut action_list = std::vec::Vec::new();
    action_list.push(action1);
    action_list.push(action2);


    // Run the thread
    let mut scheduler = sched::scheduler::Scheduler::init(action_list);
    scheduler.run();


    /// WAIT for user command to stop th thread
    let mut input = String::new();
    println!("- 's' for stopping !");

    loop {
        match io::stdin().read_line(&mut input) {
            Ok(n) => {

                if input.trim() == "s" {
                    scheduler.stop();
                    break
                }
                println!("- 's' for stopping !");
            }
            Err(error) => {
                println!("error: {}", error);
                scheduler.stop();
                break
            },
        }
    }

    println!("{}", scheduler)
}
