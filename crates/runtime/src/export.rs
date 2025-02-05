use crate::vmcontext::{
    VMCallerCheckedFuncRef, VMContext, VMGlobalDefinition, VMMemoryDefinition, VMTableDefinition,
};
use std::ptr::NonNull;
use wasmtime_environ::{DefinedMemoryIndex, Global, MemoryPlan, TablePlan};

/// The value of an export passed from one instance to another.
pub enum Export {
    /// A function export value.
    Function(ExportFunction),

    /// A table export value.
    Table(ExportTable),

    /// A memory export value.
    Memory(ExportMemory),

    /// A global export value.
    Global(ExportGlobal),
}

/// A function export value.
#[derive(Debug, Clone, Copy)]
pub struct ExportFunction {
    /// The `VMCallerCheckedFuncRef` for this exported function.
    ///
    /// Note that exported functions cannot be a null funcref, so this is a
    /// non-null pointer.
    pub anyfunc: NonNull<VMCallerCheckedFuncRef>,
}

// It's part of the contract of using `ExportFunction` that synchronization
// properties are upheld, so declare that despite the raw pointers inside this
// is send/sync.
unsafe impl Send for ExportFunction {}
unsafe impl Sync for ExportFunction {}

impl From<ExportFunction> for Export {
    fn from(func: ExportFunction) -> Export {
        Export::Function(func)
    }
}

/// A table export value.
#[derive(Debug, Clone)]
pub struct ExportTable {
    /// The address of the table descriptor.
    pub definition: *mut VMTableDefinition,
    /// Pointer to the containing `VMContext`.
    pub vmctx: *mut VMContext,
    /// The table declaration, used for compatibilty checking.
    pub table: TablePlan,
}

// See docs on send/sync for `ExportFunction` above.
unsafe impl Send for ExportTable {}
unsafe impl Sync for ExportTable {}

impl From<ExportTable> for Export {
    fn from(func: ExportTable) -> Export {
        Export::Table(func)
    }
}

/// A memory export value.
#[derive(Debug, Clone)]
pub struct ExportMemory {
    /// The address of the memory descriptor.
    pub definition: *mut VMMemoryDefinition,
    /// Pointer to the containing `VMContext`.
    pub vmctx: *mut VMContext,
    /// The memory declaration, used for compatibility checking.
    pub memory: MemoryPlan,
    /// The index at which the memory is defined within the `vmctx`.
    pub index: DefinedMemoryIndex,
}

// See docs on send/sync for `ExportFunction` above.
unsafe impl Send for ExportMemory {}
unsafe impl Sync for ExportMemory {}

impl From<ExportMemory> for Export {
    fn from(func: ExportMemory) -> Export {
        Export::Memory(func)
    }
}

/// A global export value.
#[derive(Debug, Clone)]
pub struct ExportGlobal {
    /// The address of the global storage.
    pub definition: *mut VMGlobalDefinition,
    /// The global declaration, used for compatibilty checking.
    pub global: Global,
}

// See docs on send/sync for `ExportFunction` above.
unsafe impl Send for ExportGlobal {}
unsafe impl Sync for ExportGlobal {}

impl From<ExportGlobal> for Export {
    fn from(func: ExportGlobal) -> Export {
        Export::Global(func)
    }
}
