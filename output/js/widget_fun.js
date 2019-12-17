let vertices = [
    -1, 1,
    -1, -1,
    1, 1,
    1, -1,
];

let vt_shader_code = `
attribute vec4 vertex_position;
void main()
{
    gl_Position = vertex_position;
}
`;

let fg_shader_code =`
precision highp float;
uniform vec2 resolution;
uniform float time;
void main()
{
    float time = time * 0.003;
    vec2 uv = (gl_FragCoord.xy - 0.5 * resolution.xy) / resolution.yy;
    float z = smoothstep(0.51, 0.5, length(uv));
    uv *= 6.;
    float i0 = 1.0;
    float i1 = 1.0;
    float i2 = 1.0;
    float i4 = 0.0;
    for (int s = 0; s < 7; s++) {
        vec2 r = vec2(cos(uv.y * i0 - i4 + time / i1), sin(uv.x * i0 - i4 + time / i1)) / i2;
        r += vec2(-r.y, r.x) * 0.3;
        uv.xy += r;

        i0 *= 1.7;
        i1 *= 2.;
        i2 *= 1.65;
        i4 += 0.05 + 0.1 * time * i1;
    }
    float r = sin(uv.x - time) * 0.3 + 0.6;
    float b = sin(uv.y + time) * 0.3 + 0.6;
    float g = cos((uv.x + uv.y + sin(time * 0.5)) * 0.5) * 0.5 + 0.5;
    gl_FragColor = vec4(r, g, b, z);
}
`;

main();

function webgl_tools() {
    // assume canvas_size not change in whole life of this page
    let canvas_size;
    let canvas;
    let slot_resolution;
    let slot_time_location;
    let slot_vertex_position;
    let time_begin;

    function canvas_init(id, canvas_width, canvas_height) {
        let canvas_raw = document.getElementById(id);
        // set the real render resolution
        canvas_raw.width = canvas_width;
        canvas_raw.height = canvas_width;

        canvas_size = [canvas_width, canvas_height];
        canvas = canvas_raw.getContext("webgl");
        if (canvas === null) {
            console.log("Create webgl context failed.");
            return;
        }
    }

    function gl_init() {
        let vertex_buffer = canvas.createBuffer();
        // extract pipeline because we need value slot
        let pipeline = pipeline_new(canvas, vt_shader_code, fg_shader_code);
        canvas.useProgram(pipeline);

        slot_vertex_position = canvas.getAttribLocation(pipeline, "vertex_position");
        slot_resolution = canvas.getUniformLocation(pipeline, "resolution");
        slot_time_location = canvas.getUniformLocation(pipeline, "time");
        //canvas.viewport(0, 0, canvas_raw.width, canvas_raw.height);
        canvas.bindBuffer(canvas.ARRAY_BUFFER, vertex_buffer);
        canvas.bufferData(canvas.ARRAY_BUFFER, new Float32Array(vertices), canvas.STATIC_DRAW);
        canvas.vertexAttribPointer(slot_vertex_position, 2, canvas.FLOAT, false, 0, 0);
        canvas.enableVertexAttribArray(slot_vertex_position);
        canvas.uniform2fv(slot_resolution, canvas_size);

        time_begin = Date.now();

        function pipeline_new(canvas, vt_shader_code, fg_shader_code) {
            let vt_shader = shader_new(canvas, vt_shader_code, canvas.VERTEX_SHADER);
            let fg_shader = shader_new(canvas, fg_shader_code, canvas.FRAGMENT_SHADER);
            let pipeline = canvas.createProgram();
            canvas.attachShader(pipeline, vt_shader);
            canvas.attachShader(pipeline, fg_shader);
            canvas.deleteShader(vt_shader);
            canvas.deleteShader(fg_shader);
            canvas.linkProgram(pipeline);
            if (!canvas.getProgramParameter(pipeline, canvas.LINK_STATUS)) {
                console.error("Shader link failed. " + canvas.getProgramInfoLog(pipeline));
                canvas.deleteProgram(pipeline);
                return null;
            }
            return pipeline;

            function shader_new(canvas, code, type) {
                let shader = canvas.createShader(type);
                canvas.shaderSource(shader, code);
                canvas.compileShader(shader);
                if (!canvas.getShaderParameter(shader, canvas.COMPILE_STATUS)) {
                    console.error("Shader compile failed. " + canvas.getShaderInfoLog(shader));
                    canvas.deleteShader(shader);
                    return null;
                }
                return shader;
            }
        }
    }

    function gl_render() {
        canvas.uniform1f(slot_time_location, (Date.now() - time_begin).toFixed(2));
        canvas.drawArrays(canvas.TRIANGLE_STRIP, 0, 4);
    }

    function gl_clear() {
        canvas.clearColor(0., 0., 0., 0.);
        canvas.clear(canvas.COLOR_BUFFER_BIT);
    }

    return [canvas_init, gl_init, gl_render, gl_clear];
}

function main() {
    let [canvas_init, gl_init, gl_render, _] = webgl_tools();
    canvas_init("widget_fun_canvas", 64, 64);
    gl_init();
    {
        let widget_fun = document.getElementById("widget_fun");
        let render = false;
        /** I spent a a lot of time and found this is the only legit way to
         *   implement this effect -_-b, simple but effective.
         *  Other ways like createInterval on hover and clearInterval on
         *   leave or more complex ways are just not working or not working
         *   properly...
         */
        widget_fun.onmouseover = () => {
            render = true;
        }
        widget_fun.onmouseleave = () => {
            render = false;
        }
        // Such non frequent polling may not use a lot of cpu
        setInterval(() => {
            if (render)
                gl_render()
            }, 16);
    }
}
