# unsafe_unions

**Note**: This macro currently uses the plugin [interpolate_idents](https://github.com/SkylerLipthay/interpolate_idents) as a workaround for a [rust bug](https://github.com/rust-lang/rust/issues/12249), and as such, rust nightly is a requirement.

**API**

```rust
unsafe_unions!{
    union $Union: $Storage {
        $field: $field_ty,
        ...
    }
    ...
}
```

`$Storage` shall be a POD-type bigger or equal in size of the biggest field. This needs to be specified, as we have currently no way of figuring out which field is the biggest at compile-time. 

**Generated Methods**:

```rust
pub unsafe fn new_{field}(val: {field_ty}) -> Self;
pub unsafe fn write_{field}(&mut self, val: {field_ty});
pub unsafe fn read_{field}(&self) -> {field_ty};
pub unsafe fn as_{field}(&self) -> &{field_ty};
pub unsafe fn as_{field}_mut(&mut self) -> &mut {field_ty};
```

### Example

```rust
#![feature(plugin)]
#![plugin(interpolate_idents)]

#[macro_use]
extern crate unsafe_unions;

unsafe_unions!{
    union UntaggedValue: [u64; 3] {
        nil: (),
        boolean: bool,
        integer: i64,
        floating: f64,
        string: String,
    }
}

fn main(){
    unsafe {
        let mut val = UntaggedValue::new_integer(200);
        assert_eq!(*val.as_integer(), 200);

        *val.as_boolean_mut() = false;
        assert_eq!(*val.as_boolean(), false);
    
        val.write_string("foobar".to_owned());
        assert_eq!(&**val.as_string(), "foobar");

        drop(val.read_string());
    }
}
```

### TODO

* maybe 'untagged_unions' is a more fitting name?
* write docs
* write tests
