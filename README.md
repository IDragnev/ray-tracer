# ray-tracer
A simple ray tracer based on the [books of Peter Shirley](https://raytracing.github.io/).
It ouputs a ppm file.

## Usage
No scene description is supported at the moment.
There are three examples scenes which are used for debugging:  
  - Cornell box (--scene=cornell-box)  
  - Scene of many randomly generates spheres (--scene=random-spheres)  
  - Scene with marble spheres and an area light source (--scene=simple-light)

Example usage:  
``
$ ray-tracer out.txt --height=600 --width=600 --samples=64 --scene=simple-light
``  
  
Run ``$ ray-tracer --help`` for more information.
