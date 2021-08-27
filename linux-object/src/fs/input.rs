//! Implement INode for FbINode

use super::super::time::TimeVal;
use alloc::collections::VecDeque;
use core::any::Any;
use lazy_static::lazy_static;
use rcore_fs::vfs::*;
use spin::Mutex;

/// framebuffer INode struct
#[derive(Clone)]
pub struct InputEventINode;

impl InputEventINode {
    /// create a input event INode
    pub fn new() -> Self {
        InputEventINode {}
    }
}

impl INode for InputEventINode {
    #[allow(unsafe_code)]
    fn read_at(&self, _offset: usize, buf: &mut [u8]) -> Result<usize> {
        let event = {
            let mut queue = INPUT_EVENT.lock();
            if queue.len() == 0 {
                return Ok(0);
            };
            queue.pop_front().unwrap_or(InputEvent::new(0, 0, 0))
        };
        let event: [u8; core::mem::size_of::<InputEvent>()] =
            unsafe { core::mem::transmute(event) };
        let len = event.len().min(buf.len());
        buf.copy_from_slice(&event[..len]);
        Ok(len)
    }

    fn write_at(&self, _offset: usize, _buf: &[u8]) -> Result<usize> {
        Err(FsError::NotSupported)
    }

    fn poll(&self) -> Result<PollStatus> {
        Ok(PollStatus {
            read: true,
            write: false,
            error: false,
        })
    }

    fn metadata(&self) -> Result<Metadata> {
        Ok(Metadata {
            dev: 5,
            inode: 0,
            size: 0,
            blk_size: 0,
            blocks: 0,
            atime: Timespec { sec: 0, nsec: 0 },
            mtime: Timespec { sec: 0, nsec: 0 },
            ctime: Timespec { sec: 0, nsec: 0 },
            type_: FileType::CharDevice,
            mode: 0o666,
            nlinks: 1,
            uid: 0,
            gid: 0,
            rdev: make_rdev(13, 64),
        })
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }
}

lazy_static! {
    ///
    pub static ref INPUT_EVENT: Mutex<VecDeque<InputEvent>> = Mutex::new(VecDeque::new());
}

#[repr(C)]
///
pub struct InputEvent {
    time: TimeVal,
    /// event type
    pub type_: u16,
    code: u16,
    value: i32,
}

impl InputEvent {
    ///
    pub fn new(type_: u16, code: u16, value: i32) -> Self {
        InputEvent {
            time: TimeVal::now(),
            type_,
            code,
            value,
        }
    }
}
