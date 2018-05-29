#version 120

attribute vec3 coord3d;

void main() {
    gl_Position = vec4(coord3d - vec3(0.5, 0.5, 0.5), 1.0);
}
