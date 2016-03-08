# unsafe_unions

### Updates

**(0.0.2)**: The lib now runs on stable & no_std.

Downside is, it's now a bit weirder to execute the different ops: instead of differently named functions, you now get different "modes", each mode for one op.

```
.get_foo() => .by_ref().foo()
.get_foo_mut() => .by_mut().foo()
.read_foo() => .read().foo()
.write_foo(x) => .write().foo(x)
```

### API

```rust
unsafe_unions!{
    [pub] union $union: $repr {
        $variant: $variant_ty,
        ...
    }
    ...
}
```

`$repr` shall be a POD-type bigger or equal in size of the biggest field.
This needs to be specified, as we have currently no way of figuring out which variant is the
biggest at compile-time. 

**Generated Methods**:

```rust
/** $union/$union<()> (default mode) **/

/// Creates a new $union with uninitialized memory.
pub fn new() -> Self;

/// Creates a new $union with zeroed memory.
pub fn zeroed() -> Self;

/// Creates a new $union with uninitialized memory and writes `v` to it.
pub unsafe fn $variant(v: $variant_ty) -> Self;

pub fn repr(&self) -> &$repr;
pub fn repr_mut(&mut self) -> &mut $repr;

/// Enters by-ref mode.
pub fn by_ref(&self) -> &$union<ByRef>;

/// Enters by-mut mode.
pub fn by_mut(&mut self) -> &mut $union<ByMut>;

/// Enters read mode.
pub fn read(&self) -> &$union<Read>;

/// Enters write mode.
pub fn write(&mut self) -> &mut $union<Read>;


/** $union<ByRef> (by-ref mode) **/

/// Casts `&self` to `&$variant_ty`.
pub unsafe fn $variant(&self) -> &$variant_ty;


/** $union<ByMut> (by-mut mode) **/

/// Casts `&mut self` to `&mut $variant_ty`.
pub unsafe fn $variant(&mut self) -> &mut $variant_ty;


/** $union<Read> (read mode) **/

/// `ptr::read`-operation
pub unsafe fn $variant(&self) -> $variant_ty;


/** $union<Write> (write mode) **/

/// `ptr::write`-operation
pub unsafe fn $variant(&mut self, v: $variant_ty);
```

All `$union`-types also implement `Clone` and `Copy`.

### Example

```rust
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
        let mut val: UntaggedValue = UntaggedValue::integer(200);
        assert_eq!(*val.by_ref().integer(), 200);

        *val.by_mut().boolean() = false;
        assert_eq!(*val.by_ref().boolean(), false);
    
        val.write().string("foobar".to_owned());
        assert_eq!(&**val.by_ref().string(), "foobar");

        drop(val.read().string());
    }
}
```

### TODO

* write docs
* write tests
