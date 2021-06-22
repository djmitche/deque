use deque::*;

fn main() {
    let mut d: Deque<u32> = Deque::new();
    eprintln!("insert 13");
    d.insert_head(13);
    eprintln!("insert 14");
    d.insert_head(14);
    eprintln!("pop 14");
    assert_eq!(d.pop_head(), Some(14));
    eprintln!("pop 13");
    assert_eq!(d.pop_head(), Some(13));
    eprintln!("pop None");
    assert_eq!(d.pop_head(), None);
}
