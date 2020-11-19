use anyhow::{anyhow, Context, Result};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

mod cpu;
mod gpu;
mod method;

fn process_txkit_directive(input: &DeriveInput, list: &syn::MetaList) -> Result<TokenStream> {
    let mut generated = Vec::new();

    let mut gpu_directives = Vec::new();
    let mut cpu_directives = Vec::new();

    // Read CPU and GPU directives
    for item in &list.nested {
        match &item {
            syn::NestedMeta::Meta(syn::Meta::List(list))
                if list.path.get_ident().map(|id| id == "gpu").unwrap_or(false) =>
            {
                let (tokens, gpu_directive) = gpu::process_txkit_gpu_directive(input, list)?;
                gpu_directives.push(gpu_directive);
                generated.push(tokens);
            }
            syn::NestedMeta::Meta(syn::Meta::List(list))
                if list.path.get_ident().map(|id| id == "cpu").unwrap_or(false) =>
            {
                let (tokens, cpu_directive) = cpu::process_txkit_cpu_directive(input, list)?;
                cpu_directives.push(cpu_directive);
                generated.push(tokens);
            }
            syn::NestedMeta::Meta(syn::Meta::List(list))
                if list
                    .path
                    .get_ident()
                    .map(|id| id == "method")
                    .unwrap_or(false) =>
            {
                // ignore for now, 2nd pass
            }
            other => {
                return Err(anyhow!("unexpected {:?} in txkit directive", other));
            }
        }
    }

    // Read method directive
    for item in &list.nested {
        match &item {
            syn::NestedMeta::Meta(syn::Meta::List(list))
                if list.path.get_ident().map(|id| id == "gpu").unwrap_or(false) =>
            {
                // ignore
            }
            syn::NestedMeta::Meta(syn::Meta::List(list))
                if list.path.get_ident().map(|id| id == "cpu").unwrap_or(false) =>
            {
                // ignore
            }
            syn::NestedMeta::Meta(syn::Meta::List(list))
                if list
                    .path
                    .get_ident()
                    .map(|id| id == "method")
                    .unwrap_or(false) =>
            {
                generated.push(method::process_txkit_method_directive(
                    input,
                    list,
                    &gpu_directives,
                    &cpu_directives,
                )?);
            }
            other => {
                return Err(anyhow!("unexpected {:?} in txkit directive", other));
            }
        }
    }

    Ok(TokenStream::from(quote! {
        #(#generated)*
    }))
}

pub fn process_method(input: DeriveInput) -> Result<TokenStream> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use quote::format_ident;

    #[test]
    fn test() {
        let di = DeriveInput {
            attrs: vec![syn::Attribute {
                pound_token: syn::token::Pound(proc_macro2::Span::call_site()),
                style: syn::AttrStyle::Outer,
                bracket_token: syn::token::Bracket(proc_macro2::Span::call_site()),
                path: syn::Path::from(format_ident!("gpu")),
                tokens: quote! { (vertex = "../txkit-builtin/shaders/quad.vert", fragment = "../txkit-builtin/shaders/debug.frag", program) },
            }],
            vis: syn::Visibility::Public(syn::VisPublic {
                pub_token: syn::token::Pub(proc_macro2::Span::call_site()),
            }),
            ident: format_ident!("Debug"),
            generics: syn::Generics {
                lt_token: None,
                params: syn::punctuated::Punctuated::new(),
                gt_token: None,
                where_clause: None,
            },
            data: syn::Data::Struct(syn::DataStruct {
                struct_token: syn::token::Struct(proc_macro2::Span::call_site()),
                fields: syn::Fields::Unit {},
                semi_token: None,
            }),
        };

        eprintln!("output: {}", process_method(di).unwrap());
    }
}
