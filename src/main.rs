// Here's the basic way this works: Using TryFrom
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
// use proc_macro2::TokenStream;
use quote::quote;

#[proc_macro_derive(MyDerive)]
pub fn my_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let expanded = quote! {
    // The generated impl.
        impl #impl_generics heapsize::HeapSize for #name #ty_generics #where_clause {
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

fn main() {
    let some_number = 101;
    let another_greater_than_zero: GreaterThanZero =
        GreaterThanZero::try_from(some_number).unwrap();
    println!("{:?}", another_greater_than_zero);

    let some_neg_number = -83;
    let foo: GreaterThanZero = GreaterThanZero::try_from(some_neg_number).unwrap();
    println!("{:?}", foo);
}
