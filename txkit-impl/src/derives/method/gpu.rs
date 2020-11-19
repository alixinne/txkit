use anyhow::{anyhow, Result};
use heck::SnakeCase;
use proc_macro2::TokenStream;
use syn::DeriveInput;

#[derive(Debug)]
pub struct GpuWrappedShader {
    pub path: String,
    pub symbol: String,
}

impl GpuWrappedShader {
    pub fn new(path: String) -> Self {
        let symbol = std::path::PathBuf::from(&path);
        let symbol = symbol.file_name().unwrap().to_string_lossy();
        let symbol = symbol.replace(".", "_").to_snake_case();

        Self { path, symbol }
    }
}

#[derive(Debug)]
pub struct GpuDirectiveProgram {
    pub struct_name: Option<String>,
    pub field_name: String,
    pub shaders: Vec<GpuWrappedShader>,
}

impl GpuDirectiveProgram {
    pub fn parse_from(list: &syn::MetaList) -> Result<Self> {
        let field_name = list
            .path
            .get_ident()
            .ok_or_else(|| anyhow!("expected an identifier, not a path"))?
            .to_string();

        let mut struct_name = None;

        let mut shaders = Vec::new();
        for item in &list.nested {
            match item {
                syn::NestedMeta::Lit(syn::Lit::Str(path)) => {
                    shaders.push(GpuWrappedShader::new(path.value().to_string()));
                }
                syn::NestedMeta::Meta(syn::Meta::NameValue(nv))
                    if nv.path.get_ident().map(|id| *id == "name").unwrap_or(false) =>
                {
                    if let syn::Lit::Str(s) = &nv.lit {
                        struct_name = Some(s.value().to_string());
                    }
                }
                other => return Err(anyhow!("unexpected {:?} in shader list", other)),
            }
        }

        Ok(Self {
            struct_name,
            field_name,
            shaders,
        })
    }
}

#[derive(Debug)]
pub struct GpuDirectiveMethod {
    pub run_program_name: String,
    pub params_struct_name: String,
}

impl GpuDirectiveMethod {
    pub fn parse_from(list: &syn::MetaList) -> Result<Self> {
        let mut run_program_name = None;
        let mut params_struct_name = None;

        for item in &list.nested {
            match item {
                syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) => {
                    match nv.path.get_ident().map(|id| id.to_string()).as_deref() {
                        Some("run") => {
                            if let syn::Lit::Str(s) = &nv.lit {
                                run_program_name = Some(s.value().to_string());
                            } else {
                                return Err(anyhow!(
                                    "unexpected {:?} for run in method directive",
                                    nv.lit
                                ));
                            }
                        }

                        Some("params") => {
                            if let syn::Lit::Str(s) = &nv.lit {
                                params_struct_name = Some(s.value().to_string());
                            } else {
                                return Err(anyhow!(
                                    "unexpected {:?} for params in method directive",
                                    nv.lit
                                ));
                            }
                        }

                        other => {
                            return Err(anyhow!("unexpected {:?} in method directive", other));
                        }
                    }
                }
                other => {
                    return Err(anyhow!("unexpected {:?} in method directive", other));
                }
            }
        }

        Ok(Self {
            run_program_name: run_program_name.ok_or_else(|| {
                anyhow!("missing `run = \"program_name\"` in method specification")
            })?,
            params_struct_name: params_struct_name.ok_or_else(|| {
                anyhow!("missing `params = \"params_struct_name\"` in method specification")
            })?,
        })
    }
}

#[derive(Debug)]
pub struct GpuDirective {
    pub name: String,
    pub programs: Vec<GpuDirectiveProgram>,
    pub method: Option<GpuDirectiveMethod>,
}

