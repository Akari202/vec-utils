mod vec3d;
mod quat;

mod prelude {
    pub use crate::vec3d::Vec3d;
    pub use crate::quat::Quat;
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
