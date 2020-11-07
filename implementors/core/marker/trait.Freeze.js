(function() {var implementors = {};
implementors["kernel_hal"] = [{"text":"impl Freeze for VectorRegs","synthetic":true,"types":[]},{"text":"impl Freeze for U128","synthetic":true,"types":[]},{"text":"impl Freeze for Thread","synthetic":true,"types":[]},{"text":"impl Freeze for PageTable","synthetic":true,"types":[]},{"text":"impl Freeze for PhysFrame","synthetic":true,"types":[]},{"text":"impl Freeze for InterruptManager","synthetic":true,"types":[]},{"text":"impl Freeze for SleepFuture","synthetic":true,"types":[]},{"text":"impl Freeze for SerialFuture","synthetic":true,"types":[]},{"text":"impl Freeze for MMUFlags","synthetic":true,"types":[]},{"text":"impl Freeze for CachePolicy","synthetic":true,"types":[]},{"text":"impl&lt;T, P&gt; Freeze for UserPtr&lt;T, P&gt;","synthetic":true,"types":[]},{"text":"impl&lt;P&gt; Freeze for IoVec&lt;P&gt;","synthetic":true,"types":[]},{"text":"impl&lt;P&gt; Freeze for IoVecs&lt;P&gt;","synthetic":true,"types":[]},{"text":"impl Freeze for In","synthetic":true,"types":[]},{"text":"impl Freeze for Out","synthetic":true,"types":[]},{"text":"impl Freeze for InOut","synthetic":true,"types":[]},{"text":"impl Freeze for Error","synthetic":true,"types":[]},{"text":"impl Freeze for VdsoConstants","synthetic":true,"types":[]},{"text":"impl Freeze for Features","synthetic":true,"types":[]},{"text":"impl Freeze for VersionString","synthetic":true,"types":[]}];
implementors["kernel_hal_unix"] = [{"text":"impl Freeze for Thread","synthetic":true,"types":[]},{"text":"impl Freeze for PageTable","synthetic":true,"types":[]},{"text":"impl Freeze for PhysFrame","synthetic":true,"types":[]}];
implementors["linux_object"] = [{"text":"impl Freeze for LxError","synthetic":true,"types":[]},{"text":"impl !Freeze for MemBuf","synthetic":true,"types":[]},{"text":"impl Freeze for FcntlFlags","synthetic":true,"types":[]},{"text":"impl Freeze for FileFlags","synthetic":true,"types":[]},{"text":"impl !Freeze for File","synthetic":true,"types":[]},{"text":"impl Freeze for OpenOptions","synthetic":true,"types":[]},{"text":"impl Freeze for PipeData","synthetic":true,"types":[]},{"text":"impl Freeze for Pipe","synthetic":true,"types":[]},{"text":"impl Freeze for Pseudo","synthetic":true,"types":[]},{"text":"impl Freeze for RandomINodeData","synthetic":true,"types":[]},{"text":"impl Freeze for RandomINode","synthetic":true,"types":[]},{"text":"impl Freeze for STDIN","synthetic":true,"types":[]},{"text":"impl Freeze for STDOUT","synthetic":true,"types":[]},{"text":"impl !Freeze for Stdin","synthetic":true,"types":[]},{"text":"impl Freeze for Stdout","synthetic":true,"types":[]},{"text":"impl Freeze for FileDesc","synthetic":true,"types":[]},{"text":"impl Freeze for SeekFrom","synthetic":true,"types":[]},{"text":"impl Freeze for PipeEnd","synthetic":true,"types":[]},{"text":"impl Freeze for SemidDs","synthetic":true,"types":[]},{"text":"impl !Freeze for SemArray","synthetic":true,"types":[]},{"text":"impl Freeze for ShmidDs","synthetic":true,"types":[]},{"text":"impl Freeze for ShmIdentifier","synthetic":true,"types":[]},{"text":"impl !Freeze for ShmGuard","synthetic":true,"types":[]},{"text":"impl Freeze for SemProc","synthetic":true,"types":[]},{"text":"impl Freeze for ShmProc","synthetic":true,"types":[]},{"text":"impl Freeze for IpcPerm","synthetic":true,"types":[]},{"text":"impl Freeze for LinuxElfLoader","synthetic":true,"types":[]},{"text":"impl !Freeze for LinuxProcess","synthetic":true,"types":[]},{"text":"impl Freeze for RLimit","synthetic":true,"types":[]},{"text":"impl Freeze for Sigset","synthetic":true,"types":[]},{"text":"impl Freeze for SignalAction","synthetic":true,"types":[]},{"text":"impl Freeze for SigInfo","synthetic":true,"types":[]},{"text":"impl Freeze for SignalActionFlags","synthetic":true,"types":[]},{"text":"impl Freeze for MachineContext","synthetic":true,"types":[]},{"text":"impl Freeze for SignalUserContext","synthetic":true,"types":[]},{"text":"impl Freeze for SignalFrame","synthetic":true,"types":[]},{"text":"impl Freeze for SignalStackFlags","synthetic":true,"types":[]},{"text":"impl Freeze for SignalStack","synthetic":true,"types":[]},{"text":"impl Freeze for SiginfoFields","synthetic":true,"types":[]},{"text":"impl Freeze for SignalCode","synthetic":true,"types":[]},{"text":"impl Freeze for Signal","synthetic":true,"types":[]},{"text":"impl Freeze for Event","synthetic":true,"types":[]},{"text":"impl Freeze for EventBus","synthetic":true,"types":[]},{"text":"impl Freeze for Semaphore","synthetic":true,"types":[]},{"text":"impl&lt;'a&gt; Freeze for SemaphoreGuard&lt;'a&gt;","synthetic":true,"types":[]},{"text":"impl Freeze for LinuxThread","synthetic":true,"types":[]},{"text":"impl Freeze for TimeSpec","synthetic":true,"types":[]},{"text":"impl Freeze for TimeVal","synthetic":true,"types":[]},{"text":"impl Freeze for RUsage","synthetic":true,"types":[]},{"text":"impl Freeze for Tms","synthetic":true,"types":[]}];
implementors["linux_syscall"] = [{"text":"impl&lt;'a&gt; Freeze for Syscall&lt;'a&gt;","synthetic":true,"types":[]}];
implementors["zircon_loader"] = [{"text":"impl&lt;T&gt; Freeze for Images&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: Freeze,&nbsp;</span>","synthetic":true,"types":[]}];
implementors["zircon_object"] = [{"text":"impl Freeze for ZxError","synthetic":true,"types":[]},{"text":"impl !Freeze for DebugLog","synthetic":true,"types":[]},{"text":"impl Freeze for Severity","synthetic":true,"types":[]},{"text":"impl !Freeze for BusTransactionInitiator","synthetic":true,"types":[]},{"text":"impl Freeze for BtiInfo","synthetic":true,"types":[]},{"text":"impl !Freeze for Interrupt","synthetic":true,"types":[]},{"text":"impl Freeze for InterruptFlags","synthetic":true,"types":[]},{"text":"impl Freeze for InterruptOptions","synthetic":true,"types":[]},{"text":"impl !Freeze for Iommu","synthetic":true,"types":[]},{"text":"impl Freeze for IommuPerms","synthetic":true,"types":[]},{"text":"impl !Freeze for MmioPcieAddressProvider","synthetic":true,"types":[]},{"text":"impl !Freeze for PCIeBusDriver","synthetic":true,"types":[]},{"text":"impl Freeze for PcieDeviceInfo","synthetic":true,"types":[]},{"text":"impl !Freeze for PcieDeviceKObject","synthetic":true,"types":[]},{"text":"impl Freeze for PioPcieAddressProvider","synthetic":true,"types":[]},{"text":"impl Freeze for PciEcamRegion","synthetic":true,"types":[]},{"text":"impl Freeze for MappedEcamRegion","synthetic":true,"types":[]},{"text":"impl !Freeze for PinnedMemoryToken","synthetic":true,"types":[]},{"text":"impl Freeze for ResourceFlags","synthetic":true,"types":[]},{"text":"impl !Freeze for Resource","synthetic":true,"types":[]},{"text":"impl Freeze for ResourceInfo","synthetic":true,"types":[]},{"text":"impl Freeze for PcieIrqMode","synthetic":true,"types":[]},{"text":"impl Freeze for PciAddrSpace","synthetic":true,"types":[]},{"text":"impl Freeze for ResourceKind","synthetic":true,"types":[]},{"text":"impl Freeze for PciIrqSwizzleLut","synthetic":true,"types":[]},{"text":"impl Freeze for PciInitArgsIrqs","synthetic":true,"types":[]},{"text":"impl Freeze for PciInitArgsHeader","synthetic":true,"types":[]},{"text":"impl Freeze for PciInitArgsAddrWindows","synthetic":true,"types":[]},{"text":"impl !Freeze for Guest","synthetic":true,"types":[]},{"text":"impl !Freeze for Vcpu","synthetic":true,"types":[]},{"text":"impl Freeze for VmmPageTable","synthetic":true,"types":[]},{"text":"impl !Freeze for Channel","synthetic":true,"types":[]},{"text":"impl Freeze for MessagePacket","synthetic":true,"types":[]},{"text":"impl !Freeze for Fifo","synthetic":true,"types":[]},{"text":"impl !Freeze for Socket","synthetic":true,"types":[]},{"text":"impl Freeze for SocketFlags","synthetic":true,"types":[]},{"text":"impl Freeze for SocketInfo","synthetic":true,"types":[]},{"text":"impl Freeze for Handle","synthetic":true,"types":[]},{"text":"impl Freeze for HandleBasicInfo","synthetic":true,"types":[]},{"text":"impl Freeze for HandleInfo","synthetic":true,"types":[]},{"text":"impl Freeze for Rights","synthetic":true,"types":[]},{"text":"impl Freeze for Signal","synthetic":true,"types":[]},{"text":"impl !Freeze for KObjectBase","synthetic":true,"types":[]},{"text":"impl !Freeze for DummyObject","synthetic":true,"types":[]},{"text":"impl !Freeze for Event","synthetic":true,"types":[]},{"text":"impl !Freeze for EventPair","synthetic":true,"types":[]},{"text":"impl !Freeze for Futex","synthetic":true,"types":[]},{"text":"impl Freeze for PortPacket","synthetic":true,"types":[]},{"text":"impl Freeze for PacketSignal","synthetic":true,"types":[]},{"text":"impl Freeze for PacketGuestBell","synthetic":true,"types":[]},{"text":"impl Freeze for PacketGuestMem","synthetic":true,"types":[]},{"text":"impl Freeze for PacketGuestIo","synthetic":true,"types":[]},{"text":"impl Freeze for PacketGuestVcpuInterrupt","synthetic":true,"types":[]},{"text":"impl Freeze for PacketGuestVcpuStartup","synthetic":true,"types":[]},{"text":"impl Freeze for PacketGuestVcpu","synthetic":true,"types":[]},{"text":"impl Freeze for PacketInterrupt","synthetic":true,"types":[]},{"text":"impl Freeze for PortPacketRepr","synthetic":true,"types":[]},{"text":"impl !Freeze for Port","synthetic":true,"types":[]},{"text":"impl Freeze for PortOptions","synthetic":true,"types":[]},{"text":"impl !Freeze for Timer","synthetic":true,"types":[]},{"text":"impl Freeze for Payload","synthetic":true,"types":[]},{"text":"impl Freeze for PacketGuestVcpuData","synthetic":true,"types":[]},{"text":"impl Freeze for PacketType","synthetic":true,"types":[]},{"text":"impl Freeze for PacketGuestVcpuType","synthetic":true,"types":[]},{"text":"impl Freeze for PayloadRepr","synthetic":true,"types":[]},{"text":"impl Freeze for Slack","synthetic":true,"types":[]},{"text":"impl !Freeze for Exceptionate","synthetic":true,"types":[]},{"text":"impl Freeze for ExceptionReport","synthetic":true,"types":[]},{"text":"impl !Freeze for ExceptionObject","synthetic":true,"types":[]},{"text":"impl !Freeze for Job","synthetic":true,"types":[]},{"text":"impl Freeze for JobInfo","synthetic":true,"types":[]},{"text":"impl Freeze for JobPolicy","synthetic":true,"types":[]},{"text":"impl Freeze for BasicPolicy","synthetic":true,"types":[]},{"text":"impl Freeze for TimerSlackPolicy","synthetic":true,"types":[]},{"text":"impl !Freeze for Process","synthetic":true,"types":[]},{"text":"impl Freeze for ProcessInfo","synthetic":true,"types":[]},{"text":"impl !Freeze for SuspendToken","synthetic":true,"types":[]},{"text":"impl !Freeze for Thread","synthetic":true,"types":[]},{"text":"impl Freeze for ThreadFlag","synthetic":true,"types":[]},{"text":"impl Freeze for CurrentThread","synthetic":true,"types":[]},{"text":"impl Freeze for ThreadInfo","synthetic":true,"types":[]},{"text":"impl Freeze for ExceptionType","synthetic":true,"types":[]},{"text":"impl Freeze for SetPolicyOptions","synthetic":true,"types":[]},{"text":"impl Freeze for PolicyCondition","synthetic":true,"types":[]},{"text":"impl Freeze for PolicyAction","synthetic":true,"types":[]},{"text":"impl Freeze for Status","synthetic":true,"types":[]},{"text":"impl Freeze for ThreadStateKind","synthetic":true,"types":[]},{"text":"impl Freeze for ThreadState","synthetic":true,"types":[]},{"text":"impl !Freeze for KCounter","synthetic":true,"types":[]},{"text":"impl Freeze for KCounterDescriptor","synthetic":true,"types":[]},{"text":"impl Freeze for KCounterDescriptorArray","synthetic":true,"types":[]},{"text":"impl !Freeze for Stream","synthetic":true,"types":[]},{"text":"impl Freeze for StreamInfo","synthetic":true,"types":[]},{"text":"impl Freeze for VmarFlags","synthetic":true,"types":[]},{"text":"impl !Freeze for VmAddressRegion","synthetic":true,"types":[]},{"text":"impl Freeze for VmarInfo","synthetic":true,"types":[]},{"text":"impl !Freeze for VmMapping","synthetic":true,"types":[]},{"text":"impl Freeze for TaskStatsInfo","synthetic":true,"types":[]},{"text":"impl !Freeze for VmObject","synthetic":true,"types":[]},{"text":"impl Freeze for VmoInfo","synthetic":true,"types":[]},{"text":"impl Freeze for VmoInfoFlags","synthetic":true,"types":[]},{"text":"impl Freeze for KERNEL_ASPACE","synthetic":true,"types":[]},{"text":"impl Freeze for SeekOrigin","synthetic":true,"types":[]}];
implementors["zircon_syscall"] = [{"text":"impl&lt;'a&gt; Freeze for Syscall&lt;'a&gt;","synthetic":true,"types":[]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()