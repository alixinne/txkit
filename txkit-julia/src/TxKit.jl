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
import ..libctxkit

const ImageDataType = UInt32
const ImageDataType_UInt8 = ImageDataType(0)
const ImageDataType_Float32 = ImageDataType(1)

const Context = Ptr{Cvoid}

const Image = Ptr{Cvoid}

const MappedImageDataRead = Ptr{Cvoid}
const MappedImageDataWrite = Ptr{Cvoid}

const TextureMethod = Ptr{Cvoid}

const Registry = Ptr{Cvoid}

struct ImageDim
    width::UInt
    height::UInt
    depth::UInt
    channels::UInt
end

txkit_context_destroy(ctx::Context) = ccall((:txkit_context_destroy, libctxkit), Cvoid, (Context,), ctx)
txkit_context_new_cpu() = ccall((:txkit_context_new_cpu, libctxkit), Context, ())
txkit_context_new_gpu() = ccall((:txkit_context_new_gpu, libctxkit), Context, ())

txkit_get_last_error() = ccall((:txkit_get_last_error, libctxkit), Ptr{Cchar}, ())

txkit_image_destroy(image::Image) = ccall((:txkit_image_destroy, libctxkit), Cvoid, (Image,), image)
txkit_image_dim(image::Image) = ccall((:txkit_image_dim, libctxkit), ImageDim, (Image,), image)
txkit_image_element_type(image::Image) = ccall((:txkit_image_element_type, libctxkit), ImageDataType, (Image,), image)
txkit_image_map_read(image::Image) = ccall((:txkit_image_map_read, libctxkit), MappedImageDataRead, (Image,), image)
txkit_image_map_read_data_f32(read_map::MappedImageDataRead) = ccall((:txkit_image_map_read_data_f32, libctxkit), Ptr{Cfloat}, (MappedImageDataRead,), read_map)
txkit_image_map_read_data_u8(read_map::MappedImageDataRead) = ccall((:txkit_image_map_read_data_u8, libctxkit), Ptr{UInt8}, (MappedImageDataRead,), read_map)
txkit_image_map_write(image::Image) = ccall((:txkit_image_map_write, libctxkit), MappedImageDataWrite, (Image,), image)
txkit_image_map_write_data_f32(write_map::MappedImageDataWrite) = ccall((:txkit_image_map_write_data_f32, libctxkit), Ptr{Cfloat}, (MappedImageDataWrite,), write_map)
txkit_image_map_write_data_u8(write_map::MappedImageDataWrite) = ccall((:txkit_image_map_write_data_u8, libctxkit), Ptr{UInt8}, (MappedImageDataWrite,), write_map)
txkit_image_new_cpu(dim::ImageDim, element_type::ImageDataType) = ccall((:txkit_image_new_cpu, libctxkit), Image, (ImageDim, ImageDataType), dim, element_type)
txkit_image_new_gpu_1d(dim::ImageDim, element_type::ImageDataType, context::Context) = ccall((:txkit_image_new_gpu_1d, libctxkit), Image, (ImageDim, ImageDataType, Context), dim, element_type, context)
txkit_image_new_gpu_2d(dim::ImageDim, element_type::ImageDataType, context::Context) = ccall((:txkit_image_new_gpu_2d, libctxkit), Image, (ImageDim, ImageDataType, Context), dim, element_type, context)
txkit_image_new_gpu_3d(dim::ImageDim, element_type::ImageDataType, context::Context) = ccall((:txkit_image_new_gpu_3d, libctxkit), Image, (ImageDim, ImageDataType, Context), dim, element_type, context)
txkit_image_sync(image::Image) = ccall((:txkit_image_sync, libctxkit), Int32, (Image,), image)
txkit_image_unmap_read(read_map::MappedImageDataRead) = ccall((:txkit_image_unmap_read, libctxkit), Cvoid, (MappedImageDataRead,), read_map)
txkit_image_unmap_write(write_map::MappedImageDataRead) = ccall((:txkit_image_unmap_write, libctxkit), Cvoid, (MappedImageDataWrite,), write_map)

txkit_method_compute(ctx::Context, method::TextureMethod, tgt::Image, params::Ptr{Cvoid}, params_size::UInt) = ccall((:txkit_method_compute, libctxkit), Int32, (Context, TextureMethod, Image, Ptr{Cvoid}, UInt), ctx, method, tgt, params, params_size)
txkit_method_destroy(method::TextureMethod) = ccall((:txkit_method_destroy, libctxkit), Cvoid, (TextureMethod,), method)
txkit_method_new(registry::Registry, method_name::AbstractString) = ccall((:txkit_method_new, libctxkit), TextureMethod, (Registry, Cstring), registry, method_name)

txkit_registry_destroy(registry::Registry) = ccall((:txkit_registry_destroy, libctxkit), Cvoid, (Registry,), registry)

const StatsMode = Int32

const StatsMode_Normal = StatsMode(0)
const StatsMode_Process = StatsMode(1)
const StatsMode_LookAt = StatsMode(2)