impl GpuDirective {
    pub fn parse_from(list: &syn::MetaList) -> Result<Self> {
        let mut name = None;
        let mut programs = Vec::new();
        let mut method = None;

        for item in &list.nested {
            match item {
                syn::NestedMeta::Meta(syn::Meta::NameValue(nv))
                    if nv.path.get_ident().map(|id| *id == "name").unwrap_or(false) =>
                {
                    match &nv.lit {
                        syn::Lit::Str(lit) => {
                            name = Some(lit.value().to_string());
                        }
                        other => {
                            return Err(anyhow!("unexpected {:?} for gpu struct name", other));
                        }
                    }
                }
                syn::NestedMeta::Meta(syn::Meta::List(m))
                    if m.path
                        .get_ident()
                        .map(|id| *id == "method")
                        .unwrap_or(false) =>
                {
                    method = Some(GpuDirectiveMethod::parse_from(&m)?);
                }
                syn::NestedMeta::Meta(syn::Meta::List(program)) => {
                    programs.push(GpuDirectiveProgram::parse_from(program)?);
                }
                other => {
                    return Err(anyhow!("unexpected {:?} in txkit gpu directive", other));
                }
            }
        }

        let name = name.ok_or_else(|| {
            anyhow!("missing `name = \"GpuStructName\"` declaration in txkit directive")
        })?;

        Ok(Self {
            name,
            programs,
            method,
        })
    }
}

