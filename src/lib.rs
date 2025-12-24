/// A memory buffer with associated metadata.
///
/// `Mbuf` is a fixed memory layout consisting of:
/// 1. Metadata of type `M`
/// 2. A `usize` length field
/// 3. Data elements of type `D` stored directly after in memory
///
/// The buffer dereferences to `&[D]` via pointer arithmetic aligned to the data region.
///
/// **Important:** The data region must be allocated contiguously in memory immediately after this struct.
/// Callers are responsible for upholding this invariant.
#[repr(C)]
pub struct Mbuf<'lt, M, D> {
    metadata: M,
    length: usize,
    _marker: std::marker::PhantomData<&'lt D>,
}

impl<M: Copy, D: Copy> Mbuf<'_, M, D> {
    /// Returns an immutable slice view of the buffer data.
    pub fn to_slice(&self) -> &[D] {
        self
    }

    /// Returns a mutable slice view of the buffer data.
    pub fn to_slice_mut(&mut self) -> &mut [D] {
        &mut *self
    }

    /// Returns a reference to the metadata.
    pub const fn get_metadata(&self) -> &M {
        &self.metadata
    }

    /// Sets the metadata and returns the previous value.
    pub const fn set_metadata(&mut self, metadata: M) -> M {
        std::mem::replace(&mut self.metadata, metadata)
    }

    /// Returns `true` if the buffer is empty.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of elements in the buffer.
    pub const fn len(&self) -> usize {
        self.length
    }
}

