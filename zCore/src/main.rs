#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(panic_info_message)]
#![deny(unused_must_use)]
#![feature(global_asm)]
/*
#![deny(warnings)] // comment this on develop
*/

extern crate alloc;
#[macro_use]
extern crate log;

#[cfg(target_arch = "riscv64")]
extern crate rlibc;
#[cfg(target_arch = "x86_64")]
extern crate rlibc_opt; //Only for x86_64

#[macro_use]
mod logging;
mod lang;
mod memory;

#[cfg(target_arch = "x86_64")]
use rboot::BootInfo;

#[cfg(target_arch = "riscv64")]
use kernel_hal_bare::{
    phys_to_virt, remap_the_kernel,
    virtio::{BlockDriverWrapper, EventRepr, BLK_DRIVERS, GPU_DRIVERS, INPUT_DRIVERS},
};

use alloc::{string::String, vec::Vec};

#[cfg(target_arch = "riscv64")]
global_asm!(include_str!("arch/riscv/boot/entry64.asm"));

#[cfg(target_arch = "x86_64")]
#[no_mangle]
pub extern "C" fn _start(boot_info: &BootInfo) -> ! {
    logging::init(get_value(boot_info.cmdline, "LOG").unwrap_or(""));
    memory::init_heap();
    memory::init_frame_allocator(boot_info);

    let (width, height) = boot_info.graphic_info.mode.resolution();
    let fb_addr = boot_info.graphic_info.fb_addr as usize;
    let fb_size = boot_info.graphic_info.fb_size as usize;
    kernel_hal_bare::init_framebuffer(width as u32, height as u32, fb_addr, fb_size);

    trace!("{:#x?}", boot_info);

    kernel_hal_bare::init(kernel_hal_bare::Config {
        acpi_rsdp: boot_info.acpi2_rsdp_addr,
        smbios: boot_info.smbios_addr,
        ap_fn: run,
    });

    let ramfs_data = unsafe {
        core::slice::from_raw_parts_mut(
            (boot_info.initramfs_addr + boot_info.physical_memory_offset) as *mut u8,
            boot_info.initramfs_size as usize,
        )
    };
    main(ramfs_data, boot_info.cmdline);
}

#[cfg(feature = "zircon")]
fn main(ramfs_data: &[u8], cmdline: &str) -> ! {
    use zircon_loader::{run_userboot, Images};
    let images = Images::<&[u8]> {
        userboot: include_bytes!("../../prebuilt/zircon/x64/userboot.so"),
        vdso: include_bytes!("../../prebuilt/zircon/x64/libzircon.so"),
        zbi: ramfs_data,
    };
    let _proc = run_userboot(&images, cmdline);
    run();
}

#[cfg(target_arch = "riscv64")]
#[no_mangle]
pub extern "C" fn rust_main(hartid: usize, device_tree_paddr: usize) -> ! {
    let device_tree_vaddr = phys_to_virt(device_tree_paddr);
    unsafe {
        memory::clear_bss();
    }

    logging::init("info");
    memory::init_heap();
    memory::init_frame_allocator();
    remap_the_kernel(device_tree_vaddr);

    kernel_hal_bare::init(kernel_hal_bare::Config {
        dtb: device_tree_vaddr,
    });

    let cmdline = kernel_hal_bare::cmdline();
    info!("cmdline: {:?}", cmdline);
    // logging::init(get_value(cmdline, "LOG").unwrap_or(""));

    let gpu = GPU_DRIVERS
        .read()
        .iter()
        .next()
        .expect("Gpu device not found")
        .clone();
    let (width, height) = gpu.resolution();
    let (fb_vaddr, fb_size) = gpu.setup_framebuffer();
    kernel_hal_bare::init_framebuffer(width, height, fb_vaddr, fb_size);

    // 正常由bootloader载入文件系统镜像到内存, 这里不用，而使用后面的virtio
    main(&mut [], &cmdline);
}

#[cfg(feature = "linux")]
fn get_rootproc(cmdline: &str) -> Vec<String> {
    use alloc::vec;
    if let Some(value) = get_value(cmdline, "ROOTPROC") {
        let mut iter = value.trim().splitn(2, '?');
        let k1 = iter.next().expect("failed to parse k1");
        let v1 = iter.next().expect("failed to parse v1");
        if v1 == "" {
            return vec![k1.into()];
        } else {
            return vec![k1.into(), v1.into()];
        }
    }
    vec!["/bin/busybox".into(), "sh".into()]
}

#[cfg(feature = "linux")]
fn main(ramfs_data: &'static mut [u8], cmdline: &str) -> ! {
    use alloc::boxed::Box;
    use alloc::sync::Arc;
    use alloc::vec;

    #[cfg(target_arch = "x86_64")]
    use linux_object::fs::MemBuf;
    use linux_object::fs::STDIN;

    kernel_hal_bare::serial_set_callback(Box::new({
        move || {
            let mut buffer = [0; 255];
            let len = kernel_hal_bare::serial_read(&mut buffer);
            for c in &buffer[..len] {
                STDIN.push((*c).into());
                // kernel_hal_bare::serial_write(alloc::format!("{}", *c as char).as_str());
            }
            false
        }
    }));

    let args: Vec<String> = vec!["libc-test/src/functional/argv.exe".into()];
    // let args: Vec<String> = get_rootproc(cmdline);
    let envs: Vec<String> = vec!["PATH=/usr/sbin:/usr/bin:/sbin:/bin".into()];

    #[cfg(target_arch = "x86_64")]
    let device = Arc::new(MemBuf::new(ramfs_data));
    #[cfg(target_arch = "riscv64")]
    let device = {
        let driver = BlockDriverWrapper(
            BLK_DRIVERS
                .read()
                .iter()
                .next()
                .expect("Block device not found")
                .clone(),
        );
        Arc::new(rcore_fs::dev::block_cache::BlockCache::new(driver, 0x100))
    };

    info!("Opening the rootfs ...");
    // 输入类型: Arc<Device>
    let rootfs =
        rcore_fs_sfs::SimpleFileSystem::open(device).expect("failed to open device SimpleFS");

    // fat32
    // let img_file = File::open("fat.img")?;
    // let fs = fatfs::FileSystem::new(img_file, fatfs::FsOptions::new())?;

    let _proc = linux_loader::run(args, envs, rootfs);
    info!("linux_loader is complete");

    run();
}

fn run() -> ! {
    loop {
        executor::run_until_idle();
        #[cfg(target_arch = "x86_64")]
        {
            x86_64::instructions::interrupts::enable_and_hlt();
            x86_64::instructions::interrupts::disable();
        }
        #[cfg(target_arch = "riscv64")]
        kernel_hal_bare::interrupt::wait_for_interrupt();
    }
}

fn get_value<'a>(cmdline: &'a str, key: &str) -> Option<&'a str> {
    for opt in cmdline.split(':') {
        if opt.is_empty() {
            continue;
        }
        // parse 'key=value'
        let mut iter = opt.trim().splitn(2, '=');
        let key0 = iter.next().expect("failed to parse key");
        let value = iter.next().expect("failed to parse value");
        if key == key0 {
            return Some(value);
        }
    }
    None
}

/* fn init_framebuffer(boot_info: &BootInfo) {
    let (width, height) = boot_info.graphic_info.mode.resolution();
    let fb_addr = boot_info.graphic_info.fb_addr as usize;
    kernel_hal_bare::init_framebuffer(width as u32, height as u32, fb_addr);
} */
