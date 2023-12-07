// Here's the basic way this works: Using TryFrom
// Based on: https://doc.rust-lang.org/std/convert/trait.TryFrom.html#generic-implementations
#[derive(Debug)]
struct GreaterThanZero(i32);

impl TryFrom<i32> for GreaterThanZero {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value <= 0 {
            Err("GreaterThanZero only accepts values greater than zero!")
        } else {
            Ok(GreaterThanZero(value))
        }
    }
}

// And here's the fancier way, using the proc_macro2 crate
// Based on https://github.com/dtolnay/syn/blob/master/examples/heapsize/heapsize_derive/src/lib.rs
extern crate proc_macro;
use quote::quote;

#[proc_macro_derive(MyDerive)]
pub fn my_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let expanded = quote! {
        // The generated impl.
        impl TryFrom<i32> for GreaterThanZero {
            type Error = &'static str;

            fn try_from(value: i32) -> Result<Self, Self::Error> {
                if value <= 0 {
                    Err("GreaterThanZero only accepts values greater than zero!")
                } else {
                    Ok(GreaterThanZero(value))
                }
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

// Apparently "error: can't use a procedural macro from the same crate that defines it"
#[derive(Debug, MyDerive)]
struct GreaterThanZero2(i32);

fn main() {
    let foo: GreaterThanZero = GreaterThanZero::try_from(101).unwrap();
    println!("{:?}", foo);

    let bar: GreaterThanZero = GreaterThanZero::try_from(-83).unwrap();
    println!("{:?}", bar);
}
