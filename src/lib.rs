#[repr(C)]
pub struct Mbuf<'lt, M, D> {
    metadata: M,
    length: usize,
    _marker: std::marker::PhantomData<&'lt D>,
}

impl<'lt, M, D> Mbuf<'lt, M, D> {
    pub fn to_slice(&self) -> &[D] {
        &*self
    }

    pub fn to_slice_mut(&mut self) -> &mut [D] {
        &mut *self
    }

    pub fn get_metadata(&self) -> &M {
        &self.metadata
    }

    pub fn set_metadata(&mut self, metadata: M) -> M {
        std::mem::replace(&mut self.metadata, metadata)
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

impl<'lt, M, D> Mbuf<'lt, M, D> {
    pub unsafe fn at_ptr(pointer: *const u8) -> &'lt Self {
        &*(pointer as *const Mbuf<'lt, M, D>)
    }

    pub unsafe fn at_ptr_mut(pointer: *mut u8) -> &'lt mut Self {
        &mut *(pointer as *mut Mbuf<'lt, M, D>)
    }

    pub unsafe fn at_offset(pointer: *const u8, offset: usize) -> &'lt Self {
        &*((pointer.add(offset)) as *const Mbuf<'lt, M, D>)
    }

    pub unsafe fn at_offset_mut(pointer: *mut u8, offset: usize) -> &'lt mut Self {
        &mut *((pointer.add(offset)) as *mut Mbuf<'lt, M, D>)
    }
}

impl<'lt, M, D: Copy> Mbuf<'lt, M, D> {
    pub unsafe fn write_to_ptr(pointer: *mut u8, metadata: M, data: &[D]) -> &'lt Self {
        Mbuf::write_to_ptr_mut(pointer, metadata, data)
    }

    pub unsafe fn write_to_ptr_mut(pointer: *mut u8, metadata: M, data: &[D]) -> &'lt mut Self {
        let mbuf = Mbuf::at_ptr_mut(pointer);

        mbuf.metadata = metadata;
        mbuf.length = data.len();
        mbuf.copy_from_slice(data);

        return mbuf;
    }

    pub unsafe fn write_to_offset(
        pointer: *mut u8,
        offset: usize,
        metadata: M,
        data: &[D],
    ) -> &'lt Self {
        Mbuf::write_to_offset_mut(pointer, offset, metadata, data)
    }

    pub unsafe fn write_to_offset_mut(
        pointer: *mut u8,
        offset: usize,
        metadata: M,
        data: &[D],
    ) -> &'lt mut Self {
        Mbuf::write_to_ptr_mut(pointer.add(offset), metadata, data)
    }
}

impl<'lt, M, D> std::ops::Deref for Mbuf<'lt, M, D> {
    type Target = [D];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let address = &self.length as *const usize as usize + std::mem::size_of::<usize>();

            std::slice::from_raw_parts(align::<D>(address), self.length)
        }
    }
}

impl<'lt, M, D> std::ops::DerefMut for Mbuf<'lt, M, D> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let address = &self.length as *const usize as usize + std::mem::size_of::<usize>();

            std::slice::from_raw_parts_mut(align::<D>(address) as *mut D, self.length)
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
