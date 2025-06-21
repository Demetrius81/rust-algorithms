#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

#[derive(Debug)]
enum Foo {
    Int(u32),
    Char(char),
}

#[derive(Debug)]
enum Inner {
    A(u32),
    B(u32),
}

#[derive(Debug)]
enum Outer {
    C(u32),
    D(Inner),
}

pub fn print_memory_representation<T>(t: T)
where
    T: std::fmt::Debug,
{
    print!("type={} value={t:?}: ", std::any::type_name::<T>());
    let start = &t as *const _ as *const u8;
    for i in 0..std::mem::size_of::<T>() {
        print!("{:02x} ", unsafe { *start.offset(i as isize) });
    }
    println!();
}

pub fn run() {
    print_memory_representation(Foo::Int(5));

    print_memory_representation(Foo::Char('A'));

    let a: char = 'A';
    print_memory_representation(a);

    print_memory_representation(Some(a));

    let none: Option<char> = None;
    print_memory_representation(none);

    print_memory_representation(Inner::A(2));

    print_memory_representation(Inner::B(3));

    print_memory_representation(Outer::C(5));

    print_memory_representation(Outer::D(Inner::A(2)));

    print_memory_representation(Outer::D(Inner::B(3)));
}
