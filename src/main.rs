mod services;
extern crate dialoguer;
use services::{TodoList, add, list};
use std::collections::LinkedList;
use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
 let mut my_list = TodoList::new();

 let options = &["Add", "List", "Exit"];
 loop {

    println!("\n--- CLI Task Manager ---");
    let selecao = Select::with_theme(&ColorfulTheme::default())
    .with_prompt("========")
    .default(0)
    .items(options)
    .interact()
    .unwrap();

    match selecao{
        0 => {
            println!("add");
            add(&mut my_list);
        }
        1 => {
            println!("List");
            list(&my_list);
        }
        2 => {
            println!("Exit");
            break;
        }
        _ => println!("Invalid Option"),
    }
 }


}