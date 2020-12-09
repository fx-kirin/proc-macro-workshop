use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};
use syn::spanned::Spanned;

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let bname = format!("{}Builder", name);
    let bident = Ident::new(&bname, name.span());
    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        unimplemented!();
    };

    fn ty_is_type(ty: &syn::Type, type_name: &str) -> bool {
        if let syn::Type::Path(ref p) = ty {
            return p.path.segments.len() == 1 && p.path.segments[0].ident == type_name;
        }
        return false;
    }

    fn unwrap_type<'a>(ty: &'a syn::Type, type_name: &'a str) -> &'a syn::Type {
        assert!(ty_is_type(ty, type_name));
        if let syn::Type::Path(ref p) = ty {
            if let syn::PathArguments::AngleBracketed(ref inner_ty) = p.path.segments[0].arguments {
                if inner_ty.args.len() == 1 {
                    let inner_ty = inner_ty.args.first().unwrap();
                    if let syn::GenericArgument::Type(ref t) = inner_ty {
                        return t;
                    }
                }
            }
        }
        panic!("Option type was not Option<T>");
    }
    let optionized = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        if ty_is_type(ty, "Option") {
            quote! { #name : #ty }
        } else if ty_is_type(ty, "Vec") {
            quote! { #name : #ty }
        } else {
            quote! { #name : std::option::Option<#ty> }
        }
    });

    fn extend_methods(f: &syn::Field) -> proc_macro2::TokenStream {
        let name = &f.ident;
        let mut extend_method = std::option::Option::None;
        let mut arg_name = std::option::Option::None;
        for attr in &f.attrs {
            if attr.path.segments.len() == 1 && attr.path.segments[0].ident == "builder" {
                if let std::option::Option::Some(proc_macro2::TokenTree::Group(g)) =
                    attr.tokens.clone().into_iter().next()
                {
                    let mut tokens = g.stream().into_iter();
                    let token = tokens.next().unwrap();
                    if token.to_string() != "each" {
                        return syn::Error::new(attr.path.span().join(attr.tokens.span()).unwrap(), "expected `builder(each = \"...\")`").to_compile_error();
                    }
                    assert_eq!(tokens.next().unwrap().to_string(), "=");
                    let arg = tokens.next().unwrap();
                    let arg = match arg {
                        proc_macro2::TokenTree::Literal(lit) => lit,
                        tt => panic!("expected literal, found {}", tt),
                    };

                    match syn::Lit::new(arg) {
                        syn::Lit::Str(s) => {
                            let arg = syn::Ident::new(&s.value(), s.span());
                            arg_name = std::option::Option::Some(arg.clone());
                            let inner_ty = unwrap_type(&f.ty, "Vec");
                            extend_method = std::option::Option::Some(quote! {
                                pub fn #arg(&mut self, #arg: #inner_ty) -> &mut Self {
                                    self.#name.push(#arg);
                                    self
                                }
                            });
                        }
                        tt => panic!("expected Str, found {:?}", tt),
                    }
                }
            }
        }
        let name = &f.ident;
        let ty = &f.ty;
        let method = if ty_is_type(ty, "Option") {
            let ty = unwrap_type(ty, "Option");
            quote! {
                pub fn #name(&mut self, #name: #ty) -> &mut Self {
                    self.#name = std::option::Option::Some(#name);
                    self
                }
            }
        } else if ty_is_type(ty, "Vec") {
            quote! {
                pub fn #name(&mut self, #name: #ty) -> &mut Self {
                    self.#name = #name;
                    self
                }
            }
        } else {
            quote! {
                pub fn #name(&mut self, #name: #ty) -> &mut Self {
                    self.#name = std::option::Option::Some(#name);
                    self
                }
            }
        };
        if name.is_some() {
            let name = name.as_ref().unwrap();
            if extend_method.is_some() && arg_name.is_some() {
                if name == &arg_name.unwrap() {
                    let extend_method = extend_method.unwrap();
                    return quote! {
                        #extend_method
                    };
                }
            }
        }
        quote! {
            #method
            #extend_method
        }
    }

    let methods = fields.iter().map(|f| extend_methods(f));
    let build_fields = fields.iter().map(|f| {
        let name = &f.ident;
        if ty_is_type(&f.ty, "Vec") {
            quote! {
                #name: vec![]
            }
        } else {
            quote! {
                #name: std::option::Option::None
            }
        }
    });
    let builder_fields = fields.iter().map(|f| {
        let name = &f.ident;
        if ty_is_type(&f.ty, "Option") {
            quote! {
                #name: self.#name.clone()
            }
        } else if ty_is_type(&f.ty, "Vec") {
            quote! {
                #name: self.#name.clone()
            }
        } else {
            quote! {
                #name: self.#name.clone().ok_or(concat!(stringify!(#name), " is not set"))?
            }
        }
    });
    let expanded = quote! {
        pub struct #bident {
            #(#optionized,)*
        }

        impl #bident {
            #(#methods)*

            pub fn build(&mut self) -> std::result::Result<Command, std::boxed::Box<dyn std::error::Error>> {
                Ok(#name{
                    #(#builder_fields,)*
                })
            }
        }

        impl #name {
            fn builder() -> #bident {
                #bident {
                    #(#build_fields,)*
                }

            }
        }
    };
    expanded.into()
}
