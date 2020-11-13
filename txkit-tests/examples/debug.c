#include <stdio.h>
#include "txkit.h"

// Compile with: gcc -I../include -Wall -Werror -L../target/debug -Wl,-rpath=../target/debug -ltxkit -o debug debug.c

#define TXKIT_CHECKPTR(ptr)                                             \
	do {                                                            \
		if (!(ptr)) {                                           \
			fprintf(stderr, "\e[31mtxkit error: %s\e[0m\n", \
				txkit_get_last_error());                \
		}                                                       \
	} while (0)

int main(int argc, char *argv[]) {
	TxKit_Context *ctx = txkit_context_new_gpu();
	TXKIT_CHECKPTR(ctx);

	TxKit_Context *ctx_cpu = txkit_context_new_cpu();
	TXKIT_CHECKPTR(ctx_cpu);

	TxKit_ImageDim dim = {.width = 16, .height = 16, .depth = 1, .channels = 4};

	TxKit_Image *img = txkit_image_new_gpu_2d(dim, TxKit_ImageDataType_Float32, ctx);
	TXKIT_CHECKPTR(img);

	TxKit_Image *img_cpu = txkit_image_new_cpu(dim, TxKit_ImageDataType_Float32);
	TXKIT_CHECKPTR(img_cpu);

	TxKit_Registry *reg = txkit_registry_new_builtin();
	TXKIT_CHECKPTR(reg);

	TxKit_Method *mth = txkit_method_new(reg, "debug");
	TXKIT_CHECKPTR(mth);

	TxKit_DebugParams params = {.alpha_value = 0.5f};

	txkit_method_compute(ctx, mth, img, &params, sizeof(params));
	txkit_method_compute(ctx_cpu, mth, img_cpu, &params, sizeof(params));

	txkit_image_sync(img);
	txkit_image_sync(img_cpu);

	TxKit_MappedImageDataRead *read_map = txkit_image_map_read(img);
	TXKIT_CHECKPTR(read_map);

	TxKit_MappedImageDataRead *read_map_cpu = txkit_image_map_read(img_cpu);
	TXKIT_CHECKPTR(read_map_cpu);

	const float *data = txkit_image_map_read_data_f32(read_map);
	TXKIT_CHECKPTR(data);

	const float *data_cpu = txkit_image_map_read_data_f32(read_map_cpu);
	TXKIT_CHECKPTR(data_cpu);

	for (int j = 0; j < dim.height; ++j) {
		for (int i = 0; i < dim.width; ++i) {
			for (int k = 0; k < dim.channels; ++k) {
				uintptr_t idx = k + i * dim.channels +
						j * dim.width * dim.channels;

				printf("%g ", data[idx]);

				if (data[idx] != data_cpu[idx]) {
					fprintf(stderr,
						"\n\e[31mIconsistency at (%d, "
						"%d, %d): %g != %g\e[0m\n",
						i, j, k, data[idx],
						data_cpu[idx]);
				}
			}
			printf(", ");
		}
		printf("\n");
	}

	txkit_image_unmap_read(read_map_cpu);
	txkit_image_unmap_read(read_map);

	txkit_method_destroy(mth);
	txkit_registry_destroy(reg);
	txkit_image_destroy(img_cpu);
	txkit_image_destroy(img);
	txkit_context_destroy(ctx_cpu);
	txkit_context_destroy(ctx);
}
