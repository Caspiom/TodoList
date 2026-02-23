use std::collections::LinkedList;
use std::io;
extern crate dialoguer;
use dialoguer::{theme::ColorfulTheme, Select};

#[derive(Debug)] 
pub struct Entry{
    pub task: String,
    pub done: bool,
}

#[derive(Debug)]
pub struct TodoList {
    tasks: LinkedList<Entry>,
}

impl TodoList{
    pub(crate) fn new() -> TodoList{
        TodoList{
            tasks: LinkedList::new(),
        }
    }
}

impl Entry{

    pub fn new_todo(text: String) -> Entry{
        Entry{
            task: text,
            done: false,
        }
    }
 

    pub fn mark_complet(&mut self){
        self.done = true;
    }    
}

pub(crate) fn add(my_list: &mut TodoList) {
    println!("Task name:");

    let mut input_user = String::new();

    io::stdin()
    .read_line(&mut input_user)
    .expect("Erro while trying to read what was typed");

    let clean_text = input_user.trim().to_string();

    let new_task = Entry::new_todo(clean_text);

    my_list.tasks.push_back(new_task);
}


pub(crate) fn list(my_list: &TodoList) {
    
    println!("===================");
    println!("Your To Do List");

    for (indice, tarefa) in my_list.tasks.iter().enumerate() {

        let status = if tarefa.done{
            "[x]"
        }else{
            "[ ]"
        };

    
    println!("{} {} {}", indice + 1, status, tarefa.task);
    }

    println!("===================");
}


pub(crate) fn mark_done(my_list: &mut TodoList){


    if my_list.tasks.is_empty(){
        println!("Empty List");
        return;
    }

    let names_task: Vec<String> = my_list.tasks.iter().map(|TaskName|{
        let status = if TaskName.done { "[X]"} else {"[ ]"};
        format!("{} {}", TaskName.task, status)
    } 
    ).collect();


    println!("\n--- CLI Task Manager ---");
    let selecao = Select::with_theme(&ColorfulTheme::default())
    .with_prompt("Select one to mark as done:")
    .default(0)
    .items(&names_task)
    .interact()
    .unwrap();

    if let Some(chosed_task) = my_list.tasks.iter_mut().nth(selecao){
        chosed_task.mark_complet();
        println!("{} marked as completed", chosed_task.task);
    }
}

pub(crate) fn done(my_list: &mut TodoList) {
    
    println!("===================");
    println!("Your To Do List");

    for (indice, tarefa) in my_list.tasks.iter().enumerate() {

        let status = if tarefa.done{
            "[x]"
        }else{
            "[ ]"
        };

    
    println!("{} {} {}", indice + 1, status, tarefa.task, );
    }

    let mut input_user = String::new();
    

    println!("===================") 
}