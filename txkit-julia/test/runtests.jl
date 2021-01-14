using TxKit

function run_test(registry, context, dim, image)
    new_method(registry, "debug") do method
        params = DebugParams(0.75)
        compute(context, method, image, Ref(params))

        download(image)

        map_read(image) do array
            display(array[1,:,:,1])
            println()

            display(array[1,:,:,2])
            println()

            display(array[1,:,:,3])
            println()
            @assert all(array[1,:,:,3] .== 0.0)

            display(array[1,:,:,4])
            println()
            @assert all(array[1,:,:,4] .== 0.75)
        end
    end
end

new_registry() do registry
    new_context(:gpu) do context
        dim = ImageDim(6, 3, 1, 4)

        new_image(:gpu, dim, Float32, 2, context) do image
            run_test(registry, context, dim, image)
        end
    end
end

# vim: ft=julia:ts=4:sw=4:et
