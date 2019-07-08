#version 430

in vec2 frag_uv;

out vec4 rgba;

layout(location = 0) uniform vec2 julia_c;
layout(location = 1) uniform int iter_max;

void main() {
	// iter
	int iter = 0;
    vec2 z = frag_uv;
    for(iter = 0; iter < iter_max; iter++) {
        float x = z.x * z.x - z.y * z.y + julia_c.x;
        float y = 2 * z.x * z.y + julia_c.y;

        if((x * x + y * y) > 4.0) 
        	break;
        z = vec2(x, y);
    }

    // output
    if (iter_max == iter) {
    	rgba = vec4(0., 0., 0., 1.);
    } else {
    	float r = (float(iter) + 250)/(float(iter_max) + 250);
		rgba = vec4(r, 0., 0., 1.);
    }
}