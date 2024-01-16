#version 100
attribute vec2 position;
attribute vec4 color;
varying vec4 out_color;
uniform mat4 proj_mat;
uniform mat4 model_mat;
void main()
{
        gl_Position = proj_mat * model_mat * vec4(position, 0, 1.0);
        gl_PointSize = 10.0;
        out_color = color;
}