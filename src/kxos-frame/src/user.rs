//! User space.

use crate::cpu::CpuContext;
use crate::prelude::*;
use crate::task::Task;
use crate::vm::VmSpace;

/// A user space.
///
/// Each user space has a VM address space and allows a task to execute in
/// user mode.
pub struct UserSpace {}

impl UserSpace {
    /// Creates a new instance.
    ///
    /// Each instance maintains a VM address space and the CPU state to enable
    /// execution in the user space.
    pub fn new(vm_space: VmSpace, cpu_ctx: CpuContext) -> Self {
        todo!()
    }

    /// Returns the VM address space.
    pub fn vm_space(&self) -> &VmSpace {
        todo!()
    }

    /// Returns the user mode that is bound to the current task and user space.
    ///
    /// See `UserMode` on how to use it to execute user code.
    ///
    /// # Panic
    ///
    /// This method is intended to only allow each task to have at most one
    /// instance of `UserMode` initiated. If this method is called again before
    /// the first instance for the current task is dropped, then the method
    /// panics.
    pub fn user_mode(&self) -> UserMode<'_> {
        todo!()
    }
}

/// Code execution in the user mode.
///
/// This type enables executing the code in user space from a task in the kernel
/// space safely.
///
/// Here is a sample code on how to use `UserMode`.
///  
/// ```no_run
/// use kxos_frame::task::Task;
///
/// let current = Task::current();
/// let user_space = current.user_space()
///     .expect("the current task is associated with a user space");
/// let mut user_mode = user_space.user_mode();
/// loop {
///     // Execute in the user space until some interesting user event occurs
///     let user_event = user_mode.execute();
///     todo!("handle the user event, e.g., syscall");
/// }
/// ```
pub struct UserMode<'a> {
    current: Arc<Task>,
    user_space: &'a Arc<UserSpace>,
}

// An instance of `UserMode` is bound to the current task. So it cannot be
impl<'a> !Send for UserMode<'a> {}

impl<'a> UserMode<'a> {
    /// Starts executing in the user mode.
    ///
    /// The method returns for one of three possible reasons indicated by `UserEvent`.
    /// 1. The user invokes a system call;
    /// 2. The user triggers an exception;
    /// 3. The user triggers a fault.
    ///
    /// After handling the user event and updating the user-mode CPU context,
    /// this method can be invoked again to go back to the user space.
    pub fn execute(&mut self) -> UserEvent {
        todo!()
    }

    /// Returns an immutable reference the user-mode CPU context.
    pub fn context(&self) -> &CpuContext {
        todo!()
    }

    /// Returns a mutable reference the user-mode CPU context.
    pub fn context_mut(&mut self) -> &mut CpuContext {
        todo!()
    }
}

/// A user event is what brings back the control of the CPU back from
/// the user space to the kernel space.
///
/// Note that hardware interrupts are not considered user events as they
/// are triggered by devices and not visible to user programs.
/// To handle interrupts, one should register callback funtions for
/// IRQ lines (`IrqLine`).
pub enum UserEvent {
    Syscall,
    Exception,
    Fault,
}