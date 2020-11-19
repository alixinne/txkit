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
                    let field = field.ident.as_ref().unwrap();
                    let setter_method = format_ident!("set_{}", field);

                    field_setters.push(quote! {
                        p.#setter_method(gl, self.#field);
                    });
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
