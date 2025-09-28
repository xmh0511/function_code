#[unsafe(naked)]
extern "C" fn foo() {
    core::arch::naked_asm!("/* asm content */", "mov eax, 10", "ret");
}
fn main() {
    let mut f_ptr = foo as extern "C" fn();
    let ptr_code = unsafe { *(&raw mut f_ptr as *mut *mut u8) };
    let fragments = unsafe {
        let f0 = *ptr_code;
        let f1 = *ptr_code.add(1);
        let f2 = *ptr_code.add(2);
        let f3 = *ptr_code.add(3);
        let f4 = *ptr_code.add(4);
        let f5 = *ptr_code.add(5);
        (f0, f1, f2, f3, f4, f5)
    };
    println!(
        "f0:{:02X} f1:{:02X} f2:{:02X} f3:{:02X} f4:{:02X} f5:{:02X}",
        fragments.0, fragments.1, fragments.2, fragments.3, fragments.4, fragments.5
    );
}
