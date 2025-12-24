#[repr(C)]
pub struct Mbuf<'lt, M, D> {
    metadata: M,
    length: usize,
    _marker: std::marker::PhantomData<&'lt D>,
}

impl<M: Copy, D: Copy> Mbuf<'_, M, D> {
    pub fn to_slice(&self) -> &[D] {
        self
    }

    pub fn to_slice_mut(&mut self) -> &mut [D] {
        &mut *self
    }

    pub const fn get_metadata(&self) -> &M {
        &self.metadata
    }

    pub const fn set_metadata(&mut self, metadata: M) -> M {
        std::mem::replace(&mut self.metadata, metadata)
    }

    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub const fn len(&self) -> usize {
        self.length
    }
}

impl<'lt, M: Copy, D: Copy> Mbuf<'lt, M, D> {
    /// Declares an Mbuf begins at a given pointer
    /// # Safety
    /// Safe only if the pointer points to a valid Mbuf<'lt, M, D>.
    /// - The memory region at `pointer` must outlive the returned `&Mbuf`.
    #[must_use]
    pub const unsafe fn at_ptr(pointer: *const u8) -> &'lt Self {
        &*pointer.cast::<Mbuf<'lt, M, D>>()
    }

    /// Declares a mutable Mbuf begins at a given pointer
    /// # Safety
    /// Safe only if the pointer points to a valid Mbuf<'lt, M, D> in writable memory.
    /// - The memory region at `pointer` must outlive the returned `&Mbuf`.
    pub unsafe fn at_ptr_mut(pointer: *mut u8) -> &'lt mut Self {
        &mut *pointer.cast::<Mbuf<'lt, M, D>>()
    }

    /// Declares an Mbuf begins at a given byte offset from a given pointer
    /// # Safety
    /// Safe only if the the region at (pointer + offset) contains a valid Mbuf<'lt, M, D>.
    /// - The memory region at `pointer` must outlive the returned `&Mbuf`.
    #[must_use]
    pub const unsafe fn at_offset(pointer: *const u8, offset: usize) -> &'lt Self {
        &*(pointer.add(offset)).cast::<Mbuf<'lt, M, D>>()
    }

    /// Declares a mutable Mbuf begins at a given byte offset from a given pointer
    /// # Safety
    /// Safe only if the the region at (pointer + offset) contains a valid Mbuf<'lt, M, D> in writable memory.
    /// - The memory region at `pointer` must outlive the returned `&Mbuf`.
    pub unsafe fn at_offset_mut(pointer: *mut u8, offset: usize) -> &'lt mut Self {
        &mut *(pointer.add(offset)).cast::<Mbuf<'lt, M, D>>()
    }

    /// declares a memory buffer **without data initialization -- items must be initialized by caller**
    /// # Safety
    /// Safe if the memory region pointed to is large enough to hold an Mbuf<'lt, M, D> and is writable.
    /// <br>**Calling this function does not initialize data values in the Mbuf.**
    /// <br>Don't do this unless you know what you're doing.
    /// - The memory region at `pointer` must outlive the returned `&Mbuf`.
    pub unsafe fn init_at_ptr(pointer: *mut u8, metadata: M, length: usize) -> &'lt mut Self {
        let mbuf = Mbuf::at_ptr_mut(pointer);

        mbuf.metadata = metadata;
        mbuf.length = length;

        mbuf
    }

    /// Declares a memory buffer **without data initialization**.
    ///
    /// # Safety
    ///
    /// - The memory region at `pointer.add(offset)` must be large enough to hold an `Mbuf<'lt, M, D>` and is writable.
    /// - The entire region `[pointer, pointer + offset + sizeof(Mbuf))` must be valid for the lifetime `'lt`.
    ///
    /// **Calling this function does not initialize data values in the `Mbuf`.**
    pub unsafe fn init_at_offset(
        pointer: *mut u8,
        offset: usize,
        metadata: M,
        length: usize,
    ) -> &'lt mut Self {
        Self::init_at_ptr(pointer.add(offset), metadata, length)
    }
}

