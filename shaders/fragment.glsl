precision mediump float;       

//testing changes with git plz ignore lol

struct DirectionalLight
{
    vec3 position;
    vec3 color;
};

varying vec3 fragColor;
varying vec3 fragNormal;

uniform vec3 ambientLightIntensity;
uniform DirectionalLight sun;
                                                       
void main()                                    
{                  
//   gl_FragColor = vec4(normalize(fragNormal), 1.0);
   vec3 surfaceNormal = normalize(fragNormal);

   vec3 lightIntensity = ambientLightIntensity + (sun.color * max(dot(surfaceNormal, normalize(sun.position)), 0.0));
    
//   if (sun.position.z == 0.0)
//      gl_FragColor = vec4(0.0, 1.0, 0.0, 1.0);
//   else 
      gl_FragColor = vec4(fragColor * lightIntensity, 1.0);
   
}   