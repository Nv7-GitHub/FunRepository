#include "gl.h"

static int make_resources(void)
{   
    res.vertex_buffer = make_buffer(
        GL_ARRAY_BUFFER,
        vbdata,
        sizeof(vbdata)
    );
    res.element_buffer = make_buffer(
        GL_ELEMENT_ARRAY_BUFFER,
        ebdata,
        sizeof(ebdata)
    );

    res.textures[0] = make_texture("hello1.tga");
    res.textures[1] = make_texture("hello2.tga");

    return 1;
}