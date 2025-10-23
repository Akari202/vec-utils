use std::fmt::Binary;
use std::mem;
use std::ops::{BitAnd, BitOr, BitXor, BitXorAssign, Shl, ShlAssign, Shr, ShrAssign, Sub};
// https://stackoverflow.com/questions/30330519/compile-time-generic-type-size-check
// https://doi.org/10.1063/1.1751381

trait SizeCheck<T, U, const N: usize> {
    const VALID_SIZE_NU_HOLD_T: ();
    const VALID_SIZE_T_HOLD_NU: ();
}

impl<T, U, const N: usize> SizeCheck<T, U, N> for () {
    const VALID_SIZE_NU_HOLD_T: () = assert!(
        mem::size_of::<U>() * N >= mem::size_of::<T>(),
        "N of type U are not enough to hold all the bits in type T"
    );
    const VALID_SIZE_T_HOLD_NU: () = assert!(
        mem::size_of::<U>() * N <= mem::size_of::<T>(),
        "Type T doesnt have enough bits to hold N of type U"
    );
}

/// Transposes the bits of the input into N numbers of type U, striping the bits vertically
fn transpose<T, U, const N: usize>(num: T) -> [U; N]
where
    U: From<u8> + From<bool> + Copy + Binary + Clone + BitOr<Output = U> + Shl<usize, Output = U>,
    T: From<u8>
        + Shr<usize, Output = T>
        + Clone
        + PartialEq
        + Shl<usize, Output = T>
        + BitAnd<Output = T>
{
    #[allow(clippy::let_unit_value)]
    let _: () = <() as SizeCheck<T, U, N>>::VALID_SIZE_NU_HOLD_T;

    // TODO: figure out a good way to do this
    let bit_depth = mem::size_of::<U>() * 8;
    let bit_size = mem::size_of::<T>() * 8;
    (0..N)
        .rev()
        .map(|j| {
            (0..bit_depth).fold(U::from(0u8), |acc, i| {
                let bit_index = i * N + j;
                if bit_index >= bit_size {
                    return acc;
                }
                acc | U::from(num.clone() & T::from(1u8) << bit_index != T::from(0u8)) << i
            })
        })
        .collect::<Vec<U>>()
        .try_into()
        .unwrap_or_else(|_| panic!())
}

/// Untransposes the bits from N numbers of type U,
/// with the bits striped vertically, into one number
fn untranspose<T, U, const N: usize>(nums: [U; N]) -> T
where
    U: From<u8>
        + Copy
        + Clone
        + BitOr<Output = U>
        + PartialEq
        + Shl<usize, Output = U>
        + BitAnd<Output = U>,
    T: From<u8> + Shr<usize> + Clone + Shl<usize, Output = T> + BitOr<Output = T>
{
    #[allow(clippy::let_unit_value)]
    let _: () = <() as SizeCheck<T, U, N>>::VALID_SIZE_T_HOLD_NU;
    // TODO: figure out a good way to do this
    let bit_depth = mem::size_of::<U>() * 8;
    let bit_size = mem::size_of::<T>() * 8;
    (0..bit_depth)
        .flat_map(|i| (0..N).map(move |j| (i, j)))
        .fold(T::from(0u8), |acc, (i, j)| {
            let bit_index = N * i + j;
            if bit_index >= bit_size {
                return acc;
            }
            acc | T::from(u8::from(
                nums[N - j - 1] & U::from(1u8) << i != U::from(0u8)
            )) << bit_index
        })
}

fn transpose_to_axes<U, const N: usize>(transposed: [U; N]) -> [U; N]
where
    U: BitXor<Output = U>
        + From<u8>
        + Copy
        + Clone
        + Sub<Output = U>
        + PartialEq
        + Shl<usize, Output = U>
        + BitAnd<Output = U>
        + BitXorAssign
        + ShlAssign<usize>
        + Shr<usize, Output = U>
{
    let mut axes = transposed;
    let n = U::from(2 << (N - 1));
    let mut t = axes[N - 1usize] >> 1usize;
    for i in (1..N).rev() {
        axes[i] ^= axes[i - 1];
    }
    axes[0] ^= t;
    let mut q = U::from(2);
    while q != n {
        let p = q - U::from(1u8);
        for i in (0..(N - 1)).rev() {
            if axes[i] & q == U::from(0u8) {
                t = (axes[0] ^ axes[i]) & p;
                axes[0] ^= t;
                axes[i] ^= t;
            } else {
                axes[0] ^= p;
            }
        }
        q <<= 1usize;
    }
    axes
}

