# Notes from the Doug Milford series on WebGL

- GL has no notion of 3D.
- GL thinks of your screen of a box with domains(-1 to 1). 
- zaxis points towards you. 

### workflow 

1. input data (xyz of a box) -> always triangles
2. buffer data to GPU (provide it fast access to the data)
3. two shaders: 
   3a. Vertex Shader -> modift
   3b. Fragment Shader
4. pixels are put on the screen



