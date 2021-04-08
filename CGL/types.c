#include "gl.h"

static const GLfloat vbdata[] = { 
    -1.0f, -1.0f,
     1.0f, -1.0f,
    -1.0f,  1.0f,
     1.0f,  1.0f
};
static const GLushort ebdata[] = { 0, 1, 2, 3 };

static struct {
  GLuint vertex_buffer, element_buffer;
  GLuint textures[2]; // 2 textures for now
} res;