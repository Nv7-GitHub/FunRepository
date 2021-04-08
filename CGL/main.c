// https://duriansoftware.com/joe/an-intro-to-modern-opengl.-table-of-contents

#include <stdio.h>
#include <stdlib.h>
#include "gl.h"
#include "main.h"

static void update_fade_factor(void)
{

}

int main(int argc, char** argv) {
  glutInit(&argc, argv);
  glutInitDisplayMode(GLUT_RGB | GLUT_DOUBLE);
  glutInitWindowSize(400, 300);
  glutCreateWindow("Hello World");
  glutDisplayFunc(&render);
  glutIdleFunc(&update_fade_factor);
  glewInit();
  if (!GLEW_VERSION_2_0) {
      fprintf(stderr, "OpenGL 2.0 not available\n");
      return 1;
  }
  if (!make_resources()) {
      fprintf(stderr, "Failed to load resources\n");
      return 1;
  }

  glutMainLoop();
  return 0;
}