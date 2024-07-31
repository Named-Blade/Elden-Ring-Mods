# My Elden Ring Mods

See Nexus for descriptions
[Static Bar Sizes](https://www.nexusmods.com/eldenring/mods/5028)
[Elden Ring Uncapped](https://www.nexusmods.com/eldenring/mods/4981)
[Infinite NG](https://www.nexusmods.com/eldenring/mods/5732)
[Damage Redone](https://www.nexusmods.com/eldenring/mods/4997)
[Rally Redone](https://www.nexusmods.com/eldenring/mods/5920)

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
Built Dlls are in Elden-Ring-Mods/build/Release