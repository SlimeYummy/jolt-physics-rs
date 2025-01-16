extern crate proc_macro;
use proc_macro::TokenStream;
use punctuated::Punctuated;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::*;

#[proc_macro_attribute]
pub fn vtable(attr: TokenStream, item: TokenStream) -> TokenStream {
    let allow_empty = attr.to_string() == "allow_empty";
    let mut input = parse_macro_input!(item as ItemStruct);

    let repr = extract_attr_raw(&input.attrs, "repr");
    if repr != "C" {
        return Error::new(
            proc_macro2::Span::call_site(),
            "Miss `#[repr(C)]` for structure",
        )
        .to_compile_error()
        .into();
    }

    let fields = match &mut input.fields {
        Fields::Named(fields) => fields,
        _ => {
            return Error::new(
                proc_macro2::Span::call_site(),
                "Only supported for structure with named fields",
            )
            .to_compile_error()
            .into()
        }
    };

    let vtable_name = input.ident.to_string();
    if !vtable_name.ends_with("VTable") {
        return Error::new(
            input.ident.span(),
            "The structure does not ends in 'VTable'",
        )
        .to_compile_error()
        .into();
    }

    let trait_name = Ident::new(&vtable_name[..vtable_name.len() - 6], input.ident.span());
    let vtable_name = input.ident.clone();

    let mut generated_trait = ItemTrait {
        attrs: input
            .attrs
            .iter()
            .filter(|a| {
                a.path()
                    .get_ident()
                    .as_ref()
                    .map(|i| *i == "doc")
                    .unwrap_or(false)
            })
            .cloned()
            .collect(),
        vis: Visibility::Public(Default::default()),
        unsafety: None,
        auto_token: None,
        trait_token: Default::default(),
        ident: trait_name.clone(),
        generics: Generics::default(),
        colon_token: Default::default(),
        supertraits: {
            let mut supertraits = Punctuated::new();
            supertraits.push(parse_str::<TypeParamBound>(&format!("crate::vtable::VData<{}>", vtable_name)).unwrap());
            supertraits
        },
        brace_token: Default::default(),
        items: Default::default(),
        restriction: Default::default(),
    };

    let mut vtable_ctor = vec![];
    let mut drop_idx: Option<usize> = None;

    let mut impl_empty_funcs = vec![];

    for (field_idx, field) in fields.named.iter_mut().enumerate() {
        if !matches!(field.vis, Visibility::Public(_)) {
            return Error::new(field.ty.span(), "VTable fields must be public")
                .to_compile_error()
                .into();
        }

        let ident = field.ident.as_ref().unwrap();

        let func_ty = match &mut field.ty {
            Type::BareFn(f) => f,
            _ => {
                return Error::new(field.ty.span(), "VTable fields must be bare functions")
                    .to_compile_error()
                    .into()
            }
        };

        let sig = Signature {
            constness: None,
            asyncness: None,
            unsafety: func_ty.unsafety,
            abi: None,
            fn_token: func_ty.fn_token,
            ident: ident.clone(),
            generics: Default::default(),
            paren_token: func_ty.paren_token,
            inputs: Default::default(),
            variadic: None,
            output: func_ty.output.clone(),
        };

        let mut forward_code = None;

        let mut sig_extern = sig.clone();
        sig_extern.unsafety = Some(Default::default());
        sig_extern.abi = Some(parse_str("extern \"C\"").unwrap());
        sig_extern.generics = parse_str(&format!("<VD : {}>", trait_name)).unwrap();

        let mut sig_trait = sig.clone();

        for (input_idx, param) in func_ty.inputs.iter().enumerate() {
            let arg_name = quote::format_ident!("_{}", sig_extern.inputs.len());
            let typed_arg = FnArg::Typed(PatType {
                attrs: param.attrs.clone(),
                pat: Box::new(Pat::Path(syn::PatPath {
                    attrs: Default::default(),
                    qself: None,
                    path: arg_name.clone().into(),
                })),
                colon_token: Default::default(),
                ty: Box::new(param.ty.clone()),
            });
            sig_extern.inputs.push(typed_arg.clone());

            if input_idx == 0 {
                let (ok, mutable) = match_first_param(&param.ty);
                if !ok {
                    return Error::new(
                        param.ty.span(),
                        "First parameter must be `*const u8` or `*mut u8`",
                    )
                    .to_compile_error()
                    .into();
                }

                if ident == "drop" && !mutable {
                    return Error::new(param.ty.span(), "First parameter must be `*mut u8`")
                        .to_compile_error()
                        .into();
                }

                if mutable {
                    sig_trait.inputs.push(parse_str("&mut self").unwrap());
                } else {
                    sig_trait.inputs.push(parse_str("&self").unwrap());
                }
            } else {
                sig_trait.inputs.push(typed_arg.clone());
                forward_code = Some(quote!(#forward_code #arg_name,));
            }
        }

        // Add unsafe: The function are not safe to call unless the self parameter is of the correct type
        func_ty.unsafety = Some(Default::default());

        // Add extern "C" if it isn't there
        if let Some(a) = &func_ty.abi {
            if !a.name.as_ref().map(|s| s.value() == "C").unwrap_or(false) {
                return Error::new(a.span(), "invalid ABI")
                    .to_compile_error()
                    .into();
            }
        } else {
            func_ty.abi.clone_from(&sig_extern.abi);
        }

        if ident.to_string() == "drop" {
            vtable_ctor.push(quote!(drop: {
                #sig_extern {
                    unimplemented!("Drops in C++ side are not supported now.");
                }
                drop::<VD>
            },));

            #[cfg(not(target_os = "windows"))]
            vtable_ctor.push(quote!(_drop_padding_: 0,));
            drop_idx = Some(field_idx);
        } else {
            vtable_ctor.push(quote!(#ident: {
                #sig_extern {
                    let _0 = _0 as *mut crate::vtable::VPair<VD, #vtable_name>;
                    (&mut *_0).vdata.#ident(#forward_code)
                }
                #ident::<VD>
            },));

            generated_trait.items.push(TraitItem::Fn(TraitItemFn {
                attrs: field.attrs.clone(),
                sig: sig_trait.clone(),
                default: None,
                semi_token: Some(Default::default()),
            }));

            impl_empty_funcs.push(quote!( #sig_trait { unreachable!(); } ));
        }
    }

    if drop_idx.is_some() {
        #[cfg(not(target_os = "windows"))]
        fields.named.insert(
            drop_idx.unwrap() + 1,
            Field {
                attrs: Vec::new(),
                vis: Visibility::Inherited,
                mutability: FieldMutability::None,
                ident: Some(Ident::new("_drop_padding_", vtable_name.span())),
                colon_token: None,
                ty: parse_str("usize").unwrap(),
            },
        );
    }
    
    let mut impl_empty = None;
    if allow_empty {
        impl_empty = Some(quote!(
            unsafe impl crate::vtable::VData<#vtable_name> for () {}
            impl #trait_name for () {
                #(#impl_empty_funcs)*
            }
        ));
    }

    let result = quote!(
        #input

        unsafe impl crate::vtable::VTable for #vtable_name {}

        impl #vtable_name {
            #[allow(improper_ctypes_definitions)]
            pub const fn build_vtable<VD: #trait_name>() -> Self {
                Self {
                    #(#vtable_ctor)*
                }
            }
        }

        #generated_trait

        #impl_empty
    );
    result.into()
}

