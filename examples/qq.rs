fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    let nth = find_nth(&mut v, 31);
    println!("{}", nth);
}

fn find_nth<T: Ord + Clone>(elems: &mut [T], n: usize) -> T {
    elems.sort();
    let t = &elems[n];
    return t.clone();
}
