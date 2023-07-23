#version 330 core
layout (location = 0) in vec3 in_pos;

// uniform mat4 projection;

// const mat4 projection = mat4(
//     1, 0, 0, 0,
//     0, -1, 0, 0,
//     0, 0, 1, 0,
//     0, 0, 0, 1
// );

void main() {
    // gl_Position = projection * vec4(in_pos, 1.0);
    gl_Position = vec4(in_pos, 1.0);
}