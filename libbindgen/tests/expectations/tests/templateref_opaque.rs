/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct detail_PointerType<T> {
    pub _address: u8,
    pub _phantom_0: ::std::marker::PhantomData<T>,
}
pub type detail_PointerType_Type<T> = *mut T;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UniquePtr<T> {
    pub _address: u8,
    pub _phantom_0: ::std::marker::PhantomData<T>,
}
pub type UniquePtr_Pointer<T> = detail_PointerType<T>;
