## Ray Tracing 

Ray tracing is a rendering technique that simulates the way light rays bounce around a scene. It can create images that are more realistic than those created using traditional rendering techniques. 

this project is based on the book Ray trancing in one weekend.


# Assumptions

camera is located at the origin 
the image plane (viewport) is at z = -1
the image plane is 2 units wide and 1 unit tall


# vector class
- vec3 class is used to represent 3D vectors and points. It has the following methods:

		- normalize: normalizes the vector
		- squared_length: returns the squared - - length of the vector
		- dot: dot product of two vectors
		- cross: cross product of two vectors
		- length: length of the vector
		
- point, color is alias to vec3

# ray class
- ray class is used to represent a ray. It has the following methods:
		point_at_parameter: returns the point at a given distance along the ray


# todo:
