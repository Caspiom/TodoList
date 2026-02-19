use std::collections::LinkedList;
use std::io;

#[derive(Debug)] 
pub struct Entry{
    pub task: String,
    pub done: bool,
}

#[derive(Debug)]
pub struct TodoList {
    pub tasks: LinkedList<Entry>,
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

fn add(my_list: &mut TodoList) {
    println!("Task name:");

    let mut input_user = String::new();

    io::stdin()
    .read_line(&mut input_user)
    .expect("Erro while trying to read what was typed");

    let clean_text = input_user.trim().to_string();

    let new_task = Entry::new_todo(clean_text);

    my_list.tasks.push_back(new_task);
}


fn list(my_list: &TodoList) { 
    println!("Your To Do List");
    println!("{:#?}", my_list);
}


fn main() {

 let mut my_list = TodoList {
    tasks: LinkedList::new(),
};
    add(&mut my_list);

    list(&my_list);

}