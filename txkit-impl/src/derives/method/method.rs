use anyhow::{anyhow, Result};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::DeriveInput;

pub fn process_txkit_method_directive(
    input: &DeriveInput,
    list: &syn::MetaList,
    gpu_directives: &[super::gpu::GpuDirective],
    cpu_directives: &[super::cpu::CpuDirective],
) -> Result<TokenStream> {
    // Ensure the list is formatted correctly
    if list.nested.len() != 0 {
        return Err(anyhow!(
            "unexpected tokens in top-level method txkit directive"
        ));
    }

    let struct_name = &input.ident;

    let cpu_struct_name = cpu_directives
        .iter()
        .filter_map(|cpud| {
            if cpud.method.is_some() {
                Some(())
            } else {
                None
            }
        })
        .next();

    let gpu_struct_name = gpu_directives
        .iter()
        .filter_map(|gpud| {
            if gpud.method.is_some() {
                Some(&gpud.name)
            } else {
                None
            }
        })
        .next();

    let gpu_code = if let Some(gpu_s_name) = gpu_struct_name {
        let gpu_s_name = format_ident!("{}", gpu_s_name);

        quote! {
            #[cfg(feature = "gpu")]
            Context::Gpu(gpu_context) => tgt
                .as_gpu_image_mut()
                .ok_or_else(|| Error::FormatNotSupported)
                .and_then(|tgt| {
                    use ::txkit_core::method::GpuMethod;

                    // Initialize GPU if needed
                    if let None = self.gpu {
                        self.gpu = Some(#gpu_s_name::new(gpu_context)?);
                    }

                    // Compute result using initialized GPU resources
                    let gpu = self.gpu.as_mut().unwrap();
                    gpu.compute_gpu(gpu_context, tgt, params)
                }),
            #[cfg(not(feature = "gpu"))]
            Context::Gpu(_) => Err(Error::ContextNotSupported),
        }
    } else {
        quote! {
            Context::Gpu(_) => Err(Error::ContextNotSupported),
        }
    };

    let cpu_code = if let Some(_) = cpu_struct_name {
        quote! {
            #[cfg(feature = "cpu")]
            Context::Cpu(cpu_context) => {
                use ::txkit_core::method::CpuMethod;
                self.compute_cpu(cpu_context, tgt, params)
            },
            #[cfg(not(feature = "cpu"))]
            Context::Cpu(_) => Err(Error::ContextNotSupported),
        }
    } else {
        quote! {
            Context::Cpu(_) => Err(Error::ContextNotSupported),
        }
    };

    let params_type: syn::Type = syn::parse_str(
        &gpu_directives
            .iter()
            .filter_map(|gpud| gpud.method.as_ref().map(|m| m.params_struct_name.as_str()))
            .chain(
                cpu_directives
                    .iter()
                    .filter_map(|cpud| cpud.method.as_ref().map(|m| m.params_struct_name.as_str())),
            )
            .next()
            .ok_or_else(|| {
                anyhow!("no directive found declaring a params type, cannot implement method")
            })?,
    )?;

    // Generate the impl
    Ok(TokenStream::from(quote! {
        impl ::txkit_core::method::Method for #struct_name {
            fn compute(
                &mut self,
                ctx: &mut ::txkit_core::context::Context,
                tgt: &mut ::txkit_core::image::Image,
                params: Option<&dyn std::any::Any>,
            ) -> ::txkit_core::Result<()> {
                use ::txkit_core::{context::Context, Error};
                let mut default_params: Option<#params_type> = None;
                let params = ::txkit_core::method::downcast_params(params, &mut default_params)?;

                match ctx {
                    #gpu_code
                    #cpu_code
                }
            }
        }
    }))
}