fn extract_attr_raw(attrs: &[Attribute], name: &str) -> String {
    let mut raw = String::new();
    if let Some(attr) = attrs.iter().find(|attr| attr.path().is_ident(name)) {
        let _ = attr.parse_nested_meta(|meta| {
            raw = meta.path.into_token_stream().to_string();
            Ok(())
        });
    }
    raw
}

fn match_first_param(ty: &Type) -> (bool, bool) {
    if let Type::Ptr(ptr) = ty {
        if let Type::Path(path) = &*ptr.elem {
            if let Some(ident) = path.path.get_ident() {
                if ident == "u8" {
                    return (true, ptr.mutability.is_some());
                }
            }
        }
    }
    (false, false)
}

#[proc_macro_attribute]
pub fn vdata(attr: TokenStream, item: TokenStream) -> TokenStream {
    let vtable_type = parse_macro_input!(attr as Ident);
    let input = parse_macro_input!(item as ItemStruct);

    let vdata_name = input.ident.clone();
    let static_name = Ident::new(&format!("{}_VTABLE", vdata_name), vtable_type.span());

    let result = quote!(
        #input

        unsafe impl jolt_physics_rs::vtable::VData<#vtable_type> for #vdata_name {}

        #[allow(non_upper_case_globals)]
        static #static_name: #vtable_type = #vtable_type::build_vtable::<#vdata_name>();

        impl #vdata_name {
            pub const fn new_vpair(vdata: #vdata_name) -> jolt_physics_rs::vtable::VPair<#vdata_name, #vtable_type> {
                unsafe { jolt_physics_rs::vtable::VPair::new(&#static_name, vdata) }
            }

            pub fn new_vbox(vdata: #vdata_name) -> jolt_physics_rs::vtable::VBox<#vdata_name, #vtable_type> {
                std::boxed::Box::new(unsafe { jolt_physics_rs::vtable::VPair::new(&#static_name, vdata) })
            }
        }
    );
    result.into()
}
