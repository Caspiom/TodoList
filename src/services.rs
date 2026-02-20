use std::collections::LinkedList;
use std::io;

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




fn main() {

 let mut my_list = TodoList {
    tasks: LinkedList::new(),
};
    add(&mut my_list);

    list(&my_list);

}