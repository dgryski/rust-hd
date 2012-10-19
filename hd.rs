
use std;
import io::{ReaderUtil, WriterUtil};

fn format(b: &[u8], offset: uint) {

    let mut offset = offset;

    io::print(fmt!("%08x: ",offset));

    for b.each |c| {

        if offset % 4 == 0 { io::print(" "); }

        io::print(fmt!("%02x ",c as uint));
        offset += 1;
    }

    if vec::len(b) != 16 {
        for uint::range(vec::len(b), 16) |i| {
            if i != 0 && i % 4 == 0 { io::print(" "); }
            io::print("   ");
        }

    }
    io::print("  ");

    for b.each |c| {
        io::print(fmt!("%c",if libc::isprint(c as libc::c_int)!=0{c as char}else{'.'as char}));
    }

    io::print("\n");
}

fn hd(fin: io::Reader) {

    const bufsiz: uint = 16u;

    let mut buf = vec::from_elem(bufsiz, 0u8);

    let mut offset = 0;

    while !fin.eof() {
        let n = fin.read(buf, bufsiz);
        format(vec::view(buf, 0, n), offset);
        offset += n
    }
}

fn main(args: ~[~str]) {
    if vec::len(args) == 1 { hd(io::stdin()); return }

    for vec::view(args, 1, vec::len(args)).each |arg| {
        match io::file_reader(&Path(arg)) {
          result::Ok(f) => { hd(f) }
          result::Err(e) => {
            let err = fmt!("%s: %s: %s\n",args[0],arg,e);
            io::stderr().write_str(err)
          }
        }
    }
}
