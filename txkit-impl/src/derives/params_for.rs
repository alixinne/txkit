use anyhow::{anyhow, Context, Result};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::DeriveInput;

struct ParamsForDirective {
    target_names: Vec<String>,
}

impl ParamsForDirective {
    pub fn parse_from(list: &syn::MetaList) -> Result<Self> {
        let mut target_names = Vec::with_capacity(list.nested.len());

        for item in &list.nested {
            if let syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) = item {
                match nv.path.get_ident().map(|id| id.to_string()).as_deref() {
                    Some("program") => {
                        if let syn::Lit::Str(s) = &nv.lit {
                            target_names.push(s.value().to_string());
                            continue;
                        }
                    }
                    _ => {}
                }
            }

            return Err(anyhow!(
                "unexpected {:?} in ParamsFor txkit directive",
                item
            ));
        }

        Ok(Self { target_names })
    }
}

fn process_txkit_directive(input: &DeriveInput, list: &syn::MetaList) -> Result<TokenStream> {
    let struct_name = &input.ident;
    let params_for_directive = ParamsForDirective::parse_from(list)?;
    let mut generated = Vec::new();

    // Generate field setters
    let field_setters = {
        let mut field_setters = Vec::new();

        match &input.data {
            syn::Data::Struct(ds) => {
                for field in &ds.fields {
                    let field_name = field.ident.as_ref().unwrap();
                    let mut has_io_attrs = false;

                    for attr in &field.attrs {
                        let name = attr.path.get_ident().map(|id| id.to_string());
                        let is_image = name.as_ref().map(|n| n == "image_io").unwrap_or(false);
                        let is_texture = name.as_ref().map(|n| n == "texture_io").unwrap_or(false);

                        if !is_image && !is_texture {
                            continue;
                        }

                        has_io_attrs = true;

                        match attr.parse_meta()? {
                            syn::Meta::List(list) => {
                                for io_field in list.nested {
                                    match io_field {
                                        syn::NestedMeta::Meta(syn::Meta::List(list))
                                            if list.path.get_ident().is_some() =>
                                        {
                                            let get_binding_method = format_ident!(
                                                "get_{}_binding",
                                                list.path.get_ident().unwrap()
                                            );

                                            let get_format_method = format_ident!(
                                                "get_{}_format",
                                                list.path.get_ident().unwrap()
                                            );

                                            if is_image {
                                                let args: Vec<_> = list.nested.iter().collect();
                                                let access_arg = &args[0];

                                                field_setters.push(quote! {
                                                    self.#field_name.apply_image_binding(gl, p.#get_binding_method() as _, #access_arg, p.#get_format_method());
                                                });
                                            } else if is_texture {
                                                return Err(anyhow!("unexpected flags for texture binding for `{}` on field `{}`", list.path.get_ident().unwrap(), field_name));
                                            }
                                        }
                                        syn::NestedMeta::Meta(syn::Meta::Path(p))
                                            if p.get_ident().is_some() =>
                                        {
                                            let get_binding_method = format_ident!(
                                                "get_{}_binding",
                                                p.get_ident().unwrap()
                                            );

                                            if is_image {
                                                return Err(anyhow!("image binding for `{}` on field `{}` requires access and format flags", p.get_ident().unwrap(), field_name));
                                            } else if is_texture {
                                                field_setters.push(quote! {
                                                    self.#field_name.apply_texture_binding(gl, p.#get_binding_method() as _);
                                                });
                                            }
                                        }
                                        _ => {
                                            return Err(anyhow!(
                                                "invalid io field specification on field `{}`",
                                                field_name
                                            ));
                                        }
                                    }
                                }
                            }
                            _ => {
                                return Err(anyhow!(
                                    "invalid io attribute on field `{}`",
                                    field_name
                                ));
                            }
                        }
                    }

                    if !has_io_attrs {
                        let setter_method = format_ident!("set_{}", field_name);

                        field_setters.push(quote! {
                            p.#setter_method(gl, self.#field_name);
                        });
                    }
                }
            }
            _ => {
                return Err(anyhow!("unnamed structs are not supported by txkit"));
            }
        }

        field_setters
    };

    for program in &params_for_directive.target_names {
        let ty: syn::Type = syn::parse_str(program)?;

        generated.push(quote! {
            #[cfg(any(feature = "gpu", feature = "gpu45"))]
            impl ::txkit_core::method::GpuMethodParams<#ty> for #struct_name {
                fn apply(&self, gl: &::tinygl::Context, p: &#ty) {
                    use ::txkit_core::io::gpu::GpuImageIoExt;
                    #(#field_setters)*
                }
            }
        });
    }

    Ok(TokenStream::from(quote! { #(#generated)* }))
}

pub fn process_params_for(input: DeriveInput) -> Result<TokenStream> {
    let mut generated: Vec<TokenStream> = Vec::new();

    crate::util::process_directive(
        &input.attrs,
        |list| {
            generated.push(
                process_txkit_directive(&input, list)
                    .context("failed to process txkit directive")?,
            );
            Ok(())
        },
        "txkit",
        false,
    )?;

    Ok(TokenStream::from(quote! {
        #(#generated)*
    }))
}
