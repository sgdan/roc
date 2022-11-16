// ⚠️ GENERATED CODE ⚠️ - this entire file was generated by the `roc glue` CLI command

#![allow(unused_unsafe)]
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::unused_unit)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::let_and_return)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::clone_on_copy)]

#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "wasm32",
    target_arch = "x86",
    target_arch = "x86_64"
))]
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[repr(u8)]
pub enum discriminant_Op {
    StderrWrite = 0,
    StdoutWrite = 1,
}

impl core::fmt::Debug for discriminant_Op {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::StderrWrite => f.write_str("discriminant_Op::StderrWrite"),
            Self::StdoutWrite => f.write_str("discriminant_Op::StdoutWrite"),
        }
    }
}

#[cfg(any(
    target_arch = "arm",
    target_arch = "wasm32",
    target_arch = "x86"
))]
#[repr(C)]
pub union Op {
    StderrWrite: core::mem::ManuallyDrop<roc_std::RocStr>,
    StdoutWrite: core::mem::ManuallyDrop<roc_std::RocStr>,
    _sizer: [u8; 16],
}

#[cfg(any(
    target_arch = "aarch64",
    target_arch = "x86_64"
))]
#[repr(C)]
pub union Op {
    StderrWrite: core::mem::ManuallyDrop<roc_std::RocStr>,
    StdoutWrite: core::mem::ManuallyDrop<roc_std::RocStr>,
    _sizer: [u8; 32],
}

impl Op {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "wasm32",
        target_arch = "x86"
    ))]
    /// Returns which variant this tag union holds. Note that this never includes a payload!
    pub fn discriminant(&self) -> discriminant_Op {
        unsafe {
            let bytes = core::mem::transmute::<&Self, &[u8; core::mem::size_of::<Self>()]>(self);

            core::mem::transmute::<u8, discriminant_Op>(*bytes.as_ptr().add(12))
        }
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "wasm32",
        target_arch = "x86"
    ))]
    /// Internal helper
    fn set_discriminant(&mut self, discriminant: discriminant_Op) {
        let discriminant_ptr: *mut discriminant_Op = (self as *mut Op).cast();

        unsafe {
            *(discriminant_ptr.add(12)) = discriminant;
        }
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Construct a tag named `StderrWrite`, with the appropriate payload
    pub fn StderrWrite(arg: roc_std::RocStr) -> Self {
            let mut answer = Self {
                StderrWrite: core::mem::ManuallyDrop::new(arg)
            };

            answer.set_discriminant(discriminant_Op::StderrWrite);

            answer
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Unsafely assume the given `Op` has a `.discriminant()` of `StderrWrite` and convert it to `StderrWrite`'s payload.
            /// (Always examine `.discriminant()` first to make sure this is the correct variant!)
            /// Panics in debug builds if the `.discriminant()` doesn't return `StderrWrite`.
            pub unsafe fn into_StderrWrite(mut self) -> roc_std::RocStr {
                debug_assert_eq!(self.discriminant(), discriminant_Op::StderrWrite);
        let payload = {
            let mut uninitialized = core::mem::MaybeUninit::uninit();
            let swapped = unsafe {
                core::mem::replace(
                    &mut self.StderrWrite,
                    core::mem::ManuallyDrop::new(uninitialized.assume_init()),
                )
            };

            core::mem::forget(self);

            core::mem::ManuallyDrop::into_inner(swapped)
        };

        payload
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Unsafely assume the given `Op` has a `.discriminant()` of `StderrWrite` and return its payload.
            /// (Always examine `.discriminant()` first to make sure this is the correct variant!)
            /// Panics in debug builds if the `.discriminant()` doesn't return `StderrWrite`.
            pub unsafe fn as_StderrWrite(&self) -> &roc_std::RocStr {
                debug_assert_eq!(self.discriminant(), discriminant_Op::StderrWrite);
        let payload = &self.StderrWrite;

        &payload
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Construct a tag named `StdoutWrite`, with the appropriate payload
    pub fn StdoutWrite(arg: roc_std::RocStr) -> Self {
            let mut answer = Self {
                StdoutWrite: core::mem::ManuallyDrop::new(arg)
            };

            answer.set_discriminant(discriminant_Op::StdoutWrite);

            answer
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Unsafely assume the given `Op` has a `.discriminant()` of `StdoutWrite` and convert it to `StdoutWrite`'s payload.
            /// (Always examine `.discriminant()` first to make sure this is the correct variant!)
            /// Panics in debug builds if the `.discriminant()` doesn't return `StdoutWrite`.
            pub unsafe fn into_StdoutWrite(mut self) -> roc_std::RocStr {
                debug_assert_eq!(self.discriminant(), discriminant_Op::StdoutWrite);
        let payload = {
            let mut uninitialized = core::mem::MaybeUninit::uninit();
            let swapped = unsafe {
                core::mem::replace(
                    &mut self.StdoutWrite,
                    core::mem::ManuallyDrop::new(uninitialized.assume_init()),
                )
            };

            core::mem::forget(self);

            core::mem::ManuallyDrop::into_inner(swapped)
        };

        payload
    }

    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    /// Unsafely assume the given `Op` has a `.discriminant()` of `StdoutWrite` and return its payload.
            /// (Always examine `.discriminant()` first to make sure this is the correct variant!)
            /// Panics in debug builds if the `.discriminant()` doesn't return `StdoutWrite`.
            pub unsafe fn as_StdoutWrite(&self) -> &roc_std::RocStr {
                debug_assert_eq!(self.discriminant(), discriminant_Op::StdoutWrite);
        let payload = &self.StdoutWrite;

        &payload
    }

    #[cfg(any(
        target_arch = "aarch64",
        target_arch = "x86_64"
    ))]
    /// Returns which variant this tag union holds. Note that this never includes a payload!
    pub fn discriminant(&self) -> discriminant_Op {
        unsafe {
            let bytes = core::mem::transmute::<&Self, &[u8; core::mem::size_of::<Self>()]>(self);

            core::mem::transmute::<u8, discriminant_Op>(*bytes.as_ptr().add(24))
        }
    }

    #[cfg(any(
        target_arch = "aarch64",
        target_arch = "x86_64"
    ))]
    /// Internal helper
    fn set_discriminant(&mut self, discriminant: discriminant_Op) {
        let discriminant_ptr: *mut discriminant_Op = (self as *mut Op).cast();

        unsafe {
            *(discriminant_ptr.add(24)) = discriminant;
        }
    }
}

