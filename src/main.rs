use vek::custom::Vek;

fn main() {
    let mut vek: Vek<usize> = Vek::new();
    vek.push(1);
    vek.push(2);
    vek.push(3);
    vek.push(4);
    vek.push(6);

    assert_eq!(vek.len(), 5usize);
    assert_eq!(vek.capacity(), 8usize);
}
