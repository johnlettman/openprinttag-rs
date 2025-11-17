use crate::{
    emit::{util::emit_type, EmitIdent, EmitType},
    schema::{builtin::uuid::Uuid, name},
    tracing::trace,
};
use proc_macro2::{Ident, TokenStream};
use quote2::format_ident;
use syn::{parse_quote, Expr, ExprIndex, FnArg, Item, Type, TypeParam};

trait DriveArgValue {
    fn arg_len(&self) -> usize;
    fn type_param(&self) -> Option<TypeParam>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum DeriveArgTy {
    Str(&'static str),
    Uuid,
    NfcUid,
}

impl DeriveArgTy {
    pub const STR_MAX_LEN: usize = 255;
}

impl DriveArgValue for DeriveArgTy {
    #[inline]
    fn arg_len(&self) -> usize {
        match self {
            Self::Str(_) => Self::STR_MAX_LEN,
            Self::Uuid => 16,
            Self::NfcUid => 7,
        }
    }

    #[inline]
    fn type_param(&self) -> Option<TypeParam> {
        match self {
            Self::Str(tv) => {
                let tv = name::to_ident(tv);
                Some(parse_quote!(#tv: AsRef<str>))
            },
            Self::Uuid => None,
            Self::NfcUid => None,
        }
    }
}

impl EmitType for DeriveArgTy {
    fn emit_core_type(&self) -> Option<Type> {
        Some(match self {
            Self::Str(tv) => emit_type(tv),
            Self::Uuid => emit_type("&uuid::Uuid"),
            Self::NfcUid => parse_quote!(&[u8; 7]),
        })
    }
}

#[derive(Debug, Clone)]
struct DeriveArg<V: AsRef<str>>(V, DeriveArgTy);

impl<V: AsRef<str>> DeriveArg<V> {
    pub fn as_fn_arg(&self) -> FnArg {
        let ident = self.rs_core_ident().expect("failed to get ident for UuidCoreDeriveArg");
        let ty = self.1.emit_core_type().expect("failed to get type for UuidCoreDeriveArgTy");
        parse_quote!(#ident: #ty)
    }

    pub fn concat_op(&self) -> TokenStream {
        let ident = self.rs_core_ident().expect("failed to get ident for UuidCoreDeriveArg");
        match self.1 {
            DeriveArgTy::Str(_) => parse_quote! {
                let bytes = #ident.as_ref().as_bytes();
                buf[offset .. offset + bytes.len()].copy_from_slice(bytes);
                offset += bytes.len();
            },
            DeriveArgTy::Uuid => parse_quote! {
                let bytes = #ident.as_bytes();
                buf[offset .. offset + 16].copy_from_slice(bytes);
                offset += 16;
            },
            DeriveArgTy::NfcUid => parse_quote! {
                buf[offset .. offset + 7].copy_from_slice(#ident);
                offset += 7;
            },
        }
    }
}

impl<V: AsRef<str>> EmitIdent for DeriveArg<V> {
    #[inline(always)]
    fn get_name(&self) -> Option<String> {
        Some(self.0.as_ref().to_string())
    }

    #[inline(always)]
    fn rs_core_ident(&self) -> Option<Ident> {
        Some(name::to_ident(&self.0))
    }
}

impl<V: AsRef<str>> DriveArgValue for DeriveArg<V> {
    #[inline(always)]
    fn arg_len(&self) -> usize {
        self.1.arg_len()
    }

    #[inline(always)]
    fn type_param(&self) -> Option<TypeParam> {
        self.1.type_param()
    }
}

impl Uuid {
    pub fn core_derive_fn<N, V>(&self, namespace: N, inputs: Vec<DeriveArg<V>>) -> Item
    where
        N: AsRef<str>,
        V: AsRef<str>,
    {
        let namespace = namespace.as_ref();
        let fn_name = format_ident!("derive_{}_uuid", namespace.to_lowercase());
        let namespace_var = format_ident!("{}_NAMESPACE", namespace.to_uppercase());

        let buf_len: usize = 16 + inputs.iter().map(|v| v.arg_len()).sum::<usize>();

        let fn_args: Vec<_> = inputs.iter().map(|v| v.as_fn_arg()).collect();
        let fn_tvs: Vec<_> = inputs.iter().filter_map(|v| v.type_param()).collect();
        let concat_ops: Vec<_> = inputs.iter().map(|v| v.concat_op()).collect();

        parse_quote! {
            pub fn #fn_name<#(#fn_tvs),*>(#(#fn_args),*) -> Self {
                let mut buf = [0u8; #buf_len];
                let mut offset = 0;

                #(#concat_ops)*

                Self::derive(&Self::#namespace_var, &buf[..offset])
            }
        }
    }

    pub fn core_common_impls(&self) -> Vec<Item> {
        if let Some(ident) = self.rs_core_ident() {
            let brand_derive_fn =
                self.core_derive_fn("brand", vec![DeriveArg("brand_name", DeriveArgTy::Str("B"))]);
            let material_derive_fn = self.core_derive_fn(
                "material",
                vec![
                    DeriveArg("brand_uuid", DeriveArgTy::Uuid),
                    DeriveArg("material_name", DeriveArgTy::Str("M")),
                ],
            );
            let package_derive_fn = self.core_derive_fn(
                "package",
                vec![
                    DeriveArg("brand_uuid", DeriveArgTy::Uuid),
                    DeriveArg("gtin", DeriveArgTy::Str("G")),
                ],
            );
            let instance_derive_fn = self
                .core_derive_fn("instance", vec![DeriveArg("nfc_tag_uid", DeriveArgTy::NfcUid)]);

            vec![
                parse_quote! {
                    impl #ident {
                        pub const BRAND_NAMESPACE: uuid::Uuid =
                            uuid::uuid!("5269dfb7-1559-440a-85be-aba5f3eff2d2");

                        pub const MATERIAL_NAMESPACE: uuid::Uuid =
                            uuid::uuid!("616fc86d-7d99-4953-96c7-46d2836b9be9");

                        pub const PACKAGE_NAMESPACE: uuid::Uuid =
                            uuid::uuid!("6f7d485e-db8d-4979-904e-a231cd6602b2");

                        pub const INSTANCE_NAMESPACE: uuid::Uuid =
                            uuid::uuid!("31062f81-b5bd-4f86-a5f8-46367e841508");

                        #[inline(always)]
                        pub const fn new(uuid: uuid::Uuid) -> Self {
                            Self(uuid)
                        }

                        #[inline(always)]
                        pub const fn as_uuid(&self) -> &uuid::Uuid {
                            &self.0
                        }

                        #[inline(always)]
                        pub const fn into_uuid(self) -> uuid::Uuid {
                            self.0
                        }

                        #[inline(always)]
                        pub fn as_bytes(&self) -> &[u8; 16] {
                            self.0.as_bytes()
                        }

                        #[inline(always)]
                        pub fn is_nil(&self) -> bool {
                            self.0.is_nil()
                        }

                        #[inline(always)]
                        pub fn derive(namespace: &uuid::Uuid, bytes: &[u8]) -> Self {
                            Self(uuid::Uuid::new_v5(namespace, bytes))
                        }

                        #[inline]
                        pub fn derive_uuid(namespace: &uuid::Uuid, uuid: uuid::Uuid) -> Self {
                            Self::derive(namespace, uuid.as_bytes())
                        }

                        #brand_derive_fn
                        #material_derive_fn
                        #package_derive_fn
                        #instance_derive_fn
                    }
                },
                parse_quote! {
                    impl core::fmt::Display for #ident {
                        #[inline(always)]
                        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                                self.0.fmt(f)
                        }
                    }
                },
                parse_quote! {
                    impl Default for Uuid {
                        #[inline(always)]
                        fn default() -> Self {
                            Self(uuid::Uuid::nil())
                        }
                    }
                },
            ]
        } else {
            Vec::new()
        }
    }
}
