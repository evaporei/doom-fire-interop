#ifndef doom_h
#define doom_h

#include <stdint.h>

typedef struct PixelBoard PixelBoard;

PixelBoard* create_board(size_t width, size_t height);

void create_fire_source(PixelBoard* board);

void calculate_fire_propagation(PixelBoard* board, void (* render)(unsigned char* pixels, size_t size));

void free_board(PixelBoard* board);

unsigned char* get_pixels_ptr(const PixelBoard* board);

size_t get_pixels_len(const PixelBoard* board);

#endif
