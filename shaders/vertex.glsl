precision mediump float;

attribute vec3 vertPos;
attribute vec3 vertNormal;

varying vec3 fragNormal;
varying vec3 fragColor;

uniform vec3 color;
uniform mat4 mWorld;
uniform mat4 mView;
uniform mat4 mProj;

void main()                                    
{             
   fragColor = color;
   fragNormal = (mWorld * vec4(vertNormal, 0.0)).xyz;
   gl_Position = mProj * mView * mWorld * vec4(vertPos, 1.0);
}   
