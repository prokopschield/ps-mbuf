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
