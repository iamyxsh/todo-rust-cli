use colored::{ColoredString, Colorize};
use postgres::Row;

pub fn print_result(result: ColoredString) {
    println!("");
    println!("{}", result);
    println!("");
}

pub fn print_todo(todos: Vec<Row>) {
    println!("");
    println!(
        "{}    {}     {}",
        "ID".yellow(),
        "Completed".green(),
        "Task".red()
    );
    println!("");
    println!("");
    for row in todos.into_iter().rev() {
        let ind: i32 = row.get(0);
        let task: String = row.get(1);
        let completed: String = if row.get(2) {
            "x".to_string()
        } else {
            " ".to_string()
        };
        println!(
            "{}        [{}]        {}",
            (ind).to_string().yellow(),
            completed.green(),
            task.red()
        );
        println!("");
    }
}

pub fn return_result(tx: bool) -> ColoredString {
    if tx {
        return "Successfully added".to_string().green();
    } else {
        return "Something went wrong".to_string().red();
    }
}
