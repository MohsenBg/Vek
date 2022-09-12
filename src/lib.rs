#[macro_use]
pub mod custom;

#[cfg(test)]
mod tests {
    use crate::custom::vek::Vek;
    #[test]
    fn empty_vek() {
        let vek: Vek<usize> = Vek::new();
        assert_eq!(vek.len(), 0usize);
        assert_eq!(vek.capacity(), 0usize);
    }

    #[test]
    fn singel_vek() {
        let mut vek: Vek<usize> = Vek::new();
        vek.push(5);
        assert_eq!(vek.len(), 1usize);
        assert_eq!(vek.capacity(), 4usize);
    }

    #[test]
    fn multi_vek() {
        let mut vek: Vek<usize> = Vek::new();
        vek.push(1);
        vek.push(2);
        vek.push(3);
        vek.push(4);
        vek.push(5);
        assert_eq!(vek.len(), 5usize);
        assert_eq!(vek.capacity(), 8usize);
        for idx in 0..vek.len() {
            assert_eq!(vek.get(idx), Some(&(idx + 1)));
        }
        assert_eq!(vek.get(6), None);
    }

    #[test]
    fn pop() {
        let mut vek: Vek<usize> = Vek::new();
        vek.push(1);
        vek.push(2);
        vek.push(3);
        vek.push(4);
        vek.push(5);

        let current_len = vek.len();
        vek.pop();
        vek.pop();
        vek.pop();

        for idx in 0..vek.len() {
            assert_eq!(vek.get(idx), Some(&(idx + 1)));
        }

        for idx in vek.len() + 1..current_len {
            assert_eq!(vek.get(idx), None);
        }
    }

    #[test]
    fn clear() {
        let mut vek: Vek<usize> = Vek::new();
        vek.push(1);
        vek.push(2);
        vek.push(3);
        vek.push(4);
        vek.push(5);
        vek.clear();
        assert_eq!(vek.len(), 0usize);
        assert_eq!(vek.capacity(), 0usize);
    }

    #[test]
    fn itor() {
        let mut vek: Vek<usize> = Vek::new();
        vek.push(1);
        vek.push(2);
        vek.push(3);
        vek.push(4);
        vek.push(5);

        #[allow(unused)]
        let mut value = 1usize;

        value = 1usize;
        for item in &vek {
            assert_eq!(*item, value);
            value += 1usize;
        }

        value = 1usize;
        for item in &mut vek {
            assert_eq!(*item, value);
            value += 1usize;
        }

        value = 1usize;
        for item in vek {
            assert_eq!(item, value);
            value += 1usize;
        }
    }

    #[test]
    fn macro_empty_vek() {
        let vek: Vek<usize> = vek![];
        assert_eq!(vek.len(), 0usize);
        assert_eq!(vek.capacity(), 0usize);
    }

    #[test]
    fn macro_multi_value_vek() {
        let vek: Vek<usize> = vek![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut value = 1usize;
        for item in vek {
            assert_eq!(item, value);
            value += 1usize;
        }
    }

    #[test]
    fn macro_count_vek() {
        let vek: Vek<usize> = vek![5;10];
        for item in vek {
            assert_eq!(item, 5);
        }
    }

    #[test]
    fn macro_vek_range() {
        let vek: Vek<usize> = vek![1=>10];
        let mut value = 1usize;
        for item in vek {
            assert_eq!(item, value);
            value += 1usize;
        }
    }
}
