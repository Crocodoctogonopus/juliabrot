#version 450 core

layout(location = 0) in vec2 vert_xy; // world position
layout(location = 1) in vec2 vert_uv;

out vec2 frag_uv;

layout(location = 0) uniform mat4 trans;

void main() { 	
	gl_Position = vec4(vert_xy, 0, 1);

	frag_uv = (trans * vec4(vert_uv, 0, 1)).xy;
}