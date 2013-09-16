
use std::os;
use std::io;
use std::libc;
use std::vec;
use std::result;

#[fixed_stack_segment]
fn isprint(c:u8) -> bool {
   unsafe { libc::isprint(c as libc::c_int) != 0 }
}

fn format(b: &[u8], offset: uint) {

    // optimize common case
    if b.len() == 16 {
        let mut p : [u8, ..16] = [0 as u8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
        for (i, c) in b.iter().enumerate() {
            p[i] = if isprint(*c as u8) { *c as u8 } else { '.' as u8 }
        }

        io::println(fmt!(
        "%08x:  %02x %02x %02x %02x  %02x %02x %02x %02x  %02x %02x %02x %02x  %02x %02x %02x %02x  |%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c%c|",
        offset,
        b[0] as uint, b[1] as uint, b[2] as uint, b[3] as uint,
        b[4] as uint, b[5] as uint, b[6] as uint, b[7] as uint,
        b[8] as uint, b[9] as uint, b[10] as uint, b[11] as uint,
        b[12] as uint, b[13] as uint, b[14] as uint, b[15] as uint,

        p[0] as char, p[1] as char, p[2] as char, p[3] as char,
        p[4] as char, p[5] as char, p[6] as char, p[7] as char,
        p[8] as char, p[9] as char, p[10] as char, p[11] as char,
        p[12] as char, p[13] as char, p[14] as char, p[15] as char
        ));

        return
    }

    let mut offset = offset;

    let mut hd : ~str = ~"";

//    let example = ~"00019ae0:  6e 74 31 37  5f 64 31 63  34 36 33 66  63 64 66 63  |nt17_d1c463fcdfc|\n";
//    str::reserve(&mut hd, example.len())

    hd.push_str( fmt!("%08x: ",offset) );

    for c in b.iter() {
        if offset % 4 == 0 { hd.push_str( " " ); }
        hd.push_str( fmt!("%02x ",*c as uint) );
        offset += 1;
    }

    assert!( b.len() != 16 );

    for i in range(b.len(), 16) {
        if i != 0 && i % 4 == 0 { hd.push_str(" "); }
        hd.push_str("   ");
    }

    hd.push_str(" |");

    for c in b.iter() {
        hd.push_str( fmt!("%c",if isprint(*c) {*c as char}else{'.'as char}) );
    }

    hd.push_str("|");

    io::println(hd)
}

fn hd(fin: @io::Reader) {

    static bufsiz: uint = 16u;

    let mut buf = vec::from_elem(bufsiz, 0u8);
    let mut offset = 0;

    while !fin.eof() {
        let n = fin.read(buf, bufsiz);
        if n != 0 {
            format(buf.slice( 0, n), offset);
        }
        offset += n
    }
}

fn main() {

    let args = os::args();

    if args.len() == 1 { hd(io::stdin()); return }

    let mut it = args.slice(1, args.len()).iter();

    for arg in it {
        match io::file_reader(&Path(*arg)) {
          result::Ok(f) => { hd(f) }
          result::Err(e) => {
            let err = fmt!("%s: %s: %s\n",args[0],*arg,e);
            io::stderr().write_str(err)
          }
        }
    }
}