struct Vector2_f32
    x::Float32
    y::Float32
end

struct GradientNoiseParams
    global_seed::UInt32
    scale::Float32
    stats_mode::StatsMode
    stats_look_at::Vector2_f32
end

const PhasorNoiseProfile = Int32

const PhasorNoiseProfile_Complex = PhasorNoiseProfile(0)
const PhasorNoiseProfile_Real = PhasorNoiseProfile(1)
const PhasorNoiseProfile_Imag = PhasorNoiseProfile(2)
const PhasorNoiseProfile_Sin = PhasorNoiseProfile(3)
const PhasorNoiseProfile_Saw = PhasorNoiseProfile(4)

const PhasorNoiseWeights = Int32

const PhasorNoiseWeights_None = PhasorNoiseWeights(0)
const PhasorNoiseWeights_Bernoulli = PhasorNoiseWeights(1)
const PhasorNoiseWeights_Uniform = PhasorNoiseWeights(2)

const PhasorNoisePointDistribution = Int32

const PhasorNoisePointDistribution_StratPoisson = PhasorNoisePointDistribution(0)
const PhasorNoisePointDistribution_Poisson = PhasorNoisePointDistribution(1)

struct PhasorNoiseParams
    global_seed::UInt32
    scale::Float32
    stats_mode::StatsMode
    stats_look_at::Vector2_f32

    noise_lookahead::Int32
    kernel_count::Int32
    noise_profile::PhasorNoiseProfile
    noise_weights::PhasorNoiseWeights
    noise_point_distribution::PhasorNoisePointDistribution

    noise_frequency::Float32
    noise_angle::Float32
end

struct SimplexNoiseParams
    global_seed::UInt32
    scale::Float32
    stats_mode::StatsMode
    stats_look_at::Vector2_f32
end

struct ValueNoiseParams
    global_seed::UInt32
    scale::Float32
    stats_mode::StatsMode
    stats_look_at::Vector2_f32
end

struct WhiteNoiseParams
    global_seed::UInt32
end

struct DebugParams
    alpha_value::Cfloat
end

txkit_registry_new_builtin() = ccall((:txkit_registry_new_builtin, libctxkit), Registry, ())

end # module

import .Api.Vector2_f32, .Api.StatsMode, .Api.StatsMode_Normal,
       .Api.StatsMode_Process, .Api.StatsMode_LookAt, .Api.GradientNoiseParams,
       .Api.PhasorNoiseProfile, .Api.PhasorNoiseProfile_Complex,
       .Api.PhasorNoiseProfile_Real, .Api.PhasorNoiseProfile_Imag,
       .Api.PhasorNoiseProfile_Sin, .Api.PhasorNoiseProfile_Saw,
       .Api.PhasorNoiseWeights, .Api.PhasorNoiseWeights_None,
       .Api.PhasorNoiseWeights_Bernoulli, .Api.PhasorNoiseWeights_Uniform,
       .Api.PhasorNoisePointDistribution,
       .Api.PhasorNoisePointDistribution_StratPoisson,
       .Api.PhasorNoisePointDistribution_Poisson, .Api.PhasorNoiseParams,
       .Api.SimplexNoiseParams, .Api.ValueNoiseParams, .Api.WhiteNoiseParams,
       .Api.DebugParams

export Vector2_f32, StatsMode_Normal, StatsMode_Process, StatsMode_LookAt, GradientNoiseParams, PhasorNoiseParams,
       PhasorNoiseProfile, PhasorNoiseProfile_Complex, PhasorNoiseProfile_Real, PhasorNoiseProfile_Imag,
       PhasorNoiseProfile_Sin, PhasorNoiseProfile_Saw, PhasorNoiseWeights, PhasorNoiseWeights_None,
       PhasorNoiseWeights_Bernoulli, PhasorNoiseWeights_Uniform, PhasorNoisePointDistribution,
       PhasorNoisePointDistribution_StratPoisson, PhasorNoisePointDistribution_Poisson, SimplexNoiseParams,
       ValueNoiseParams, WhiteNoiseParams, DebugParams, StatsMode

struct Context
    context::Api.Context
end

function new_context(type::Symbol)
    ptr = if type == :cpu
        Api.txkit_context_new_cpu()
    elseif type == :gpu
        Api.txkit_context_new_gpu()
    else
        error("unknown context type: " * string(type))
    end

    if ptr == C_NULL
        error("error creating context: " * unsafe_string(Api.txkit_get_last_error()))
    end

    Context(ptr)
end

function new_context(f::Function, type::Symbol)
    ctx = new_context(type)

    try
        f(ctx)
    finally
        destroy(ctx)
    end
end

function destroy(context::Context)
    Api.txkit_context_destroy(context.context)
end

import .Api.ImageDim, .Api.ImageDataType

struct Image{E}
    image::Api.Image
end

