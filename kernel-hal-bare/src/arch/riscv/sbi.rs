pub fn console_putchar(ch: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, ch, 0, 0);
}

pub fn console_getchar() -> usize {
    return sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0);
}

fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret: usize;
    unsafe {
        asm!(
            "ecall",
            in("a0") arg0,
            in("a1") arg1,
            in("a2") arg2,
            in("a7") which,
            lateout("a0") ret,
        );
    }
    ret
}

pub fn set_timer(stime_value: u64) {
    #[cfg(target_pointer_width = "32")]
    sbi_call(SBI_SET_TIMER, stime_value as usize, (stime_value >> 32), 0);

    #[cfg(target_pointer_width = "64")]
    sbi_call(SBI_SET_TIMER, stime_value as usize, 0, 0);
}

pub fn clear_ipi() {
    sbi_call(SBI_CLEAR_IPI, 0, 0, 0);
}

pub fn send_ipi(sipi_value: usize) {
    sbi_call(SBI_SEND_IPI, sipi_value, 0, 0);
}

const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_CLEAR_IPI: usize = 3;
const SBI_SEND_IPI: usize = 4;
const SBI_REMOTE_FENCE_I: usize = 5;
const SBI_REMOTE_SFENCE_VMA: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
const SBI_SHUTDOWN: usize = 8;
