# Vec-Utils

![PyPI - Version](https://img.shields.io/pypi/v/vec-utils-py)

A Rust crate for 3D vector math, quaternions, geometry, complex numbers, and angles. Built for simplicity and correctness over raw performance. Optimizations are planned. Python bindings are also available for much of the library.

- `f64` precision
- Many matrix operations are zero copy and generic

## Modules

- `vec3d`: 3D vector operations
- `quat`: Quaternion operations
- `matrix`: Matrix operations
    - `complex`: A complex valued matrix
    - `generic`: Generic 2d matrx of any size or type
    - `real`: A real valued matrix
- `geometry`: Geometry operations
    - `circle`, `plane`, `sphere`: Geometry primitives
    - `intersection`: Calculate intersections between geometry objects
- `complex`: Complex numbers
- `angle`: Angles in degrees and radians

## Crate Features

- `std`: standard library support across all active dependencies (default)
- `all-nostd`: all crate features excluding `std`
- `all`: all crate features
- `matrix`: matrix multiplicaiton and other operations, still missing a lot of features
- `rand`: Random generation of types (default)
- `nalgebra`: interop with `nalgebra` types
- `glam`: interop with `glam` types
- `rkyv`: zero-copy serialization
- `serde`: serialization/deserialization

Unless any features are enabled only `libm`, `thiserror`, and `rand` are depended on.

## License

GPL-3.0-only
See [LICENSE](./LICENSE) for details

