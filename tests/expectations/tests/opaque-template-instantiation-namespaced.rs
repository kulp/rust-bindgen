/* automatically generated by rust-bindgen */


#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]


#[allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub mod root {
    #[allow(unused_imports)]
    use self::super::root;
    pub mod zoidberg {
        #[allow(unused_imports)]
        use self::super::super::root;
        #[repr(C)]
        #[derive(Debug, Copy, Clone, Hash)]
        pub struct Template<T> {
            pub member: T,
            pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
        }
        impl <T> Default for Template<T> {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Hash)]
        pub struct Foo {
            pub c: ::std::os::raw::c_char,
        }
        #[test]
        fn bindgen_test_layout_Foo() {
            assert_eq!(::std::mem::size_of::<Foo>() , 1usize , concat ! (
                       "Size of: " , stringify ! ( Foo ) ));
            assert_eq! (::std::mem::align_of::<Foo>() , 1usize , concat ! (
                        "Alignment of " , stringify ! ( Foo ) ));
            assert_eq! (unsafe {
                        & ( * ( 0 as * const Foo ) ) . c as * const _ as usize
                        } , 0usize , concat ! (
                        "Alignment of field: " , stringify ! ( Foo ) , "::" ,
                        stringify ! ( c ) ));
        }
        impl Clone for Foo {
            fn clone(&self) -> Self { *self }
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Hash)]
        pub struct Bar {
            pub i: ::std::os::raw::c_int,
        }
        #[test]
        fn bindgen_test_layout_Bar() {
            assert_eq!(::std::mem::size_of::<Bar>() , 4usize , concat ! (
                       "Size of: " , stringify ! ( Bar ) ));
            assert_eq! (::std::mem::align_of::<Bar>() , 4usize , concat ! (
                        "Alignment of " , stringify ! ( Bar ) ));
            assert_eq! (unsafe {
                        & ( * ( 0 as * const Bar ) ) . i as * const _ as usize
                        } , 0usize , concat ! (
                        "Alignment of field: " , stringify ! ( Bar ) , "::" ,
                        stringify ! ( i ) ));
        }
        impl Clone for Bar {
            fn clone(&self) -> Self { *self }
        }
        #[repr(C)]
        #[derive(Debug, Copy, Hash)]
        pub struct ContainsInstantiation {
            pub not_opaque: root::zoidberg::Template<root::zoidberg::Foo>,
        }
        #[test]
        fn bindgen_test_layout_ContainsInstantiation() {
            assert_eq!(::std::mem::size_of::<ContainsInstantiation>() , 1usize
                       , concat ! (
                       "Size of: " , stringify ! ( ContainsInstantiation ) ));
            assert_eq! (::std::mem::align_of::<ContainsInstantiation>() ,
                        1usize , concat ! (
                        "Alignment of " , stringify ! ( ContainsInstantiation
                        ) ));
            assert_eq! (unsafe {
                        & ( * ( 0 as * const ContainsInstantiation ) ) .
                        not_opaque as * const _ as usize } , 0usize , concat !
                        (
                        "Alignment of field: " , stringify ! (
                        ContainsInstantiation ) , "::" , stringify ! (
                        not_opaque ) ));
        }
        impl Clone for ContainsInstantiation {
            fn clone(&self) -> Self { *self }
        }
        impl Default for ContainsInstantiation {
            fn default() -> Self { unsafe { ::std::mem::zeroed() } }
        }
        #[repr(C)]
        #[derive(Debug, Default, Copy, Hash)]
        pub struct ContainsOpaqueInstantiation {
            pub opaque: u32,
        }
        #[test]
        fn bindgen_test_layout_ContainsOpaqueInstantiation() {
            assert_eq!(::std::mem::size_of::<ContainsOpaqueInstantiation>() ,
                       4usize , concat ! (
                       "Size of: " , stringify ! ( ContainsOpaqueInstantiation
                       ) ));
            assert_eq! (::std::mem::align_of::<ContainsOpaqueInstantiation>()
                        , 4usize , concat ! (
                        "Alignment of " , stringify ! (
                        ContainsOpaqueInstantiation ) ));
            assert_eq! (unsafe {
                        & ( * ( 0 as * const ContainsOpaqueInstantiation ) ) .
                        opaque as * const _ as usize } , 0usize , concat ! (
                        "Alignment of field: " , stringify ! (
                        ContainsOpaqueInstantiation ) , "::" , stringify ! (
                        opaque ) ));
        }
        impl Clone for ContainsOpaqueInstantiation {
            fn clone(&self) -> Self { *self }
        }
    }
    #[test]
    fn __bindgen_test_layout_Template_open0_Foo_close0_instantiation() {
        assert_eq!(::std::mem::size_of::<root::zoidberg::Template<root::zoidberg::Foo>>()
                   , 1usize , concat ! (
                   "Size of template specialization: " , stringify ! (
                   root::zoidberg::Template<root::zoidberg::Foo> ) ));
        assert_eq!(::std::mem::align_of::<root::zoidberg::Template<root::zoidberg::Foo>>()
                   , 1usize , concat ! (
                   "Alignment of template specialization: " , stringify ! (
                   root::zoidberg::Template<root::zoidberg::Foo> ) ));
    }
}
