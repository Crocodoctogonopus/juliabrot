#version 450

in vec2 frag_uv;

out vec4 rgba;

layout(location = 1) uniform int iter_max;

void main() {
	// our point
	vec2 c = frag_uv;

	// iter
	int iter = 0;
    vec2 z = c;
    for(iter = 0; iter < iter_max; iter++) {
        float x = z.x * z.x - z.y * z.y + c.x;
        float y = 2 * z.x * z.y + c.y;

        if((x * x + y * y) > 4.0) 
        	break;
        z = vec2(x, y);
    }

    // output
    if (iter_max == iter) {
    	rgba = vec4(0., 0, 0, 1.);
    } else {
    	float r = (float(iter) + 5)/(float(iter_max) + 5);
		rgba = vec4(r, 0., 0., 1.);
    }
}

// 40ms, 43ms, 41ms, 36ms
// 51ms, 57ms, 50ms, 45ms