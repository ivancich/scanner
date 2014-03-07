#[allow(unused_imports)];

use std::io::fs::File;
use std::io::mem::MemReader;
use Scanner;


#[test]
fn mem_test() {
    let data = bytes!("   17 -23  \t   +41\n12345\t99");
    let buffer = MemReader::new(data.to_owned());
    let mut s = Scanner::new_from_reader(~buffer as ~Reader);

    println("starting file test");

    assert_eq!(Some(17), s.next_int());
    assert_eq!(Some(-23), s.next_int());
    assert_eq!(Some(41), s.next_int());
    assert_eq!(Some(12345), s.next_int());
    assert_eq!(Some(99), s.next_int());
    assert_eq!(None, s.next_int());

    println("ending file test");
}


#[test]
#[ignore]
fn file_test() {
    let path = Path::new("nums.txt");
    let file = File::open(&path);
    let mut s = Scanner::new_from_reader(~file as ~Reader);

    println("starting file test");

    assert_eq!(Some(12), s.next_int());
    assert_eq!(Some(13), s.next_int());
    assert_eq!(Some(14), s.next_int());
    assert_eq!(Some(15), s.next_int());
    assert_eq!(Some(-16), s.next_int());
    assert_eq!(Some(17), s.next_int());
    assert_eq!(None, s.next_int());

    println("finished file test");
}
