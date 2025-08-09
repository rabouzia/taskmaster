	use std::process::{Command, Child};
use std::collections::HashMap;

#[derive(Debug)]
enum RestartPolicy {
    Always,
    OnFailure,
    Never,
}

#[derive(Debug)]
enum TaskStatus {
    Starting,
    Running,
    Stopped,
    Exited,
    Failed,
}

struct Daemon {
    name: String,
    command: String,
    arguments: Vec<String>,
    pid: Option<u32>,
    status: TaskStatus,
    restart_policy: RestartPolicy,
    attempts: u32,
}

// === Functions to implement ===

fn load_config() -> Vec<Daemon> {
    // TODO: Parse a config file and return list of Daemon tasks
	let content = fs::read_to_string("config.toml")
		.expect("Failed to read config file");
	serde_yaml::from_str(&content)
		.expect("Failed to parse config file");
	
    Vec::new()
}

fn register_tasks(tasks: Vec<Daemon>) {
    // TODO: Store tasks in a map or struct
}

fn run_all_tasks() {
    // TODO: Spawn all defined tasks (in threads if needed)
}

fn monitor_tasks() {
    // TODO: Check if tasks are still alive, restart if necessary
}

fn get_task_by_name(_name: &str) -> Option<&'static mut Daemon> {
    // TODO: Lookup task by name from global state
    None
}

fn start_task(_task: &mut Daemon) {
    // TODO: Use std::process::Command to start
}

fn stop_task(_task: &mut Daemon) {
    // TODO: Kill the process by PID
}

fn restart_task(task: &mut Daemon) {
    stop_task(task);
    start_task(task);
}

fn should_restart(_task: &Daemon) -> bool {
    // TODO: Evaluate restart policy and exit status
    false
}


fn main() {
    println!("Taskmaster started (foreground mode)");

    let tasks = load_config();
    register_tasks(tasks);
    run_all_tasks();

    loop {
        monitor_tasks();
    }
}
