#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<const R: usize, const C: usize, T>([[T; C]; R]);

impl<const R: usize, const C: usize, T> Default for Matrix<R, C, T>
where
    T: Default,
{
    fn default() -> Self {
        let mut data: std::mem::MaybeUninit<[[T; C]; R]> = std::mem::MaybeUninit::uninit();

        for col in 0..C {
            for row in 0..R {
                unsafe { (*data.as_mut_ptr())[row][col] = T::default() };
            }
        }
        Self(unsafe { data.assume_init() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_construction() {
        let mat: Matrix<2, 3, f64> = Matrix::default();
        assert_eq!(mat, Matrix([[0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]));

        #[derive(Debug, Default, PartialEq)]
        struct NonCopy(u32, u32);
        let mat: Matrix<2, 3, NonCopy> = Matrix::default();
        assert_eq!(
            mat,
            Matrix([
                [NonCopy(0, 0), NonCopy(0, 0), NonCopy(0, 0)],
                [NonCopy(0, 0), NonCopy(0, 0), NonCopy(0, 0)]
            ])
        );
    }
}
