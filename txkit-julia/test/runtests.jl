using TxKit

function run_test(context, dim, image)
	registry = Api.txkit_registry_new_builtin()
	@assert registry != C_NULL
	method = Api.txkit_method_new(registry, "debug")
	@assert method != C_NULL

	result = Api.txkit_method_compute(context, method, image, C_NULL, UInt(0))
	@assert result == 0

	Api.txkit_image_sync(image)

	map_read = Api.txkit_image_map_read(image)
	@assert map_read != C_NULL

	map_read_data = Api.txkit_image_map_read_data_f32(map_read)
	@assert map_read_data != C_NULL

	# Wrap the array returned by txkit
	array = unsafe_wrap(Array, map_read_data, (dim.channels, dim.width, dim.height, dim.depth))
	# Permute its dimensions to match the column major
	# ordering of Julia
	array = permutedims(array, (4, 3, 2, 1))

	display(array[1,:,:,1])
	println()

	display(array[1,:,:,2])
	println()

	display(array[1,:,:,3])
	println()
	@assert all(array[1,:,:,3] .== 0.0)

	display(array[1,:,:,4])
	println()
	@assert all(array[1,:,:,4] .== 1.0)

	Api.txkit_image_unmap_read(map_read)
	Api.txkit_method_destroy(method)
	Api.txkit_registry_destroy(registry)
end

context = Api.txkit_context_new_gpu()
@assert context != C_NULL

dim = Api.ImageDim(6, 3, 1, 4)
image = Api.txkit_image_new_gpu_2d(dim, Api.ImageDataType_Float32, context)
@assert image != C_NULL

run_test(context, dim, image)

Api.txkit_image_destroy(image)
Api.txkit_context_destroy(context)
