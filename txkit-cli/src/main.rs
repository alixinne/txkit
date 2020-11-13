#[macro_use]
extern crate log;

use std::path::PathBuf;

use argh::FromArgs;
use color_eyre::eyre::Result;

fn write_gpu_method_result(
    mut method: Box<dyn txkit_core::method::Method>,
    args: &Args,
) -> Result<()> {
    let width = args.size;
    let height = args.size;

    // Create context
    let mut ctx = txkit_core::context::Context::new_gpu()?;

    // Create target image
    let mut img = txkit_core::image::Image::new_gpu_2d(
        txkit_core::image::ImageDim::new(width, height, 4),
        txkit_core::image::ImageDataType::UInt8,
        &ctx,
    )?;

    // Compute resulting image
    method.compute(&mut ctx, &mut img, None)?;

    // Sync image
    img.sync()?;

    // Map it for reading
    let data = img.data()?;
    image::save_buffer(
        &args.output_path,
        data.as_u8_nd_array().unwrap().as_slice().unwrap(),
        width as u32,
        height as u32,
        image::ColorType::Rgba8,
    )?;

    info!("Wrote {}", args.output_path.display());

    Ok(())
}

fn write_cpu_method_result(
    mut method: Box<dyn txkit_core::method::Method>,
    args: &Args,
) -> Result<()> {
    let width = args.size;
    let height = args.size;

    // Create context
    let mut ctx = txkit_core::context::Context::new_cpu()?;

    // Create target image
    let mut img = txkit_core::image::Image::new_cpu(
        txkit_core::image::ImageDim::new(width, height, 4),
        txkit_core::image::ImageDataType::UInt8,
    );

    // Compute resulting image
    method.compute(&mut ctx, &mut img, None)?;

    // Sync image
    img.sync()?;

    // Map it for reading
    let data = img.data()?;
    image::save_buffer(
        &args.output_path,
        data.as_u8_nd_array().unwrap().as_slice().unwrap(),
        width as u32,
        height as u32,
        image::ColorType::Rgba8,
    )?;

    info!("Wrote {}", args.output_path.display());

    Ok(())
}

#[derive(Debug, FromArgs)]
/// txkit command-line interface
struct Args {
    #[argh(option, short = 'm')]
    /// built-in method to render
    method: String,

    #[argh(option, short = 'o')]
    /// output path
    output_path: PathBuf,

    #[argh(option, short = 's', default = "256")]
    /// size of the output in pixels
    size: usize,

    #[argh(switch)]
    /// force use of the CPU for computing results
    cpu: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::Builder::from_default_env()
        .format_timestamp(None)
        .filter_level(log::LevelFilter::Info)
        .try_init()?;

    let args: Args = argh::from_env();
    let registry = txkit_builtin::methods::new_registry();

    if args.cpu {
        write_cpu_method_result(
            registry
                .build(args.method.as_str())
                .ok_or(txkit_core::Error::MethodNotFound)?,
            &args,
        )
    } else {
        write_gpu_method_result(
            registry
                .build(args.method.as_str())
                .ok_or(txkit_core::Error::MethodNotFound)?,
            &args,
        )
    }
}
