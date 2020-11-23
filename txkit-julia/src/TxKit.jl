"""
    TxKit

Main module for interfacing with the txkit texture generation library.
"""
module TxKit

# Load binary dependency
const deps_file = joinpath(dirname(@__FILE__), "..", "deps", "deps.jl")
if !isfile(deps_file)
    error("TxKit.jl is not installed properly, run Pkg.build(\"TxKit\") and restart Julia.")
end
include(deps_file)

function __init__()
    check_deps()
end

# C function API
module Api
import ..libtxkit_core, ..libtxkit_builtin

const ImageDataType = UInt32
const ImageDataType_UInt8 = ImageDataType(0)
const ImageDataType_Float32 = ImageDataType(1)

const Context = Ptr{Cvoid}

const Image = Ptr{Cvoid}

const MappedImageDataRead = Ptr{Cvoid}
const MappedImageDataWrite = Ptr{Cvoid}

const Method = Ptr{Cvoid}

const Registry = Ptr{Cvoid}

struct ImageDim
    width::UInt
    height::UInt
    depth::UInt
    channels::UInt
end

txkit_context_destroy(ctx::Context) = ccall((:txkit_context_destroy, libtxkit_core), Cvoid, (Context,), ctx)
txkit_context_new_cpu() = ccall((:txkit_context_new_cpu, libtxkit_core), Context, ())
txkit_context_new_gpu() = ccall((:txkit_context_new_gpu, libtxkit_core), Context, ())

txkit_get_last_error() = ccall((:txkit_get_last_error, libtxkit_core), Ptr{Cchar}, ())

txkit_image_destroy(image::Image) = ccall((:txkit_image_destroy, libtxkit_core), Cvoid, (Image,), image)
txkit_image_dim(image::Image) = ccall((:txkit_image_dim, libtxkit_core), ImageDim, (Image,), image)
txkit_image_element_type(image::Image) = ccall((:txkit_image_element_type, libtxkit_core), ImageDataType, (Image,), image)
txkit_image_map_read(image::Image) = ccall((:txkit_image_map_read, libtxkit_core), MappedImageDataRead, (Image,), image)
txkit_image_map_read_data_f32(read_map::MappedImageDataRead) = ccall((:txkit_image_map_read_data_f32, libtxkit_core), Ptr{Cfloat}, (MappedImageDataRead,), read_map)
txkit_image_map_read_data_u8(read_map::MappedImageDataRead) = ccall((:txkit_image_map_read_data_u8, libtxkit_core), Ptr{UInt8}, (MappedImageDataRead,), read_map)
txkit_image_map_write(image::Image) = ccall((:txkit_image_map_write, libtxkit_core), MappedImageDataWrite, (Image,), image)
txkit_image_map_write_data_f32(write_map::MappedImageDataWrite) = ccall((:txkit_image_map_write_data_f32, libtxkit_core), Ptr{Cfloat}, (MappedImageDataWrite,), write_map)
txkit_image_map_write_data_u8(write_map::MappedImageDataWrite) = ccall((:txkit_image_map_write_data_u8, libtxkit_core), Ptr{UInt8}, (MappedImageDataWrite,), write_map)
txkit_image_new_cpu(dim::ImageDim, element_type::ImageDataType) = ccall((:txkit_image_new_cpu, libtxkit_core), Image, (ImageDim, ImageDataType), dim, element_type)
txkit_image_new_gpu_1d(dim::ImageDim, element_type::ImageDataType, context::Context) = ccall((:txkit_image_new_gpu_1d, libtxkit_core), Image, (ImageDim, ImageDataType, Context), dim, element_type, context)
txkit_image_new_gpu_2d(dim::ImageDim, element_type::ImageDataType, context::Context) = ccall((:txkit_image_new_gpu_2d, libtxkit_core), Image, (ImageDim, ImageDataType, Context), dim, element_type, context)
txkit_image_new_gpu_3d(dim::ImageDim, element_type::ImageDataType, context::Context) = ccall((:txkit_image_new_gpu_3d, libtxkit_core), Image, (ImageDim, ImageDataType, Context), dim, element_type, context)
txkit_image_sync(image::Image) = ccall((:txkit_image_sync, libtxkit_core), Int32, (Image,), image)
txkit_image_unmap_read(read_map::MappedImageDataRead) = ccall((:txkit_image_unmap_read, libtxkit_core), Cvoid, (MappedImageDataRead,), read_map)
txkit_image_unmap_write(write_map::MappedImageDataRead) = ccall((:txkit_image_unmap_write, libtxkit_core), Cvoid, (MappedImageDataWrite,), write_map)

txkit_method_compute(ctx::Context, method::Method, tgt::Image, params::Ptr{Cvoid}, params_size::UInt) = ccall((:txkit_method_compute, libtxkit_core), Int32, (Context, Method, Image, Ptr{Cvoid}, UInt), ctx, method, tgt, params, params_size)
txkit_method_destroy(method::Method) = ccall((:txkit_method_destroy, libtxkit_core), Cvoid, (Method,), method)
txkit_method_new(registry::Registry, method_name::AbstractString) = ccall((:txkit_method_new, libtxkit_core), Method, (Registry, Cstring), registry, method_name)

txkit_registry_destroy(registry::Registry) = ccall((:txkit_registry_destroy, libtxkit_core), Cvoid, (Registry,), registry)

struct GradientNoiseParams
end

struct ValueNoiseParams
end

struct WhiteNoiseParams
end

struct DebugParams
    alpha_value::Cfloat
end

txkit_registry_new_builtin() = ccall((:txkit_registry_new_builtin, libtxkit_builtin), Registry, ())

end # module

export Api

end # module

# vim: ft=julia:sw=4:ts=4:et
