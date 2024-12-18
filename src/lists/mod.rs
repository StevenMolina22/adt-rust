pub mod linked;
pub mod sorted;

use linked::LinkedList;
use sorted::SortedLinked;

pub fn use_sorte_l() {
    println!("----- WELCOME TO SORTED LINKED LIST -----");

    let mut list = SortedLinked::new();
    list.append(3);
    list.append(2);
    list.append(1);

    println!("Sorted linked list after appending 3, 2, 1:");

    for value in list.iter() {
        println!("Value: {}", value);
    }
    println!();
}

pub fn use_list() {
    println!("----- WELCOME TO LINKED LIST -----");

    let mut list = LinkedList::new();
    list.append(1);
    list.append(2);
    list.append(3);
    list.append(4);

    println!("Linked list after appending 1, 2, 3:");

    for value in list.iter().map(|x| x) {
        println!("Value: {}", value);
    }
    println!();
}
