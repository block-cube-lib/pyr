use num::{traits::NumAssignOps, Num};

pub trait VectorElement:
    Num + NumAssignOps<Self> + Clone + Copy + std::fmt::Debug + Default
{
}

impl<T> VectorElement for T where T: Num + NumAssignOps<Self> + Clone + Copy + std::fmt::Debug + Default {}

pub trait VectorLike<T: VectorElement, const DIMENSION: usize> {
    fn get(&self, index: usize) -> &T;
}

impl<T: VectorElement, const DIMENSION: usize> VectorLike<T, DIMENSION> for [T; DIMENSION] {
    fn get(&self, index: usize) -> &T {
        &(*self)[index]
    }
}

impl<T: VectorElement> VectorLike<T, 2> for (T, T) {
    fn get(&self, index: usize) -> &T {
        match index {
            0 => &self.0,
            1 => &self.1,
            _ => panic!("Out of range: {index}"),
        }
    }
}

impl<T: VectorElement> VectorLike<T, 3> for (T, T, T) {
    fn get(&self, index: usize) -> &T {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Out of range: {index}"),
        }
    }
}

impl<T: VectorElement> VectorLike<T, 4> for (T, T, T, T) {
    fn get(&self, index: usize) -> &T {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            _ => panic!("Out of range: {index}"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vector_like_get_array() {
        let a = [0, 1, 2, 3];
        assert_eq!(a.get(0), &0);
        assert_eq!(a.get(1), &1);
        assert_eq!(a.get(2), &2);
        assert_eq!(a.get(3), &3);
    }

    #[test]
    #[should_panic]
    fn vector_like_get_array_out_of_range() {
        let a = [0, 1, 2, 3];
        let _ = a.get(4);
    }

    #[test]
    fn vector_like_get_tuple_2() {
        let a = (1, 2);
        assert_eq!(a.get(0), &1);
        assert_eq!(a.get(1), &2);
    }

    #[test]
    fn vector_like_get_tuple_3() {
        let a = (1, 2, 3);
        assert_eq!(a.get(0), &1);
        assert_eq!(a.get(1), &2);
        assert_eq!(a.get(2), &3);
    }

    #[test]
    fn vector_like_get_tuple_4() {
        let a = (1, 2, 3, 4);
        assert_eq!(a.get(0), &1);
        assert_eq!(a.get(1), &2);
        assert_eq!(a.get(2), &3);
        assert_eq!(a.get(3), &4);
    }

    #[test]
    #[should_panic]
    fn vector_like_get_tuple_out_of_range() {
        let a = (1, 2, 3, 4);
        let _ = a.get(4);
    }
}
