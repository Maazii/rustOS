use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use crate::println;
use lazy_static::lazy_static;

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame){
    println!("Exception! Breakpoint\n{:#?}",stack_frame);
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    }
}

pub fn init_idt(){
    IDT.load();
}