function new_image(type::Symbol, dim::ImageDim, etype::Union{Type{UInt8}, Type{Float32}}, dims::Integer, context::Context)
    element_type = if etype == UInt8
        Api.ImageDataType_UInt8
    elseif etype == Float32
        Api.ImageDataType_Float32
    else
        error("unknown element type: " * string(etype))
    end

    ptr = if type == :cpu
        Api.txkit_image_new_cpu(dim, element_type)
    elseif type == :gpu
        if dims == 1
            Api.txkit_image_new_gpu_1d(dim, element_type, context.context)
        elseif dims == 2
            Api.txkit_image_new_gpu_2d(dim, element_type, context.context)
        elseif dims == 3
            Api.txkit_image_new_gpu_3d(dim, element_type, context.context)
        else
            error("unknown number of dims for GPU image: " * string(dims))
        end
    else
        error("unknown type of image: " * string(type))
    end

    if ptr == C_NULL
        error("error creating image: " * unsafe_string(Api.txkit_get_last_error()))
    end

    Image{etype}(ptr)
end

function new_image(f::Function, type::Symbol, dim::ImageDim, etype::Union{Type{UInt8}, Type{Float32}}, dims::Integer, context::Context)
    img = new_image(type, dim, etype, dims, context)

    try
        f(img)
    finally
        destroy(img)
    end
end

function destroy(image::Image)
    Api.txkit_image_destroy(image.image)
end

function sync(image::Image)
    if Api.txkit_image_sync(image.image) != 0
        error("error syncing image: " * unsafe_string(Api.txkit_get_last_error()))
    end

    nothing
end

function map_read(f::Function, image::Image{E}) where {E}
    map = Api.txkit_image_map_read(image.image)

    if map == C_NULL
        error("error mapping image for read: " * unsafe_string(Api.txkit_get_last_error()))
    end

    try
        map_read_data = if E == UInt8
            Api.txkit_image_map_read_data_u8(map)
        elseif E == Float32
            Api.txkit_image_map_read_data_f32(map)
        end

        if map_read_data == C_NULL
            error("error obtaining pointer to data for map: " * unsafe_string(Api.txkit_get_last_error()))
        end

        # Wrap the array returned by txkit
        dim = Api.txkit_image_dim(image.image)
        array = unsafe_wrap(Array{E}, map_read_data, (dim.channels, dim.width, dim.height, dim.depth))
        array = permutedims(array, (4, 3, 2, 1))

        # Call the user function
        f(array)
    finally
        Api.txkit_image_unmap_read(map)
    end
end

function map_write(f::Function, image::Image{E}) where {E}
    map = Api.txkit_image_map_write(image.image)

    if map == C_NULL
        error("error mapping image for write: " * unsafe_string(Api.txkit_get_last_error()))
    end

    try
        map_write_data = if E == UInt8
            Api.txkit_image_map_write_data_u8(map)
        elseif E == Float32
            Api.txkit_image_map_write_data_f32(map)
        end

        if map_write_data == C_NULL
            error("error obtaining pointer to data for map: " * unsafe_string(Api.txkit_get_last_error()))
        end

        # Wrap the array returned by txkit
        dim = Api.txkit_image_dim(image.image)
        array = unsafe_wrap(Array{E}, map_write_data, (dim.channels, dim.width, dim.height, dim.depth))
        array = permutedims(array, (4, 3, 2, 1))

        # Call the user function
        f(array)
    finally
        Api.txkit_image_unmap_write(map)
    end
end

struct Registry
    registry::Api.Registry
end

function new_registry()
    ptr = Api.txkit_registry_new_builtin()

    if ptr == C_NULL
        error("error creating registry: " * unsafe_string(Api.txkit_get_last_error()))
    end

    Registry(ptr)
end

function new_registry(f::Function)
    registry = new_registry()

    try
        f(registry)
    finally
        destroy(registry)
    end
end

function destroy(registry::Registry)
    Api.txkit_registry_destroy(registry.registry)
end

struct TextureMethod
    method::Api.TextureMethod
end

function new_method(registry::Registry, name::AbstractString)
    ptr = Api.txkit_method_new(registry.registry, name)

    if ptr == C_NULL
        error("error creating method: " * unsafe_string(Api.txkit_get_last_error()))
    end

    TextureMethod(ptr)
end

function new_method(f::Function, registry::Registry, name::AbstractString)
    mth = new_method(registry, name)

    try
        f(mth)
    finally
        destroy(mth)
    end
end

function destroy(method::TextureMethod)
    Api.txkit_image_destroy(method.method)
end

function compute(context::Context, method::TextureMethod, target::Image, params::Union{Nothing, Any}) where {P}
    result = if params == nothing
        Api.txkit_method_compute(context.context, method.method, target.image, C_NULL, 0)
    else
        Api.txkit_method_compute(context.context, method.method, target.image, pointer_from_objref(params), UInt64(sizeof(params[])))
    end

    if result != 0
        error("error computing result: " * unsafe_string(Api.txkit_get_last_error()))
    end

    nothing
end

export Api, Context, new_context, ImageDim, Image, new_image, destroy, sync, map_read, map_write, TextureMethod, new_method, compute, Registry, new_registry

end # module

# vim: ft=julia:sw=4:ts=4:et
