use actual::math::add::add_two;

pub mod add_two_tests {
    use super::*;

    #[test]
    fn adds_positive_numbers() {
        assert_eq!(add_two(2, 3), 5);
    }

    #[test]
    fn adds_zero() {
        assert_eq!(add_two(42, 0), 42);
    }
}
pub mod add_vector {
    use actual::math::add::sum_of_vector;

    // use super::*; // or parents shi
    #[test]
    fn add_vec1() {
        let a = vec![0];
        assert_eq!(sum_of_vector(a), 0);
    }
    #[test]
    fn add_vec2() {
        let a = vec![1];
        assert_eq!(sum_of_vector(a), 1);
    }
    #[test]
    fn add_vec3() {
        let a = vec![20, 30, 40];
        assert_eq!(sum_of_vector(a), 90);
        // assert_eq!(sum_of_vector(a), );
    }
    #[test]
    fn add_vec4() {
        let a: Vec<i32> = Vec::new();
        // let result
        assert_eq!(sum_of_vector(a), 0);
    }
}
pub mod add_5 {
    use super::*;
    #[test]
    fn add_zero() {
        assert_eq!(add_two(0, 0), 0);
    }
}
/* mod vec_add_tests {
    use super::*;

    #[test]
    fn adds_vectors() {
        assert_eq!(vec_add(vec![1, 2], vec![3, 4]), vec![4, 6]);
    }

    #[test]
    fn empty_vectors() {
        assert_eq!(vec_add(vec![], vec![]), Vec::<i32>::new());
    }
}
 */
