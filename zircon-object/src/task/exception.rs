#![allow(dead_code)]

use {
    super::*, crate::ipc::*, crate::object::*, alloc::sync::Arc, alloc::vec, alloc::vec::Vec,
    core::mem::size_of, core::time::Duration, futures::channel::oneshot, futures::pin_mut,
    kernel_hal::UserContext, spin::Mutex,
};

/// Kernel-owned exception channel endpoint.
pub struct Exceptionate {
    type_: ExceptionChannelType,
    inner: Mutex<ExceptionateInner>,
}

struct ExceptionateInner {
    channel: Option<Arc<Channel>>,
    thread_rights: Rights,
    process_rights: Rights,
}

impl Exceptionate {
    pub fn new(type_: ExceptionChannelType) -> Arc<Self> {
        Arc::new(Exceptionate {
            type_,
            inner: Mutex::new(ExceptionateInner {
                channel: None,
                thread_rights: Rights::empty(),
                process_rights: Rights::empty(),
            }),
        })
    }

    pub fn create_channel(&self) -> ZxResult<Arc<Channel>> {
        let mut inner = self.inner.lock();
        if let Some(channel) = inner.channel.as_ref() {
            if channel.peer().is_ok() {
                // already has a valid channel
                return Err(ZxError::ALREADY_BOUND);
            }
        }
        let (sender, receiver) = Channel::create();
        inner.channel.replace(sender);
        Ok(receiver)
    }

    fn send_exception(&self, exception: &Arc<Exception>) -> ZxResult<oneshot::Receiver<()>> {
        let mut inner = self.inner.lock();
        let channel = inner.channel.as_ref().ok_or(ZxError::NEXT)?;
        let info = ExceptionInfo {
            tid: exception.thread.id(),
            pid: exception.thread.proc().id(),
            type_: exception.type_,
            padding: Default::default(),
        };
        let (sender, receiver) = oneshot::channel::<()>();
        let object = ExceptionObject::create(exception.clone(), sender);
        let handle = Handle::new(object, Rights::DEFAULT_EXCEPTION);
        let msg = MessagePacket {
            data: info.pack(),
            handles: vec![handle],
        };
        channel.write(msg).map_err(|err| {
            if err == ZxError::PEER_CLOSED {
                inner.channel.take();
                return ZxError::NEXT;
            }
            err
        })?;
        Ok(receiver)
    }
}

#[repr(C)]
pub struct ExceptionInfo {
    pub tid: KoID,
    pub pid: KoID,
    pub type_: ExceptionType,
    pub padding: u32,
}