fn axes_to_transpose<U, const N: usize>(axes: [U; N]) -> [U; N]
where
    U: BitXor<Output = U>
        + From<u8>
        + Copy
        + Clone
        + Sub<Output = U>
        + PartialEq
        + PartialOrd
        + Shl<usize, Output = U>
        + BitAnd<Output = U>
        + BitXorAssign
        + ShlAssign<usize>
        + ShrAssign<usize>
        + Shr<usize, Output = U>
{
    let mut transposed = axes;
    let m = U::from(1 << (N - 1));
    let mut q = m;
    while q > U::from(1u8) {
        let p = q - U::from(1u8);
        for i in 0..N {
            if transposed[i] & q == U::from(0u8) {
                let t = (transposed[0] ^ transposed[i]) & p;
                transposed[0] ^= t;
                transposed[i] ^= t;
            } else {
                transposed[0] ^= p;
            }
        }
        q >>= 1usize;
    }

    for i in 1..N {
        transposed[i] ^= transposed[i - 1];
    }
    let mut t = U::from(0u8);

    while q > U::from(1u8) {
        if transposed[N - 1] & q != U::from(0u8) {
            t = q - U::from(1u8);
        }
        q >>= 1usize;
    }

    for i in 0..N {
        transposed[i] ^= t;
    }

    axes
}

/// Converts an index to coordinates on a hilbert curve in N dimensions
pub fn hilbert_index_to_axes<T, U, const N: usize>(index: T) -> [U; N]
where
    U: From<u8>
        + From<bool>
        + Copy
        + Binary
        + Clone
        + BitXor<Output = U>
        + Sub<Output = U>
        + PartialEq
        + PartialOrd
        + Shl<usize, Output = U>
        + BitAnd<Output = U>
        + BitXorAssign
        + ShlAssign<usize>
        + Shr<usize, Output = U>
        + BitAnd
        + BitOr<Output = U>
        + Shl<usize, Output = U>,
    T: From<u8>
        + Shr<usize, Output = T>
        + Clone
        + PartialEq
        + Shl<usize, Output = T>
        + BitAnd<Output = T>
        + BitOr<Output = T>
{
    transpose_to_axes(transpose(index))
}

/// Converts coordinates on a hilbert curve in N dimensions to an index
pub fn axes_to_hilbert_index<T, U, const N: usize>(axes: [U; N]) -> T
where
    U: From<u8>
        + From<bool>
        + Copy
        + Binary
        + Clone
        + BitXor<Output = U>
        + Sub<Output = U>
        + PartialEq
        + PartialOrd
        + Shl<usize, Output = U>
        + BitAnd<Output = U>
        + BitXorAssign
        + ShlAssign<usize>
        + ShrAssign<usize>
        + Shr<usize, Output = U>
        + BitAnd
        + BitOr<Output = U>
        + Shl<usize, Output = U>,
    T: From<u8>
        + Shr<usize, Output = T>
        + Clone
        + PartialEq
        + Shl<usize, Output = T>
        + BitAnd<Output = T>
        + BitOr<Output = T>
{
    untranspose(axes_to_transpose(axes))
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use rand::distr::{Distribution, Uniform};

    use super::*;

    #[test]
    fn test_transpose() {
        let num: u16 = 25676;
        let transposed: [u8; 3] = transpose(num);
        assert_eq!(transposed[0], 17);
        assert_eq!(transposed[1], 24);
        assert_eq!(transposed[2], 6);

        let range = Uniform::try_from(u16::MIN..u16::MAX).unwrap();
        let mut rng = rand::rng();
        for _ in 0..1000 {
            let num = range.sample(&mut rng);

            let transposed_two: [u8; 2] = transpose(num);
            let untransposed_two: u16 = untranspose(transposed_two);
            assert_eq!(
                format!("{:#018b}", num),
                format!("{:#018b}", untransposed_two)
            );

            let transposed_three: [u8; 3] = transpose(num);
            let untransposed_three: u32 = untranspose(transposed_three);
            assert_eq!(u32::from(num), untransposed_three);

            let transposed_four: [u8; 4] = transpose(num);
            let untransposed_four: u32 = untranspose(transposed_four);
            assert_eq!(u32::from(num), untransposed_four);

            let transposed_five: [u8; 5] = transpose(num);
            let untransposed_five: u64 = untranspose(transposed_five);
            assert_eq!(u64::from(num), untransposed_five);
        }
    }

    #[test]
    fn test_hilbert() {
        let range = Uniform::try_from(u32::MIN..u32::MAX).unwrap();
        let mut rng = rand::rng();
        for _ in 0..100 {
            let index = range.sample(&mut rng);
            let axes: [u8; 4] = hilbert_index_to_axes(index);
            let calc_index: u32 = axes_to_hilbert_index(axes);
            assert_eq!(index, calc_index);
        }
    }
}
