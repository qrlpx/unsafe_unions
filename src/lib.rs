#![no_std]

#[doc(hidden)]
pub mod _core {
    pub use core::*;
}

pub struct ByRef;
pub struct ByMut;
pub struct Read;
pub struct Write;

#[macro_export]
macro_rules! unsafe_unions {
    // endpoint
    () => {};
    
    // pub union
    (
        $(#[$attr:meta])* 
        pub union $union:ident: $repr:ty {
            $($variant:ident: $variant_ty:ty),+
        }
        $($ff:tt)*
    ) => {
        pub struct $union<_Mode = ()> {
            _repr: $repr,
            _phantom: $crate::_core::marker::PhantomData<fn(_Mode)>,
        }

        unsafe_unions!(@impl_pub $union; $repr; $($variant: $variant_ty),+);
        unsafe_unions!($($ff)*);
    };
    
    // pub union (trailing comma)
    (
        $(#[$attr:meta])* 
        pub union $union:ident: $repr:ty {
            $($variant:ident: $variant_ty:ty),+,
        }
        $($ff:tt)*
    ) => {
        pub struct $union<_Mode = ()> {
            _repr: $repr,
            _phantom: $crate::_core::marker::PhantomData<fn(_Mode)>,
        }

        unsafe_unions!(@impl_pub $union; $repr; $($variant: $variant_ty),+);
        unsafe_unions!($($ff)*);
    };
        
    // non-pub union
    (
        $(#[$attr:meta])* 
        union $union:ident: $repr:ty {
            $($variant:ident: $variant_ty:ty),+
        }
        $($ff:tt)*
    ) => {
        struct $union<_Mode = ()> {
            _repr: $repr,
            _phantom: $crate::_core::marker::PhantomData<fn(_Mode)>,
        }
        
        unsafe_unions!(@impl_non_pub $union; $repr; $($variant: $variant_ty),+);
        unsafe_unions!($($ff)*);
    };
    
    // non-pub union (trailing comma)
    (
        $(#[$attr:meta])* 
        union $union:ident: $repr:ty {
            $($variant:ident: $variant_ty:ty),+,
        }
        $($ff:tt)*
    ) => {
        struct $union<_Mode = ()> {
            _repr: $repr,
            _phantom: $crate::_core::marker::PhantomData<fn(_Mode)>,
        }
        
        unsafe_unions!(@impl_non_pub $union; $repr; $($variant: $variant_ty),+);
        unsafe_unions!($($ff)*);
    };
    
    (@impl_common $union: ident; $repr:ty; $($variant:ident: $variant_ty:ty),+) => {
        impl Clone for $union {
            fn clone(&self) -> Self {
                use $crate::_core;
                Self::_check_repr_type();
                $union{
                    _repr: unsafe { _core::ptr::read(&self._repr) },
                    _phantom: _core::marker::PhantomData  
                }
            }
        }
        
        impl Copy for $union {}
        
        impl<_T> $union<_T> {
            #[doc(hidden)]
            fn _check_repr_type() {
                // Check repr-type size 
                // Currently, we can only ensure this at run-time (Panics are preferable 
                // to segfault, anyways).
                
                // TODO: errmsg is a bit too wordy.
                use $crate::_core::mem;
                $(
                    assert!(
                        mem::size_of::<$variant_ty>() <= mem::size_of::<$repr>(),
                        "union '{}': repr-type needs to be bigger or equal in \
                        size to the biggest field. \
                        '{}' is '{}' bytes, field '{}' of type '{}' is '{}' bytes",
                        stringify!($union),
                        stringify!($repr), 
                        mem::size_of::<$repr>(),
                        stringify!($variant), 
                        stringify!($variant_ty), 
                        mem::size_of::<$variant_ty>(),
                    );
                )+
            }
        }
    };
    
    (@impl_pub $union: ident; $repr:ty; $($variant:ident: $variant_ty:ty),+) => {  
        unsafe_unions!(@impl_common $union; $repr; $($variant: $variant_ty),+);
              
        impl $union {
            pub fn new() -> Self {
                use $crate::_core;
                Self::_check_repr_type();
                $union{ 
                    _repr: unsafe { _core::mem::uninitialized() }, 
                    _phantom: _core::marker::PhantomData 
                }   
            }
            pub fn zeroed() -> Self {
                use $crate::_core;
                Self::_check_repr_type();
                $union{ 
                    _repr: unsafe { _core::mem::zeroed() }, 
                    _phantom: _core::marker::PhantomData  
                }
            }
            
            $(pub unsafe fn $variant(v: $variant_ty) -> Self {
                let mut ret = Self::new();
                ret.write().$variant(v);
                ret
            })+
    
            pub fn repr(&self) -> &$repr { &self._repr }
            pub fn repr_mut(&mut self) -> &mut $repr { &mut self._repr }
 
            pub fn by_ref(&self) -> &$union<$crate::ByRef> {
                use $crate::_core;
                unsafe { _core::mem::transmute(self) }
            }
            pub fn by_mut(&mut self) -> &mut $union<$crate::ByMut> {
                use $crate::_core;
                unsafe { _core::mem::transmute(self) }
            }
            pub fn read(&self) -> &$union<$crate::Read> {
                use $crate::_core;
                unsafe { _core::mem::transmute(self) }
            }
            pub fn write(&mut self) -> &mut $union<$crate::Write> {
                use $crate::_core;
                unsafe { _core::mem::transmute(self) }
            }  
        }
        
        impl $union<$crate::ByRef> {
            $(pub unsafe fn $variant(&self) -> &$variant_ty {
                use $crate::_core;
                _core::mem::transmute(self)
            })+
        }

        impl $union<$crate::ByMut> {
            $(pub unsafe fn $variant(&mut self) -> &mut $variant_ty {
                use $crate::_core;
                _core::mem::transmute(self)
            })+
        }

        impl $union<$crate::Read> {
            $(pub unsafe fn $variant(&self) -> $variant_ty {
                use $crate::_core;
                _core::ptr::read(_core::mem::transmute(self))
            })+
        }

        impl $union<$crate::Write> {
            $(pub unsafe fn $variant(&mut self, v: $variant_ty){
                use $crate::_core;
                _core::ptr::write(_core::mem::transmute(self), v)
            })+
        }
    };
    
    (@impl_non_pub $union: ident; $repr:ty; $($variant:ident: $variant_ty:ty),+) => {
        unsafe_unions!(@impl_common $union; $repr; $($variant: $variant_ty),+);
        
        #[allow(unused, missing_docs)]
        impl $union {
            fn new() -> Self {
                use $crate::_core;
                $union{ 
                _repr: unsafe { _core::mem::uninitialized() }, 
                _phantom: _core::marker::PhantomData 
                }   
            }
            fn zeroed() -> Self {
                use $crate::_core;
                $union{ 
                    _repr: unsafe { _core::mem::zeroed() }, 
                    _phantom: _core::marker::PhantomData  
                }
            }
            
            $(unsafe fn $variant(v: $variant_ty) -> Self {
                let mut ret = Self::new();
                ret.write().$variant(v);
                ret
            })+
    
            fn repr(&self) -> &$repr { &self._repr }
            fn repr_mut(&mut self) -> &mut $repr { &mut self._repr }
 
            fn by_ref(&self) -> &$union<$crate::ByRef> {
                use $crate::_core;
                unsafe { _core::mem::transmute(self) }
            }
            fn by_mut(&mut self) -> &mut $union<$crate::ByMut> {
                use $crate::_core;
                unsafe { _core::mem::transmute(self) }
            }
            fn read(&self) -> &$union<$crate::Read> {
                use $crate::_core;
                unsafe { _core::mem::transmute(self) }
            }
            fn write(&mut self) -> &mut $union<$crate::Write> {
                use $crate::_core;
                unsafe { _core::mem::transmute(self) }
            }  
        }
        
        #[allow(unused, missing_docs)]
        impl $union<$crate::ByRef> {
            $(unsafe fn $variant(&self) -> &$variant_ty {
                use $crate::_core;
                _core::mem::transmute(self)
            })+
        }

        #[allow(unused, missing_docs)]
        impl $union<$crate::ByMut> {
            $(unsafe fn $variant(&mut self) -> &mut $variant_ty {
                use $crate::_core;
                _core::mem::transmute(self)
            })+
        }

        #[allow(unused, missing_docs)]
        impl $union<$crate::Read> {
            $(unsafe fn $variant(&self) -> $variant_ty {
                use $crate::_core;
                _core::ptr::read(_core::mem::transmute(self))
            })+
        }

        #[allow(unused, missing_docs)]
        impl $union<$crate::Write> {
            $(unsafe fn $variant(&mut self, v: $variant_ty){
                use $crate::_core;
                _core::ptr::write(_core::mem::transmute(self), v)
            })+
        }
    };
}