impl Drop for Op {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn drop(&mut self) {
        // Drop the payloads
                    match self.discriminant() {
                discriminant_Op::StderrWrite => unsafe { core::mem::ManuallyDrop::drop(&mut self.StderrWrite) },
                discriminant_Op::StdoutWrite => unsafe { core::mem::ManuallyDrop::drop(&mut self.StdoutWrite) },
            }

    }
}

impl Eq for Op {}

impl PartialEq for Op {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn eq(&self, other: &Self) -> bool {
            if self.discriminant() != other.discriminant() {
                return false;
            }

            unsafe {
            match self.discriminant() {
                discriminant_Op::StderrWrite => self.StderrWrite == other.StderrWrite,
                discriminant_Op::StdoutWrite => self.StdoutWrite == other.StdoutWrite,
            }
        }
    }
}

impl PartialOrd for Op {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        match self.discriminant().partial_cmp(&other.discriminant()) {
            Some(core::cmp::Ordering::Equal) => {}
            not_eq => return not_eq,
        }

        unsafe {
            match self.discriminant() {
                discriminant_Op::StderrWrite => self.StderrWrite.partial_cmp(&other.StderrWrite),
                discriminant_Op::StdoutWrite => self.StdoutWrite.partial_cmp(&other.StdoutWrite),
            }
        }
    }
}

impl Ord for Op {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
            match self.discriminant().cmp(&other.discriminant()) {
                core::cmp::Ordering::Equal => {}
                not_eq => return not_eq,
            }

            unsafe {
            match self.discriminant() {
                discriminant_Op::StderrWrite => self.StderrWrite.cmp(&other.StderrWrite),
                discriminant_Op::StdoutWrite => self.StdoutWrite.cmp(&other.StdoutWrite),
            }
        }
    }
}

impl Clone for Op {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn clone(&self) -> Self {
        let mut answer = unsafe {
            match self.discriminant() {
                discriminant_Op::StderrWrite => Self {
                    StderrWrite: self.StderrWrite.clone(),
                },
                discriminant_Op::StdoutWrite => Self {
                    StdoutWrite: self.StdoutWrite.clone(),
                },
            }

        };

        answer.set_discriminant(self.discriminant());

        answer
    }
}

impl core::hash::Hash for Op {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {        match self.discriminant() {
            discriminant_Op::StderrWrite => unsafe {
                    discriminant_Op::StderrWrite.hash(state);
                    self.StderrWrite.hash(state);
                },
            discriminant_Op::StdoutWrite => unsafe {
                    discriminant_Op::StdoutWrite.hash(state);
                    self.StdoutWrite.hash(state);
                },
        }
    }
}

impl core::fmt::Debug for Op {
    #[cfg(any(
        target_arch = "arm",
        target_arch = "aarch64",
        target_arch = "wasm32",
        target_arch = "x86",
        target_arch = "x86_64"
    ))]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Op::")?;

        unsafe {
            match self.discriminant() {
                discriminant_Op::StderrWrite => f.debug_tuple("StderrWrite")
        .field(&*self.StderrWrite)
        .finish(),
                discriminant_Op::StdoutWrite => f.debug_tuple("StdoutWrite")
        .field(&*self.StdoutWrite)
        .finish(),
            }
        }
    }
}