impl<'lt, M: Copy, D: Copy> Mbuf<'lt, M, D> {
    /// Interprets a pointer as an `Mbuf` with immutable access.
    ///
    /// # Safety
    ///
    /// - `pointer` must point to a valid, initialized `Mbuf<'lt, M, D>`.
    /// - The `Mbuf` and its data must be valid for the lifetime `'lt`.
    #[must_use]
    pub const unsafe fn at_ptr(pointer: *const u8) -> &'lt Self {
        &*pointer.cast::<Mbuf<'lt, M, D>>()
    }

    /// Interprets a pointer as an `Mbuf` with mutable access.
    ///
    /// # Safety
    ///
    /// - `pointer` must point to a valid, initialized `Mbuf<'lt, M, D>` in writable memory.
    /// - The `Mbuf` and its data must be valid for the lifetime `'lt`.
    pub unsafe fn at_ptr_mut(pointer: *mut u8) -> &'lt mut Self {
        &mut *pointer.cast::<Mbuf<'lt, M, D>>()
    }

    /// Interprets a byte offset from a pointer as an `Mbuf` with immutable access.
    ///
    /// # Safety
    ///
    /// - `pointer.add(offset)` must point to a valid, initialized `Mbuf<'lt, M, D>`.
    /// - The `Mbuf` and its data must be valid for the lifetime `'lt`.
    #[must_use]
    pub const unsafe fn at_offset(pointer: *const u8, offset: usize) -> &'lt Self {
        &*(pointer.add(offset)).cast::<Mbuf<'lt, M, D>>()
    }

    /// Interprets a byte offset from a pointer as an `Mbuf` with mutable access.
    ///
    /// # Safety
    ///
    /// - `pointer.add(offset)` must point to a valid, initialized `Mbuf<'lt, M, D>` in writable memory.
    /// - The `Mbuf` and its data must be valid for the lifetime `'lt`.
    pub unsafe fn at_offset_mut(pointer: *mut u8, offset: usize) -> &'lt mut Self {
        &mut *(pointer.add(offset)).cast::<Mbuf<'lt, M, D>>()
    }

    /// Initializes an `Mbuf` at a pointer without initializing data.
    ///
    /// Sets the metadata and length fields. **The caller must initialize the data region before access.**
    ///
    /// # Safety
    ///
    /// - `pointer` must point to writable memory large enough to hold `Mbuf<'lt, M, D>` plus `length` elements of type `D`.
    /// - `pointer` must be aligned as an `Mbuf<'lt, M, D>`.
    /// - The entire buffer region must be valid for the lifetime `'lt`.
    pub unsafe fn init_at_ptr(pointer: *mut u8, metadata: M, length: usize) -> &'lt mut Self {
        let mbuf = Mbuf::at_ptr_mut(pointer);

        mbuf.metadata = metadata;
        mbuf.length = length;

        mbuf
    }

    /// Initializes an `Mbuf` at a byte offset from a pointer without initializing data.
    ///
    /// Sets the metadata and length fields. **The caller must initialize the data region before access.**
    ///
    /// # Safety
    ///
    /// - `pointer.add(offset)` must point to writable memory large enough to hold `Mbuf<'lt, M, D>` plus `length` elements of type `D`.
    /// - `pointer.add(offset)` must be aligned as an `Mbuf<'lt, M, D>`.
    /// - The entire buffer region must be valid for the lifetime `'lt`.
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
    /// Initializes an `Mbuf` at a pointer and copies data into it.
    ///
    /// # Safety
    ///
    /// - `pointer` must point to writable memory large enough to hold the `Mbuf` header plus `data.len()` elements of type `D`.
    /// - `pointer` must be aligned as an `Mbuf<'lt, M, D>`.
    /// - The entire buffer region must be valid for the lifetime `'lt`.
    #[must_use]
    pub unsafe fn write_to_ptr(pointer: *mut u8, metadata: M, data: &[D]) -> &'lt Self {
        Mbuf::write_to_ptr_mut(pointer, metadata, data)
    }

    /// Initializes a mutable `Mbuf` at a pointer and copies data into it.
    ///
    /// # Safety
    ///
    /// - `pointer` must point to writable memory large enough to hold the `Mbuf` header plus `data.len()` elements of type `D`.
    /// - `pointer` must be aligned as an `Mbuf<'lt, M, D>`.
    /// - The entire buffer region must be valid for the lifetime `'lt`.
    pub unsafe fn write_to_ptr_mut(pointer: *mut u8, metadata: M, data: &[D]) -> &'lt mut Self {
        let mbuf = Mbuf::init_at_ptr(pointer, metadata, data.len());

        mbuf.copy_from_slice(data);

        mbuf
    }

    /// Initializes an `Mbuf` at a byte offset from a pointer and copies data into it.
    ///
    /// # Safety
    ///
    /// - `pointer.add(offset)` must point to writable memory large enough to hold the `Mbuf` header plus `data.len()` elements of type `D`.
    /// - `pointer.add(offset)` must be aligned as an `Mbuf<'lt, M, D>`.
    /// - The entire buffer region must be valid for the lifetime `'lt`.
    pub unsafe fn write_to_offset(
        pointer: *mut u8,
        offset: usize,
        metadata: M,
        data: &[D],
    ) -> &'lt Self {
        Mbuf::write_to_offset_mut(pointer, offset, metadata, data)
    }

    /// Initializes a mutable `Mbuf` at a byte offset from a pointer and copies data into it.
    ///
    /// # Safety
    ///
    /// - `pointer.add(offset)` must point to writable memory large enough to hold the `Mbuf` header plus `data.len()` elements of type `D`.
    /// - `pointer.add(offset)` must be aligned as an `Mbuf<'lt, M, D>`.
    /// - The entire buffer region must be valid for the lifetime `'lt`.
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
        let ptr: *const Self = self;
        let address = ptr as usize + std::mem::size_of::<Self>();

        unsafe { std::slice::from_raw_parts(align::<D>(address), self.length) }
    }
}

impl<M: Copy, D: Copy> std::ops::DerefMut for Mbuf<'_, M, D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let ptr: *const Self = self;
        let address = ptr as usize + std::mem::size_of::<Self>();

        unsafe { std::slice::from_raw_parts_mut(align::<D>(address).cast_mut(), self.length) }
    }
}

/// Aligns an address up to the required alignment for type `T`.
///
/// Returns the aligned address as a const pointer. If already aligned, returns unchanged.
const fn align<T>(address: usize) -> *const T {
    let align_size = std::mem::align_of::<T>();
    let remainder = address % align_size;

    if remainder == 0 {
        address as *const T
    } else {
        (address + align_size - remainder) as *const T
    }
}
