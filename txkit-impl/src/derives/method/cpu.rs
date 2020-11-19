use anyhow::{anyhow, Result};
use proc_macro2::TokenStream;
use syn::DeriveInput;

#[derive(Debug)]
pub enum CpuDirectiveMethodKind {
    Iter { path: String },
}

impl CpuDirectiveMethodKind {
    pub fn parse_from(nv: &syn::MetaNameValue) -> Result<Self> {
        match nv.path.get_ident().map(|id| id.to_string()).as_deref() {
            Some("iter") => match &nv.lit {
                syn::Lit::Str(s) => Ok(Self::Iter {
                    path: s.value().to_string(),
                }),
                _ => Err(anyhow!("unexpected {:?} for cpu method", nv)),
            },
            _ => Err(anyhow!("unexpected {:?} for cpu method", nv)),
        }
    }
}

#[derive(Debug)]
pub struct CpuDirectiveMethod {
    pub kind: CpuDirectiveMethodKind,
    pub params_struct_name: String,
}

impl CpuDirectiveMethod {
    pub fn parse_from(list: &syn::MetaList) -> Result<Self> {
        let mut kind = None;
        let mut params = None;

        for item in &list.nested {
            match item {
                syn::NestedMeta::Meta(syn::Meta::NameValue(nv))
                    if nv.path.get_ident().map(|id| *id == "iter").unwrap_or(false) =>
                {
                    kind = Some(CpuDirectiveMethodKind::parse_from(nv)?);
                }
                syn::NestedMeta::Meta(syn::Meta::NameValue(syn::MetaNameValue {
                    path,
                    lit: syn::Lit::Str(s),
                    ..
                })) if path.get_ident().map(|id| *id == "params").unwrap_or(false) => {
                    params = Some(s.value().to_string());
                }
                _ => {}
            }
        }

        Ok(Self {
            kind: kind
                .ok_or_else(|| anyhow!("missing `iter = \"...\"` in cpu directive method"))?,
            params_struct_name: params
                .ok_or_else(|| anyhow!("missing `params = \"...\"` in cpu directive method"))?,
        })
    }
}

#[derive(Debug)]
pub struct CpuDirective {
    pub method: Option<CpuDirectiveMethod>,
}

impl CpuDirective {
    pub fn parse_from(list: &syn::MetaList) -> Result<Self> {
        let mut method = None;

        for item in &list.nested {
            match item {
                syn::NestedMeta::Meta(syn::Meta::List(list))
                    if list
                        .path
                        .get_ident()
                        .map(|id| *id == "method")
                        .unwrap_or(false) =>
                {
                    method = Some(CpuDirectiveMethod::parse_from(list)?);
                    continue;
                }
                _ => {}
            }

            return Err(anyhow!("unexpected {:?} in cpu directive", item));
        }

        Ok(Self { method })
    }
}

#[cfg(feature = "cpu")]
pub fn process_txkit_cpu_directive(
    input: &DeriveInput,
    list: &syn::MetaList,
) -> Result<(TokenStream, CpuDirective)> {
    use quote::quote;

    let cpu_directive = CpuDirective::parse_from(list)?;

    let mut generated = Vec::new();

    if let Some(method) = &cpu_directive.method {
        let struct_name = &input.ident;
        let params_struct: syn::Type = syn::parse_str(&method.params_struct_name)?;
        // TODO: Support other types
        let CpuDirectiveMethodKind::Iter { path } = &method.kind;
        let path: syn::Path = syn::parse_str(path)?;

        generated.push(quote! {
            #[cfg(feature = "cpu")]
            impl ::txkit_core::method::CpuMethod for #struct_name {
                type Params = #params_struct;

                fn compute_cpu(
                    &mut self,
                    ctx: &mut ::txkit_core::context::CpuContext,
                    tgt: &mut ::txkit_core::image::Image,
                    params: &Self::Params,
                ) -> ::txkit_core::Result<()> {
                    use ::txkit_core::image::IntoElementType;
                    use ::ndarray::par_azip;

                    let dim = tgt.dim();
                    let mut data_mut = tgt.data_mut()?;

                    if let Some(data) = data_mut.as_u8_nd_array_mut() {
                        ctx.thread_pool.install(|| {
                            par_azip!((index idx, o in data) {
                                *o = #path(idx, dim, params).into_u8();
                            });
                        });

                        Ok(())
                    } else if let Some(data) = data_mut.as_f32_nd_array_mut() {
                        ctx.thread_pool.install(|| {
                            par_azip!((index idx, o in data) {
                                *o = #path(idx, dim, params).into_f32();
                            });
                        });

                        Ok(())
                    } else {
                        Err(::txkit_core::Error::FormatNotSupported)
                    }
                }
            }
        });
    }

    Ok((TokenStream::from(quote! { #(#generated)* }), cpu_directive))
}

#[cfg(not(feature = "cpu"))]
pub fn process_txkit_cpu_directive(
    _input: &DeriveInput,
    list: &syn::MetaList,
) -> Result<(TokenStream, CpuDirective)> {
    let cpu_directive = CpuDirective::parse_from(list)?;
    Ok((TokenStream::new(), cpu_directive))
}
