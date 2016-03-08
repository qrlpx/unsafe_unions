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
        let mut val = UntaggedValue::<()>::integer(200);
        assert_eq!(*val.by_ref().integer(), 200);

        *val.by_mut().boolean() = false;
        assert_eq!(*val.by_ref().boolean(), false);
    
        val.write().string("foobar".to_owned());
        assert_eq!(&**val.by_ref().string(), "foobar");

        drop(val.read().string());
    }
}
