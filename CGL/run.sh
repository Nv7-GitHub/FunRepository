clang main.c utils/utils.c -I/opt/local/include -framework OpenGL -framework GLUT -lGLEW -Wno-deprecated-declarations
./a.out
du -h a.out
rm a.out