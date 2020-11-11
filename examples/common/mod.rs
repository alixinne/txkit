pub use anyhow::Result;

use txkit::image::ImageData;

pub fn write_gpu_method_result<T: txkit::method::Method>(
    method_constructor: impl Fn() -> T,
    method_name: &str,
) -> Result<()> {
    const SIZE: usize = 256;

    // Create context
    let mut ctx = txkit::context::Context::new_gpu()?;

    // Create target image
    let mut img = txkit::image::Image::new_gpu_2d(
        txkit::image::ImageDim::new(SIZE, SIZE, 4),
        txkit::image::ImageDataType::UInt8,
        &ctx,
    )?;

    // Create method
    let mut method = method_constructor();

    // Compute resulting image
    method.compute(&mut ctx, &mut img, None)?;

    // Sync image
    img.sync()?;

    // Map it for reading
    let data = img.as_gpu_image().unwrap().data()?;
    let path = format!("{}{}.png", method_name, SIZE);
    image::save_buffer(
        &path,
        data.as_u8_nd_array().unwrap().as_slice().unwrap(),
        SIZE as u32,
        SIZE as u32,
        image::ColorType::Rgba8,
    )?;
    println!("wrote {}", path);

    Ok(())
}