impl<'lt, M: Copy, D: Copy> Mbuf<'lt, M, D> {
    /// Declares an Mbuf at `pointer` and copies `metadata` and `data` into it.
    /// # Safety
    /// - `pointer` must point to a large enough place in memory to hold `metadata` + `usize` + `data`.
    /// - `pointer` must be aligned to at least `usize` and at least `M`.
    /// - The memory region at `pointer` must outlive the returned `&Mbuf`.
    /// - The memory region at `pointer` must be writable
    pub unsafe fn write_to_ptr(pointer: *mut u8, metadata: M, data: &[D]) -> &'lt Self {
        Mbuf::write_to_ptr_mut(pointer, metadata, data)
    }

    /// Declares a mutable Mbuf at `pointer` and copies `metadata` and `data` into it.
    /// # Safety
    /// - `pointer` must point to a large enough place in memory to hold `metadata` + `usize` + `data`.
    /// - `pointer` must be aligned to at least `usize` and at least `M`.
    /// - The memory region at `pointer` must outlive the returned `&Mbuf`.
    /// - The memory region at `pointer` must be writable
    pub unsafe fn write_to_ptr_mut(pointer: *mut u8, metadata: M, data: &[D]) -> &'lt mut Self {
        let mbuf = Mbuf::init_at_ptr(pointer, metadata, data.len());

        mbuf.copy_from_slice(data);

        mbuf
    }

    /// Declares an Mbuf at `pointer + offset` and copies `metadata` and `data` into it.
    /// # Safety
    /// - `pointer` must point to a large enough place in memory to hold `metadata` + `usize` + `data`.
    /// - `pointer` must be aligned to at least `usize` and at least `M`.
    /// - The memory region at `pointer` must outlive the returned `&Mbuf`.
    /// - The memory region at `pointer` must be writable
    pub unsafe fn write_to_offset(
        pointer: *mut u8,
        offset: usize,
        metadata: M,
        data: &[D],
    ) -> &'lt Self {
        Mbuf::write_to_offset_mut(pointer, offset, metadata, data)
    }

    /// Declares a mutable Mbuf at `pointer + offset` and copies `metadata` and `data` into it.
    /// # Safety
    /// - `pointer` must point to a large enough place in memory to hold `metadata` + `usize` + `data`.
    /// - `pointer` must be aligned to at least `usize` and at least `M`.
    /// - The memory region at `pointer` must outlive the returned `&Mbuf`.
    /// - The memory region at `pointer` must be writable
    pub unsafe fn write_to_offset_mut(
        pointer: *mut u8,
        offset: usize,
        metadata: M,
        data: &[D],
    ) -> &'lt mut Self {
        Mbuf::write_to_ptr_mut(pointer.add(offset), metadata, data)
    }
}

impl<M: Copy, D: Copy> AsRef<[D]> for Mbuf<'_, M, D> {
    fn as_ref(&self) -> &[D] {
        self
    }
}

impl<M: Copy, D: Copy> AsMut<[D]> for Mbuf<'_, M, D> {
    fn as_mut(&mut self) -> &mut [D] {
        self
    }
}

impl<M: Copy, D: Copy> std::ops::Deref for Mbuf<'_, M, D> {
    type Target = [D];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let address =
                std::ptr::from_ref::<usize>(&self.length) as usize + std::mem::size_of::<usize>();

            std::slice::from_raw_parts(align::<D>(address), self.length)
        }
    }
}

impl<M: Copy, D: Copy> std::ops::DerefMut for Mbuf<'_, M, D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let address =
                std::ptr::from_ref::<usize>(&self.length) as usize + std::mem::size_of::<usize>();

            std::slice::from_raw_parts_mut(align::<D>(address).cast_mut(), self.length)
        }
    }
}

const fn align<T>(address: usize) -> *const T {
    let align_size = std::mem::align_of::<T>();
    let remainder = address % align_size;

    if remainder == 0 {
        address as *const T
    } else {
        (address + align_size - remainder) as *const T
    }
}
