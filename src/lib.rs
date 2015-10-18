#![feature(
    plugin,
    optin_builtin_traits,
)]
#![plugin(interpolate_idents)]

#[macro_export]
macro_rules! unsafe_unions {
    ($(
        $(#[$attr:meta])*  
        union $Union:ident: $Repr:ty {
            $($fields:tt)+
        }
    )+) => {$(

        $(#[$attr])*
        pub struct $Union {
            _data: $Repr
        }

        #[allow(unused, missing_docs)]
        impl $Union {
            unsafe_unions!(GEN_ACCESSORS; $Union; $($fields)+);
            unsafe_unions!(GEN_STORAGE_CHECK; $Union; $Repr; $($fields)+);
        }

    )+};

    (GEN_ACCESSORS; $Union:ident; $($field_name:ident: $field_ty:ty),*) => {$(
    
        interpolate_idents!{ // FIXME https://github.com/rust-lang/rust/issues/12249

        // new_*
        pub unsafe fn [new_ $field_name](val: $field_ty) -> Self {
            use std::mem;

            Self::__check_storage_type();

            let mut union = $Union{ _data: mem::uninitialized() };
            union.[write_ $field_name](val);
            union
        }

        pub unsafe fn [write_ $field_name](&mut self, val: $field_ty){
            use std::ptr;

            ptr::write(self as *mut Self as *mut $field_ty, val);
        }

        pub unsafe fn [read_ $field_name](&self) -> $field_ty {
            use std::ptr;

            ptr::read(self as *const Self as *const $field_ty)
        }

        pub unsafe fn [as_ $field_name](&self) -> &$field_ty {
            &*(self as *const Self as *const $field_ty)
        }

        pub unsafe fn [as_ $field_name _mut](&mut self) -> &mut $field_ty {
            &mut*(self as *mut Self as *mut $field_ty)
        }


        } // FIXME https://github.com/rust-lang/rust/issues/12249 

    )*};
    (GEN_ACCESSORS; $Union:ident; $($field_name:ident: $field_ty:ty,)*) => {
        unsafe_unions!(GEN_ACCESSORS; $Union; $($field_name: $field_ty),*);
    };

    (GEN_STORAGE_CHECK; $Union:ident; $Repr:ty; $($field_name:ident: $field_ty:ty),*) => {
        
        fn __check_storage_type(){
            
            // FIXME currently doesn't work [rustc 1.5.0-nightly (9d3e79ad3 2015-10-10)]
            /*{
                // Repr-type may not implement Drop.
                // We can ensure this at compile-time.

                trait NoDrop {}
                impl NoDrop for .. {}
                impl<T: Drop> !NoDrop for T {}

                fn ct_check_nodrop<T: NoDrop>(){}

                ct_check_nodrop::<$Repr>();
            }*/


            {
                // Check Repr-type size 
                // Currently, we can only ensure this at run-time (Panics are preferable 
                // to segfault, anyways).
                
                // TODO: errmsg is a bit too wordy.

                use std::mem;

                $(
                    assert!(
                        mem::size_of::<$field_ty>() <= mem::size_of::<$Repr>(),
                        "union '{}': Repr-type needs to be bigger or equal in \
                        size to the biggest field. \
                        '{}' is '{}' bytes, field '{}' of type '{}' is '{}' bytes",
                        stringify!($Union),
                        stringify!($Repr), 
                        mem::size_of::<$Repr>(),
                        stringify!($field_name), 
                        stringify!($field_ty), 
                        mem::size_of::<$field_ty>(),
                    );

                )+

            }

        }

    };
    (GEN_STORAGE_CHECK; $Union:ident; $Repr:ty; $($field_name:ident: $field_ty:ty,)*) => {
        unsafe_unions!(GEN_STORAGE_CHECK; $Union; $Repr; $($field_name: $field_ty),*);
    };
}

