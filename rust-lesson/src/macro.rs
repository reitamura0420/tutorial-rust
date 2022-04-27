#[macro_export]
macro_rules! {
  ( $( $x: expr), *) => {
      {
        let mut temp_vec = Vec::new();
        $(
          temp_vec.push($x);
        )*
        temp_vec
      }
  };
}

use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {}
