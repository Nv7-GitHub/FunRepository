#include <GL/glew.h>
#include <stdlib.h>
#include <stdio.h>

void *file_contents(const char *filename, GLint *length);
static short le_short(unsigned char *bytes);
void *read_tga(const char *filename, int *width, int *height);