use core::io::{ReaderUtil, WriterUtil};

pub fn read() -> ~str {
    let input = io::stdin().read_line();
    //io::stderr().write_line(fmt!("READ: %s", input));
    return input;
}

pub fn write(s: &str) {
    io::stdout().write_line(s);
    //io::stderr().write_line(fmt!("WRITE: %s", s));
}

pub fn debug<T: ToStr>(object: T) {
    io::stderr().write_line(fmt!("DEBUG: %s", object.to_str()));
}
