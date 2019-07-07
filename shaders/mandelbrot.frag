#version 450

in vec2 frag_uv;

out vec4 rgba;

layout(location = 1) uniform mat4 tex;

void main() {
	// our point
	dvec2 c = frag_uv;

	// iter
	int iter_max = 125;
	int iter = 0;
    dvec2 z = c;
    for(iter = 0; iter < iter_max; iter++) {
        double x = (z.x * z.x - z.y * z.y) + c.x;
        double y = (z.y * z.x + z.x * z.y) + c.y;

        if((x * x + y * y) > 4.0) 
        	break;
        z = dvec2(x, y);
    }

    // output
    if (iter_max == iter) {
    	rgba = vec4(0., 0, 0, 1.);
    } else {
    	float r = (float(iter) + 5)/(float(iter_max) + 5);
		rgba = vec4(r, 0., 0., 1.);
    }
}