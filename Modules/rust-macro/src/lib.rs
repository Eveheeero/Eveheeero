extern crate proc_macro;

// 사용하려면 Cargo.toml에 proc_macro 사용
#[proc_macro]
pub fn make_string(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    r#""My String".to_string()"#.parse().unwrap()
}

#[proc_macro]
pub fn make_function(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    // quote를 사용하면 변수가 # 뒤에 들어가게된다.
    let output = quote::quote! {
        fn #input() {
            println!("Hello, world!");
        }
    };

    output.into()
}

#[proc_macro_attribute]
pub fn my_attribute(
    _attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item = proc_macro2::TokenStream::from(item);

    let output = quote::quote! {
        #[derive(Debug, Default)]
        #item
    };

    output.into()
}
// #[my_attribute] 를 적용하면 #[derive(Debug, Default)] 와 마찬가지다.

#[proc_macro_attribute]
pub fn my_attribute2(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr = proc_macro2::TokenStream::from(attr);
    let item = proc_macro2::TokenStream::from(item);

    let output = quote::quote! {
        #[derive(#attr)]
        #item
    };

    output.into()
}
// #[my_attribute2(Default)] 를 적용하면 #[derive(Default)] 와 마찬가지다.

#[proc_macro_derive(MyTrait)]
pub fn my_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let output = quote::quote! {
        impl MyTrait for #input {
            fn my_trait(&self) {
                println!("MyTrait");
            }
        }
    };

    output.into()
}
// #[derive(MyTrait)] 가능
