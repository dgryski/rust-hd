
extern mod std;
use io::{ReaderUtil, WriterUtil};

fn format(b: &[u8], offset: uint) {

    // optimize common case
    if vec::len(b) == 16 {
        let p : [mut u8 * 16] = [mut 0 as u8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]/16;
        for b.eachi |i,c| {
            p[i] = if libc::isprint(*c as libc::c_int) != 0 { *c as u8 } else { '.' as u8 }
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

    let example = ~"00019ae0:  6e 74 31 37  5f 64 31 63  34 36 33 66  63 64 66 63  |nt17_d1c463fcdfc|\n";

    str::reserve(&mut hd, str::len(example));

    hd += fmt!("%08x: ",offset);

    for b.each |c| {
        if offset % 4 == 0 { hd += " "; }
        hd += fmt!("%02x ",*c as uint);
        offset += 1;
    }

    assert vec::len(b) != 16;
    for uint::range(vec::len(b), 16) |i| {
        if i != 0 && i % 4 == 0 { hd += " "; }
        hd += "   ";
    }

    hd += " |";

    for b.each |c| {
        hd += fmt!("%c",if libc::isprint(*c as libc::c_int)!=0{*c as char}else{'.'as char});
    }

    hd += "|";

    io::println(hd)
}

fn hd(fin: io::Reader) {

    const bufsiz: uint = 16u;

    let mut buf = vec::from_elem(bufsiz, 0u8);
    let mut offset = 0;

    while !fin.eof() {
        let n = fin.read(buf, bufsiz);
        if n != 0 {
            format(vec::view(buf, 0, n), offset);
        }
        offset += n
    }
}


fn main() {

    let args = os::args();

    if vec::len(args) == 1 { hd(io::stdin()); return }

    for vec::view(args, 1, vec::len(args)).each |arg| {
        match io::file_reader(&Path(*arg)) {
          result::Ok(f) => { hd(f) }
          result::Err(e) => {
            let err = fmt!("%s: %s: %s\n",args[0],*arg,e);
            io::stderr().write_str(err)
          }
        }
    }
}
