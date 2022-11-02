# Java Native Interface Usage Sample

1. Write your code with static and System.loadLibrary, Other sources.
2. javac -h <Output_Directory> <Source_File> to generate header
   - use javah Command if you have Old java version and javah exists.
3. Write your library code with header.
4. Compile library with fPIC, shared option.