fn include_code_for(p: &std::path::Path) -> TokenStream {
    use quote::quote;

    let path = p.to_str().expect("failed to convert path as UTF-8");
    TokenStream::from(quote! {
        const _: &[u8] = include_bytes!(#path);
    })
}

#[cfg(any(feature = "gpu", feature = "gpu45"))]
pub fn process_txkit_gpu_directive(
    input: &DeriveInput,
    list: &syn::MetaList,
) -> Result<(TokenStream, GpuDirective)> {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::iter::FromIterator;
    use std::rc::Rc;

    use quote::{format_ident, quote};
    use tinygl_compiler::{codegen::WrappedItem, model::GlslObject};

    let gpu_directive = GpuDirective::parse_from(list)?;

    let wrapped_code = Rc::new(RefCell::new(Vec::new()));
    let mut track_cb: Box<dyn FnMut(&std::path::Path) -> ()> = {
        let wrapped_code = wrapped_code.clone();
        Box::new(move |p| {
            wrapped_code.borrow_mut().push(include_code_for(p));
        })
    };

    // Initialize GPU compiler
    let mut compiler = tinygl_compiler::Compiler::with_include_callback(true, None, {
        let wrapped_code = wrapped_code.clone();
        Some(Box::new(move |p| {
            wrapped_code.borrow_mut().push(include_code_for(p));
        }))
    })
    .unwrap()
    .with_shaderc();
    let reflector = tinygl_compiler::reflect::SpirVBackend::new();

    // Create the GPU structure name
    let gpu_struct_name = format_ident!("{}", gpu_directive.name);
    let base_path = std::path::PathBuf::from(
        std::env::var_os("CARGO_MANIFEST_DIR").unwrap_or_else(|| std::ffi::OsString::from(".")),
    );

    // TODO: Notify of dependency on file with listener in tinygl_compiler

    // Wrap all shaders from the parsed specification
    let wrapped_shaders: HashMap<&String, _> = {
        let mut result = HashMap::new();

        for program in &gpu_directive.programs {
            for shader in &program.shaders {
                if result.contains_key(&shader.path) {
                    continue;
                }

                // TODO: Might encounter symbolic links that should resolve to the same inode
                let object = if cfg!(feature = "gpu45") {
                    GlslObject::from_path(base_path.join(&shader.path), None)?
                        .track(&mut track_cb)
                        .preprocess(&mut compiler)?
                        .compile(&mut compiler)?
                        .reflect_spirv(&reflector)?
                } else {
                    GlslObject::from_path(base_path.join(&shader.path), None)?
                        .track(&mut track_cb)
                        .compile(&mut compiler)?
                        .reflect_spirv(&reflector)?
                };

                result.insert(
                    &shader.path,
                    compiler.wrap_shader(object, !cfg!(feature = "gpu45"))?,
                );
            }
        }

        result
    };

    // Build wrapped program structures from the wrapped shaders
    let wrapped_programs: HashMap<&String, _> = HashMap::from_iter(
        gpu_directive
            .programs
            .iter()
            .map(|program| {
                Ok((
                    &program.field_name,
                    compiler.wrap_program(
                        &program
                            .shaders
                            .iter()
                            .map(|shader| {
                                wrapped_shaders.get(&shader.path).unwrap()
                                    as &dyn tinygl_compiler::WrappedShaderDetails
                            })
                            .collect::<Vec<_>>()[..],
                        &program
                            .struct_name
                            .as_ref()
                            .map(|s| s.clone())
                            .unwrap_or_else(|| input.ident.to_string()),
                    )?,
                ))
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter(),
    );

    // The GPU struct fields for holding program instances
    let mut gpu_struct_fields = Vec::new();
    // The code to initialize GPU program fields given a context
    let mut gpu_struct_field_initializers = Vec::new();

    // Borrow wrapped_code for the rest of this, we don't need to access the shader callback
    // anymore
    let mut wrapped_code = wrapped_code.borrow_mut();

    // Generate code for programs
    for (program, wrapped_program) in &wrapped_programs {
        // Generate identifiers
        let program_ident = format_ident!("{}", program);
        let program_struct_name = format_ident!("{}", wrapped_program.struct_name());

        // Add field for program
        gpu_struct_fields.push(quote! {
            #program_ident: tinygl::wrappers::GlHandle<#program_struct_name>
        });

        // Add field initializer
        // TODO: Only initialize shaders once, i.e. do not use the build method
        gpu_struct_field_initializers.push(quote! {
            #program_ident: tinygl::wrappers::GlHandle::new(&gl, #program_struct_name::build(&*gl)?),
        });

        // Add generated code for the wrapped program
        wrapped_code.push(wrapped_program.generate()?);
    }

    // Generate code for shaders
    for shader in wrapped_shaders.values() {
        wrapped_code.push(shader.generate()?);
    }

    // Add code for GPU method
    if let Some(method) = &gpu_directive.method {
        // Parse params struct name as a type name
        let params_struct_type: syn::Type = syn::parse_str(&method.params_struct_name)?;
        let program_field_name = format_ident!("{}", method.run_program_name);

        wrapped_code.push(quote! {
            impl ::txkit_core::method::GpuMethod for #gpu_struct_name {
                type Params = #params_struct_type;

                fn compute_gpu(
                    &mut self,
                    ctx: &mut ::txkit_core::context::GpuContext,
                    tgt: &mut ::txkit_core::image::gpu::GpuImageData,
                    params: &Self::Params,
                ) -> ::txkit_core::Result<()> {
                    use ::tinygl::wrappers::ProgramCommonExt;
                    use ::txkit_core::{image::{ImageDataBase, ImageDimGpuExt}, method::GpuMethodParams};

                    let dim = tgt.dim().into_cgmath();
                    ctx.render_to_framebuffer(tgt, |gl, layer| {
                        unsafe {
                            self.#program_field_name.use_program(gl);
                        }

                        // Common parameters
                        self.#program_field_name.set_i_resolution(gl, dim);
                        self.#program_field_name.set_i_layer(gl, layer);

                        // Method parameters
                        params.apply(gl, &self.#program_field_name);

                        unsafe {
                            gl.draw_arrays(tinygl::gl::TRIANGLES, 0, 3);
                        }

                        Ok(())
                    })
                }
            }
        });
    }

    Ok((
        TokenStream::from(quote! {
            pub struct #gpu_struct_name {
                #(#gpu_struct_fields),*
            }

            impl #gpu_struct_name {
                pub fn new(ctx: &txkit_core::context::GpuContext) -> txkit_core::Result<Self> {
                    let gl = ctx.gl.clone();

                    Ok(Self {
                        #(#gpu_struct_field_initializers),*
                    })
                }
            }

            #(#wrapped_code)*
        }),
        gpu_directive,
    ))
}

#[cfg(not(any(feature = "gpu", feature = "gpu45")))]
pub fn process_txkit_gpu_directive(
    _input: &DeriveInput,
    list: &syn::MetaList,
) -> Result<(TokenStream, GpuDirective)> {
    let gpu_directive = GpuDirective::parse_from(list)?;
    Ok((TokenStream::new(), gpu_directive))
}
