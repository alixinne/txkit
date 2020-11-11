mod common;
use common::*;

fn main() -> Result<()> {
    Ok(write_gpu_method_result(
        || txkit::methods::WhiteNoise::new(),
        "white_noise",
    )?)
}
