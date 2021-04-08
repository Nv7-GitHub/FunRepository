#include "gl.h"
#include "utils/utils.h"

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

static GLuint make_texture(const char *filename)
{   
    GLuint texture;
    int width, height;

    // Load pixel data
    void *pixels = read_tga(filename, &width, &height);
    if (!pixels) {
        return 0;
    }

    // Create Texture Object
    glGenTextures(1, &texture);
    glBindTexture(GL_TEXTURE_2D, texture);

    // Setup texture object
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S,     GL_CLAMP_TO_EDGE);
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T,     GL_CLAMP_TO_EDGE);

    // Load pixel data into texture object
    glTexImage2D(
        GL_TEXTURE_2D, 0,           /* target, level of detail */
        GL_RGB8,                    /* internal format */
        width, height, 0,           /* width, height, border */
        GL_BGR, GL_UNSIGNED_BYTE,   /* external format, type */
        pixels                      /* pixels */
    );
    
    // Unload pixeldata
    free(pixels);
    return texture;
}