impl ExceptionInfo {
    #[allow(unsafe_code)]
    pub fn pack(&self) -> Vec<u8> {
        let buf: [u8; size_of::<ExceptionInfo>()] = unsafe { core::mem::transmute_copy(self) };
        Vec::from(buf)
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct ExceptionHeader {
    pub size: u32,
    pub type_: ExceptionType,
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
#[derive(Default, Clone)]
pub struct ExceptionContext {
    pub vector: u64,
    pub err_code: u64,
    pub cr2: u64,
}

#[cfg(target_arch = "aarch64")]
#[repr(C)]
#[derive(Default, Clone)]
pub struct ExceptionContext {
    pub esr: u32,
    pub padding1: u32,
    pub far: u64,
    pub padding2: u64,
}

impl ExceptionContext {
    #[cfg(target_arch = "x86_64")]
    fn from_user_context(cx: &UserContext) -> Self {
        ExceptionContext {
            vector: cx.trap_num as u64,
            err_code: cx.error_code as u64,
            cr2: kernel_hal::fetch_fault_vaddr() as u64,
        }
    }
    #[cfg(target_arch = "aarch64")]
    fn from_user_context(_cx: &UserContext) -> Self {
        unimplemented!()
    }
}

#[repr(C)]
#[derive(Clone)]
pub struct ExceptionReport {
    pub header: ExceptionHeader,
    pub context: ExceptionContext,
}

impl ExceptionReport {
    fn new(type_: ExceptionType, cx: Option<&UserContext>) -> Self {
        ExceptionReport {
            header: ExceptionHeader {
                type_,
                size: core::mem::size_of::<ExceptionReport>() as u32,
            },
            context: cx
                .map(ExceptionContext::from_user_context)
                .unwrap_or_default(),
        }
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum ExceptionType {
    General = 0x008,
    FatalPageFault = 0x108,
    UndefinedInstruction = 0x208,
    SoftwareBreakpoint = 0x308,
    HardwareBreakpoint = 0x408,
    UnalignedAccess = 0x508,
    // exceptions generated by kernel instead of the hardware
    Synth = 0x8000,
    ThreadStarting = 0x8008,
    ThreadExiting = 0x8108,
    PolicyError = 0x8208,
    ProcessStarting = 0x8308,
}

#[repr(u32)]
#[derive(Copy, Clone,PartialEq)]
pub enum ExceptionChannelType {
    None = 0,
    Debugger = 1,
    Thread = 2,
    Process = 3,
    Job = 4,
    JobDebugger = 5,
}

/// This will be transmitted to registered exception handlers in userspace
/// and provides them with exception state and control functionality.
/// We do not send exception directly since it's hard to figure out
/// when will the handle close.
pub struct ExceptionObject {
    base: KObjectBase,
    exception: Arc<Exception>,
    close_signal: Option<oneshot::Sender<()>>,
}

impl_kobject!(ExceptionObject);

impl ExceptionObject {
    fn create(exception: Arc<Exception>, close_signal: oneshot::Sender<()>) -> Arc<Self> {
        Arc::new(ExceptionObject {
            base: KObjectBase::new(),
            exception,
            close_signal: Some(close_signal),
        })
    }
    pub fn get_exception(&self) -> &Arc<Exception> {
        &self.exception
    }
}

impl Drop for ExceptionObject {
    fn drop(&mut self) {
        self.close_signal
            .take()
            .and_then(|signal| signal.send(()).ok());
    }
}

/// An Exception represents a single currently-active exception.
pub struct Exception {
    thread: Arc<Thread>,
    type_: ExceptionType,
    report: ExceptionReport,
    inner: Mutex<ExceptionInner>,
}

struct ExceptionInner {
    current_channel_type: ExceptionChannelType,
    // Task rights copied from Exceptionate
    thread_rights: Rights,
    process_rights: Rights,
    handled: bool,
    second_chance: bool,
}

impl Exception {
    pub fn create(
        thread: Arc<Thread>,
        type_: ExceptionType,
        cx: Option<&UserContext>,
    ) -> Arc<Self> {
        Arc::new(Exception {
            thread,
            type_,
            report: ExceptionReport::new(type_, cx),
            inner: Mutex::new(ExceptionInner {
                current_channel_type: ExceptionChannelType::None,
                thread_rights: Rights::DEFAULT_THREAD,
                process_rights: Rights::DEFAULT_PROCESS,
                handled: false,
                second_chance: false,
            }),
        })
    }
    /// Handle the exception. The return value indicate if the thread is exited after this.
    /// Note that it's possible that this may returns before exception was send to any exception channel
    /// This happens only when the thread is killed before we send the exception
    pub async fn handle(self: &Arc<Self>) -> bool {
        self.thread.set_exception(Some(self.clone()));
        let future = self.handle_internal();
        pin_mut!(future);
        let result: ZxResult = self
            .thread
            .blocking_run(
                future,
                ThreadState::BlockedException,
                Duration::from_nanos(u64::max_value()),
            )
            .await;
        self.thread.set_exception(None);
        if let Err(err) = result {
            #[allow(clippy::if_same_then_else)]
            if err == ZxError::STOP {
                // We are killed
                self.thread.exit();
                return false;
            } else if err == ZxError::NEXT {
                // Nobody handled the exception, kill myself
                self.thread.exit();
                // TODO: In zircon the process is also killed, but for now don't do it
                // since this may break the core-test
                return false;
            }
        }
        self.thread.exit();
        false
    }
    async fn handle_internal(self: &Arc<Self>) -> ZxResult {
        for exceptionate in ExceptionateIterator::new(self) {
            let closed = match exceptionate.send_exception(self) {
                Ok(receiver) => receiver,
                // This channel is not available now!
                Err(ZxError::NEXT) => continue,
                Err(err) => return Err(err),
            };
            self.inner.lock().current_channel_type = exceptionate.type_;
            // If this error, the sender is dropped, and the handle should also be closed.
            closed.await.ok();
            let handled = {
                let mut inner = self.inner.lock();
                inner.current_channel_type = ExceptionChannelType::None;
                inner.handled
            };
            if handled {
                return Ok(());
            }
        }
        Err(ZxError::NEXT)
    }

    pub fn get_thread_and_rights(&self) -> (Arc<Thread>, Rights) {
        (self.thread.clone(), self.inner.lock().thread_rights)
    }

    pub fn get_process_and_rights(&self) -> (Arc<Process>, Rights) {
        (self.thread.proc().clone(), self.inner.lock().process_rights)
    }

    pub fn get_current_channel_type(&self) -> ExceptionChannelType {
        self.inner.lock().current_channel_type
    }

    pub fn get_report(&self) -> ExceptionReport {
        self.report.clone()
    }

    pub fn get_state(&self) -> u32 {
        self.inner.lock().handled as u32
    }

    pub fn set_state(&self, state: u32) {
        self.inner.lock().handled = state == 1;
    }

    pub fn get_strategy(&self) -> u32 {
        self.inner.lock().second_chance as u32
    }

    pub fn set_strategy(&self, strategy: u32) -> ZxResult {
        let mut inner = self.inner.lock();
        match inner.current_channel_type {
            ExceptionChannelType::Debugger | ExceptionChannelType::JobDebugger => {
                inner.second_chance = strategy == 1;
                Ok(())
            }
            _ => Err(ZxError::BAD_STATE),
        }
    }
}

/// An iterator used to find Exceptionates used while handling the exception
/// We can use rust generator instead here but that is somehow not stable
/// Exception handlers are tried in the following order:
/// - debugger (first process, then job, then its parent job, and so on)
/// - thread
/// - process
/// - debugger (in dealing with a second-chance exception)
/// - job (first owning job, then its parent job, and so on up to root job)
struct ExceptionateIterator<'a> {
    exception: &'a Exception,
    state: ExceptionateIteratorState,
}

/// The state used in ExceptionateIterator.
/// Name of optional is what to consider next
enum ExceptionateIteratorState {
    Thread,
    Process,
    Job(Arc<Job>),
    Finished,
}

impl<'a> ExceptionateIterator<'a> {
    fn new(exception: &'a Exception) -> Self {
        ExceptionateIterator {
            exception,
            state: ExceptionateIteratorState::Thread,
        }
    }
}

impl<'a> Iterator for ExceptionateIterator<'a> {
    type Item = Arc<Exceptionate>;
    fn next(&mut self) -> Option<Self::Item> {
        match &self.state {
            ExceptionateIteratorState::Thread => {
                self.state = ExceptionateIteratorState::Process;
                Some(self.exception.thread.get_exceptionate())
            }
            ExceptionateIteratorState::Process => {
                let proc = self.exception.thread.proc();
                self.state = ExceptionateIteratorState::Job(proc.job());
                Some(proc.get_exceptionate())
            }
            ExceptionateIteratorState::Job(job) => {
                let parent = job.parent();
                let result = job.get_exceptionate();
                self.state = parent.map_or(
                    ExceptionateIteratorState::Finished,
                    ExceptionateIteratorState::Job,
                );
                Some(result)
            }
            ExceptionateIteratorState::Finished => None,
        }
    }
}
