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
