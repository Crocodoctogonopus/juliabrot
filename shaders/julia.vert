#version 430 core

layout(location = 0) in vec2 vert_xy; // world position
layout(location = 1) in vec2 vert_uv;

out vec2 frag_uv;

void main() { 	
	gl_Position = vec4(vert_xy, 0, 1);

	frag_uv = vert_uv;
}