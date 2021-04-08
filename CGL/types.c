#include "gl.h"

static struct {
  GLuint vertex_buffer, element_buffer;
  GLuint textures[2]; // 2 textures for now
} res;