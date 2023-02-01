fn main() {
    // stack_buffer_overflow();
    stack_use_after_scope();
}

// fn stack_buffer_overflow() {
//     let xs = [0, 1, 2, 3];
//     let y = unsafe { *xs.as_ptr().offset(4) };
//     println!("{y:?}");
// }

fn stack_use_after_scope() {
    static mut P: *mut usize = std::ptr::null_mut();

    unsafe {
        {
            let mut x = 0;
            P = &mut x;
        }
        std::ptr::write_volatile(P, 123);
    }
}
