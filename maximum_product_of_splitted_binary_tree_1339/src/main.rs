use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let rc = Rc::new(RefCell::new(10));

    // Chama Rc::as_ptr (Endereço do RefCell na Heap)
    let ptr1 = rc.as_ptr(); 
    
    // Chama RefCell::as_ptr (Endereço do i32 dentro do RefCell)
    // Aqui o acesso explícito é obrigatório para desambiguar
    let ptr2 = (*rc).as_ptr(); 

    println!("Ptr do RC:      {:?}", ptr1);
    println!("Ptr do RefCell: {:?}", ptr2);
    
    let rc = &rc;

    // Chama Rc::as_ptr (Endereço do RefCell na Heap)
    let ptr1 = rc.as_ptr(); 
    
    // Chama RefCell::as_ptr (Endereço do i32 dentro do RefCell)
    // Aqui o acesso explícito é obrigatório para desambiguar
    let ptr2 = (*rc).as_ptr(); 

    println!("Ptr do RC:      {:?}", ptr1);
    println!("Ptr do RefCell: {:?}", ptr2);
}