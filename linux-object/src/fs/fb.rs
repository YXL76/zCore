//! Implement INode for FbINode

use super::ioctl::*;
use core::any::Any;
use kernel_hal::{FbFixScreeninfo, FbVarScreeninfo, FRAME_BUFFER};
use rcore_fs::vfs::*;

/// framebuffer INode struct
#[derive(Clone)]
pub struct FbINode;

impl FbINode {
    /// create a framebuffer INode
    pub fn new() -> Self {
        FbINode {}
    }
}

impl INode for FbINode {
    #[allow(unsafe_code)]
    fn read_at(&self, offset: usize, buf: &mut [u8]) -> Result<usize> {
        if let Some(fb) = FRAME_BUFFER.read().as_ref() {
            if offset >= fb.screen_size {
                return Ok(0);
            }
            let len = buf.len().min(fb.screen_size - offset);
            let data =
                unsafe { core::slice::from_raw_parts((fb.vaddr + offset) as *const u8, len) };
            buf[..len].copy_from_slice(&data);
            Ok(len)
        } else {
            Err(FsError::NoDevice)
        }
    }

    #[allow(unsafe_code)]
    fn write_at(&self, offset: usize, buf: &[u8]) -> Result<usize> {
        if let Some(fb) = FRAME_BUFFER.write().as_mut() {
            if offset > fb.screen_size {
                return Err(FsError::NoDeviceSpace);
            }
            let len = buf.len().min(fb.screen_size - offset);
            let data =
                unsafe { core::slice::from_raw_parts_mut((fb.vaddr + offset) as *mut u8, len) };
            data.copy_from_slice(&buf[..len]);
            Ok(len)
        } else {
            Err(FsError::NoDevice)
        }
    }

    fn poll(&self) -> Result<PollStatus> {
        Ok(PollStatus {
            read: true,
            write: false,
            error: false,
        })
    }

    #[allow(unsafe_code)]
    fn io_control(&self, cmd: u32, data: usize) -> Result<usize> {
        match cmd as usize {
            FBIOGET_FSCREENINFO => {
                if let Some(fb) = FRAME_BUFFER.read().as_ref() {
                    let fix_info = unsafe { &mut *(data as *mut FbFixScreeninfo) };
                    fix_info.fill_from(&fb);
                }
                Ok(0)
            }
            FBIOGET_VSCREENINFO => {
                if let Some(fb) = FRAME_BUFFER.read().as_ref() {
                    let var_info = unsafe { &mut *(data as *mut FbVarScreeninfo) };
                    var_info.fill_from(&fb);
                }
                Ok(0)
            }
            _ => Err(FsError::NotSupported),
        }
    }

    fn metadata(&self) -> Result<Metadata> {
        Ok(Metadata {
            dev: 5,
            inode: 662,
            size: 0,
            blk_size: 4096,
            blocks: 0,
            atime: Timespec { sec: 0, nsec: 0 },
            mtime: Timespec { sec: 0, nsec: 0 },
            ctime: Timespec { sec: 0, nsec: 0 },
            type_: FileType::CharDevice,
            mode: 0o666, // 0o660
            nlinks: 1,
            uid: 0,
            gid: 0,
            rdev: make_rdev(29, 0),
        })
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }
}
