mod services;
extern crate dialoguer;
use services::{TodoList, add, list, mark_done, remove_task, save_to_file};
use std::collections::LinkedList;
use dialoguer::{theme::ColorfulTheme, Select};

fn main() {
 let mut my_list = TodoList::new();

 let options = &["Add", "List", "Mark as done", "Remove", "Save" ,"Exit"];
 loop {

    println!("\n--- CLI Task Manager ---");
    let selecao = Select::with_theme(&ColorfulTheme::default())
    .with_prompt("Select one option:")
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
            println!("Mark as done");
            mark_done(&mut my_list);
        }
        3 => {
            println!("Remove");
            remove_task(&mut my_list);
        }
        4 => {
            println!("Save");
            save_to_file(&mut my_list);
        }
        5 => {
            println!("Exit");
            save_to_file(&mut my_list);
            break;
        }
        _ => println!("Invalid Option"),
    }
 }


}