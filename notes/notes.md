# scedule

1. convert programs into scenes, make a general Scene class 
   -> DONE , learned about the value of Box<> and the Vec<Box<dyn Struct>> paradigm
2. abstract the content within Scenes to a Renderer which can render various things
3. build a new scene, with a Point renderer. Render a bunch of 2D points.
4. build a new scene, with a ball 3D renderer. render a bunch of bouncing 3d balls


## small things 

- build console::log, singleton to deal with logging -> DONE 
- build scrolling with mouse wheel -> ...
- 

________________________________________________________________________________

# Notes from the Doug Milford series on WebGL

- GL has no notion of 3D.
- GL thinks of your screen of a box with domains(-1 to 1). 
- zaxis points towards you. 



# workflow 

1. input data (xyz of a box) -> always triangles
2. buffer data to GPU (provide it fast access to the data)
3. two shaders: 
   3a. Vertex Shader -> modift
   3b. Fragment Shader
4. pixels are put on the screen



