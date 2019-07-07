#version 430

in vec2 frag_uv;

out vec4 rgba;

layout(location = 1) uniform vec2 julia_c;

void main() {
	// our point
	dvec2 c = dvec2(0., 0.);

	// iter
	int iter_max = 75;
	int iter = 0;
    dvec2 z = frag_uv;
    for(iter = 0; iter < iter_max; iter++) {
        double x = (z.x * z.x - z.y * z.y) + julia_c.x;
        double y = (z.y * z.x + z.x * z.y) + julia_c.y;

        if((x * x + y * y) > 4.0) 
        	break;
        z = dvec2(x, y);
    }

    // output
    if (iter_max == iter) {
    	rgba = vec4(0., 0., 0., 1.);
    } else {
    	float rr = (float(iter) + 5)/(float(iter_max) + 5);
		rgba = vec4(rr, 0., 0., 1.);
    }
}