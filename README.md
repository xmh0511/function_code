mov eax, 10 的二进制编码取决于具体的指令格式和操作数类型。在x86架构中，该指令通常对应的二进制编码为：

B8 0A 00 00 00

分解说明：

    B8 - MOV指令的操作码（将立即数移动到EAX寄存器）
    0A 00 00 00 - 32位立即数10的小端表示（十六进制0x0000000A）

在x86架构中，ret指令的二进制编码根据其具体形式有所不同：

    ‌基本ret指令‌（无操作数）的二进制编码为：

    C3

    该指令从栈顶弹出返回地址并修改EIP寄存器实现返回操作

  测试
````rust
#[unsafe(naked)]
extern "C" fn foo() {
    core::arch::naked_asm!("/* asm content */", "mov eax, 10", "ret");
}
fn main() {
    let mut f_ptr = foo as extern "C" fn();
    /*
     f_ptr：
     ----------------            --------------------------------
     | func_pointer |  --------->| B8 | 0A | 00 | 00 |  00 | C3 |
     ----------------            --------------------------------
    */
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

````
<img width="693" height="58" alt="图片" src="https://github.com/user-attachments/assets/70968a41-9934-4071-aad4-cc1e07878f09" />
