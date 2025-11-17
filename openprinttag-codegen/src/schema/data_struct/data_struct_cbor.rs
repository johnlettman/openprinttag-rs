use crate::{
    emit::{EmitIdent, EmitType},
    schema::{name, DataStruct},
};
use proc_macro2::{Span, TokenStream};
use quote2::format_ident;
use syn::{parse_quote, Item};

impl DataStruct {
    pub fn core_cbor_impls(&self) -> Vec<Item> {
        if let Some(ident) = self.rs_core_ident() {
            let context = name::to_ident("c");
            let decoder = name::to_ident("d");
            let position = name::to_ident("p");

            struct FieldComponent {
                decode_vars_expr: TokenStream,
                decode_store_expr: TokenStream,
                decode_match_arm: syn::Arm,
            }

            let field_components: Vec<FieldComponent> = self.fields.iter().filter_map(|f| {
                let field_key = f.key as i64;
                let field_ty = f.emit_core_type();
                let field_name = f.name.clone()?;
                let field_ident = name::to_ident(&field_name);
                let field_ref = name::to_field_reference(&self.schema_name, field_name.as_str());
                let field_ref_lit = syn::LitStr::new(&field_ref, Span::call_site());


                Some(FieldComponent {
                    decode_vars_expr: parse_quote!(let mut #field_ident: core::option::Option<#field_ty> = None;),
                    decode_match_arm: parse_quote! {
                        #field_key => {
                            match minicbor::Decode::decode(#decoder, #context) {
                                Ok(v) => #field_ident = Some(v),
                                Err(e) if e.is_unknown_variant() => #decoder.skip()?,
                                Err(e) => return Err(e)
                            }
                        }
                    },

                    decode_store_expr: parse_quote! {
                        #field_ident: if let Some(v) = #field_ident {
                            v
                        } else if let Some(z) = <#field_ty as minicbor::Decode::<C>>::nil() {
                            z
                        } else {
                            return Err(minicbor::decode::Error::missing_value(2).with_message(#field_ref_lit).at(#position))
                        }
                    }
                })
            }).collect();

            let decode_vars_exprs: Vec<_> =
                field_components.iter().map(|c| &c.decode_vars_expr).collect();
            let decode_match_arms: Vec<_> =
                field_components.iter().map(|c| &c.decode_match_arm).collect();
            let decode_store_exprs: Vec<_> =
                field_components.iter().map(|c| &c.decode_store_expr).collect();

            vec![
                parse_quote! {
                    impl<'b, C> minicbor::Decode<'b, C> for #ident {
                        fn decode(#decoder: &mut minicbor::Decoder<'b>, #context: &mut C) -> core::result::Result<#ident, minicbor::decode::Error> {
                            let #position = #decoder.position();

                            #(#decode_vars_exprs)*

                            if let Some(map_len) = #decoder.map()? {
                                for _ in 0..map_len {
                                    match #decoder.i64()? {
                                        #(#decode_match_arms)*
                                        _ => #decoder.skip()?
                                    }
                                }
                            } else {
                                while minicbor::data::Type::Break != #decoder.datatype()? {
                                    match #decoder.i64()? {
                                        #(#decode_match_arms)*
                                        _ => #decoder.skip()?
                                    }
                                }
                                #decoder.skip()?
                            }

                            Ok(#ident { #(#decode_store_exprs),* })
                        }
                    }
                }
            ]
        } else {
            Vec::new()
        }
    }
}
