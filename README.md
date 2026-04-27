# Vec-Utils / Bektor

![Crates.io Version](https://img.shields.io/crates/v/bektor?link=https%3A%2F%2Fcrates.io%2Fcrates%2Fbektor)
![PyPI - Version](https://img.shields.io/pypi/v/vec-utils-py?link=https%3A%2F%2Fpypi.org%2Fproject%2Fvec-utils-py%2F)
![docs.rs](https://img.shields.io/docsrs/bektor?link=https%3A%2F%2Fdocs.rs%2Fbektor%2F0.3.2%2Fbektor%2F)

A Rust crate for 3D vector math, quaternions, geometry, complex numbers, and angles. Python bindings are also available for much of the library.

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

## Features

- `std`: standard library support across all active dependencies (default)
- `matrix`: matrix multiplicaiton and other operations, still missing a lot of features
- `rand`: Random generation of types (default)
- `nalgebra`: interop with `nalgebra` types
- `glam`: interop with `glam` types
- `rkyv`: zero-copy serialization
- `serde`: serialization/deserialization

The only base dependencies are `libm`, `thiserror`

## License

GPL-3.0-only
See [LICENSE](./LICENSE) for details

