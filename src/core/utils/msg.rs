use colored::Colorize;


pub fn error(msg: &str) {
    println!("{} {}", "error:".on_red(), msg);
}

pub fn warning(msg: &str) {
    println!("{} {}", "warning:".yellow(), msg);
}

pub fn hint(msg: &str) {
    println!("{}\n\t{}", "hints:", msg.yellow());
}

pub fn done(msg: &str) {
    println!("{} {}", "done:".green(), msg);
}

