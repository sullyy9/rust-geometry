#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<const R: usize, const C: usize, const S: usize, T>([T; S]);

impl<const R: usize, const C: usize, const S: usize, T> Default for Matrix<R, C, S, T>
where
    T: Default,
{
    fn default() -> Self {
        let mut data: std::mem::MaybeUninit<[T; S]> = std::mem::MaybeUninit::uninit();

        for i in 0..S {
            unsafe { (*data.as_mut_ptr())[i] = T::default() };
        }
        Self(unsafe { data.assume_init() })
    }
}

impl<const R: usize, const C: usize, const S: usize, T> Matrix<R, C, S, T> {
    fn all<U>(val: U) -> Self
    where
        U: Into<T>,
        T: Copy,
    {
        Self([val.into(); S])
    }
}

impl<const R: usize, const C: usize, const S: usize, T, U> From<[[U; C]; R]> for Matrix<R, C, S, T>
where
    U: Copy + Into<T>,
{
    fn from(array: [[U; C]; R]) -> Self {
        let mut data: std::mem::MaybeUninit<[T; S]> = std::mem::MaybeUninit::uninit();

        for row in 0..R {
            for col in 0..C {
                unsafe { (*data.as_mut_ptr())[row * col] = array[row][col].into() };
            }
        }
        Self(unsafe { data.assume_init() })
    }
}

// impl<const R: usize, const C: usize, T, U> From<Matrix<R, C, U>> for Matrix<R, C, T> {
//     fn from(array: [[T; C]; R]) -> Self {
//         Self(array)
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_construction() {
        let mat: Matrix<2, 3, { 2 * 3 }, f64> = Matrix::default();
        assert_eq!(mat, Matrix([0.0; 6]));

        #[derive(Debug, Default, PartialEq)]
        struct NonCopy(u32, u32);
        let mat: Matrix<2, 3, { 2 * 3 }, NonCopy> = Matrix::default();
        assert_eq!(
            mat,
            Matrix([
                NonCopy(0, 0),
                NonCopy(0, 0),
                NonCopy(0, 0),
                NonCopy(0, 0),
                NonCopy(0, 0),
                NonCopy(0, 0)
            ])
        );
    }

    #[test]
    fn test_all_construction() {
        let mat: Matrix<2, 3, {2 * 3}, f64> = Matrix::all(5);
        assert_eq!(mat, Matrix([5.0, 5.0, 5.0, 5.0, 5.0, 5.0]));
    }

    #[test]
    fn test_array_conversion() {
        let mat: Matrix<2, 3, {2 * 3}, f64> = Matrix::from([[5, 5, 5], [5, 5, 5]]);
        assert_eq!(mat, Matrix([5.0, 5.0, 5.0, 5.0, 5.0, 5.0]));
    }
}
