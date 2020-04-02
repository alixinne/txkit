decl_method! {
    name Whitenoise;
    gpu {
        name WhitenoiseGpu;
        program crate::shaders::WhitenoiseProgram;
    }
    cpu ((k, j, i, l), sz) -> f32 => {
        let mut x = ((i + j * sz.width + k * sz.width * sz.height) * sz.channels + l) as u32;

        // Hash
        x = ((x >> 16) ^ x).wrapping_mul(0x45d9f3bu32);
        x = ((x >> 16) ^ x).wrapping_mul(0x45d9f3bu32);
        x = (x >> 16) ^ x;

        // Convert to float
        f32::from_bits(0x7fu32 << 23 | x >> 9) - 1.0f32
    }
}
