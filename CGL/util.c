#include "gl.h"

static GLuint make_buffer(
    GLenum target,
    const void *buffer_data,
    GLsizei buffer_size
) {
    GLuint buffer;
    glGenBuffers(1, &buffer);
    glBindBuffer(target, buffer);
    glBufferData(target, buffer_size, buffer_data, GL_STATIC_DRAW); // GL_STATIC_DRAW means it will most likely never change, GL_DYNAMIC_DRAW means it will
    return buffer;
}