pub mod custom;

#[cfg(test)]
mod tests {

    use crate::custom::Vek;
    #[test]
    fn test_vek() {
        let mut vec = Vec::new();
        vec.push(1usize);
        vec.push(2);
        vec.push(3);
        assert_eq!(vec.capacity(), 4);
        assert_eq!(vec.len(), 3);
    }

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
            assert_eq!(vek.get(idx), Some(idx + 1));
        }
    }
}
