pub mod markers;
pub mod table;
pub mod moves;
pub mod game_checker;

use game_checker::GameChecker;
use table::Table;

fn main() {
    let str_tablero = [
        "O X",
        "O X",
        "O X",
    ];
    let mut table = Table::from_string(&str_tablero);

    println!("{}", GameChecker::check_table(&mut table));
}
