# Welcome to veloren

## What is veloren
veloren is a multiplayer voxelbased game which is heavily influenced by cubeworld.

## how to compile it
1. install dependencies
```bash
#arch:
pacman -S rust
pacman -U https://archive.archlinux.org/packages/s/sfml/sfml-2.4.2-5-x86_64.pkg.tar.xz #needed for now, because sfml is normaly 2.5 and csfml only 2.4
pacman -S csfml
```

2. download git submodules
```bash
git submodule update --init --recursive
```

2. compile and run tests
```bash
( cd worldtest && cargo run )
```

3. compile and run server
```bash
( cd server-cli && cargo run )
```

4. compile and run client
```bash
#compile shaders first. get the VulkanSDK/1.0.xxx.0/x86_64/bin/glslangValidator, e.g. from the vulkanSDK https://vulkan.lunarg.com/
glslangValidator -V frontend/src/shader/quad.frag -o frontend/src/data/frag.spv -S frag
glslangValidator -V frontend/src/shader/quad.vert -o frontend/src/data/vert.spv -S vert
# run frontend with vulkan or gl
( cd frontend && cargo run --features gl)
```
