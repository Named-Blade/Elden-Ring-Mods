# My Elden Ring Mods

To Build The Mods:

**_Requirements:_** git, CMake, Visual Studio with any toolchain
```
git clone --recurse-submodules https://github.com/Named-Blade/Elden-Ring-Mods.git
cd Elden-Ring-Mods
mkdir build
cd build
cmake ..
cmake --build . --config Release
```
Build Dlls are in Elden-Ring-Mods/build/Release