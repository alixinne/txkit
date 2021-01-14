#[macro_use]
extern crate log;

use std::io::prelude::*;
use std::path::PathBuf;

use argh::FromArgs;
use color_eyre::eyre::Result;

fn write_method_result(
    width: u32,
    height: u32,
    data: &dyn txkit_core::image::MappedImageData,
    args: &Args,
) -> Result<()> {
    if let Some(output_path) = &args.output_path {
        image::save_buffer(
            &output_path,
            data.as_u8_nd_array().unwrap().as_slice().unwrap(),
            width,
            height,
            image::ColorType::Rgba8,
        )?;

        info!("Wrote {}", output_path.display());
    } else if let Ok(_) = std::env::var("KITTY_WINDOW_ID") {
        // We are probably running under Kitty terminal emulator, send image using
        // TODO: Actually check for support using the query command

        // Buffer for raw PNG data
        let mut buf = Vec::new();
        // PNG encoder
        let encoder = image::codecs::png::PngEncoder::new(&mut buf);
        // Write PNG to buffer
        encoder.encode(
            data.as_u8_nd_array().unwrap().as_slice().unwrap(),
            width,
            height,
            image::ColorType::Rgba8,
        )?;
        // Encode to base64
        let encoded = base64::encode(&buf);
        // Split into chunks
        let chunks = encoded.as_bytes().chunks(4096).collect::<Vec<_>>();
        // Transmit chunks
        let mut out = std::io::stdout();
        for (i, chunk) in chunks.iter().enumerate() {
            let last = if i == chunks.len() - 1 { b"0" } else { b"1" };

            match i {
                0 => {
                    // First chunk
                    out.write_all(b"\x1B_Gf=100,a=T,m=")?;
                }
                _ => {
                    // Other chunks
                    out.write_all(b"\x1B_Gm=")?;
                }
            }

            out.write_all(last)?;
            out.write_all(b";")?;
            out.write_all(chunk)?;
            out.write_all(b"\x1B\\")?;
        }

        // Finish with new-line
        out.write_all(b"\n")?;
    } else {
        warn!("no output method");
    }

    Ok(())
}

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
    img.download()?;

    // Map it for reading
    let data = img.data()?;
    write_method_result(width as u32, height as u32, &*data, args)?;

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
    img.download()?;

    // Map it for reading
    let data = img.data()?;
    write_method_result(width as u32, height as u32, &*data, args)?;

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
    output_path: Option<PathBuf>,

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
