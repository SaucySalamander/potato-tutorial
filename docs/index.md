## Welcome to Documentation on the Potato Engine

This is a project to learn the Vulkan API as well as Rust. This docment is ment to layout and describe the different parts of the engine.

### Vulkan API Objects

The potato engine interfaces with the Vulkan API via the rust crate [ash](https://github.com/MaikKlein/ash). It uses the [winit](https://github.com/rust-windowing/winit) rust crate for windowing.

#### Winit
##### Event Loop
#### Window

#### Vulkan
##### Entry

The entry is a structure of the ash crate. It is used to load the vulkan library. You instantiate the entry and use it to create the instance.

#### [Instance](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#initialization-instances)

The instance contains the application state. You can use the instance to pass the implmentaion information about the application.

#### [Validation Layers](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#fundamentals-errors)

Vulkan does not ship by deafult with debugging capabilities. You need to enable validation layers to aid in development.

>One of the core principles of Vulkan is that building and submitting command buffers should be highly efficient. Thus error checking and validation of state in the core layer is minimal, although more rigorous validation can be enabled through the use of layers.

#### Surface
