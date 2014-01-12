
use std::os;
use std::io;
use std::libc;
use std::vec;
use std::io::File;

fn isprint(c:u8) -> bool {
   unsafe { libc::isprint(c as libc::c_int) != 0 }
}

fn format(b: &[u8], offset: uint) {

    // optimize common case
    if b.len() == 16 {
        let mut p : [char,  ..16] = [0u8 as char,.. 16];
        for (i, c) in b.iter().enumerate() {
            p[i] = if isprint(*c) { *c as char } else { '.' };
        }

        println!(
        "{:08x}:  {:02x} {:02x} {:02x} {:02x}  {:02x} {:02x} {:02x} {:02x}  {:02x} {:02x} {:02x} {:02x}  {:02x} {:02x} {:02x} {:02x}  |{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}|",
        offset,
        b[0] as uint, b[1] as uint, b[2] as uint, b[3] as uint,
        b[4] as uint, b[5] as uint, b[6] as uint, b[7] as uint,
        b[8] as uint, b[9] as uint, b[10] as uint, b[11] as uint,
        b[12] as uint, b[13] as uint, b[14] as uint, b[15] as uint,

        p[0], p[1], p[2], p[3],
        p[4], p[5], p[6], p[7],
        p[8], p[9], p[10], p[11],
        p[12], p[13], p[14], p[15]
        );

        return
    }

    let mut offset = offset;

    let mut hd : ~str = ~"";

//    let example = ~"00019ae0:  6e 74 31 37  5f 64 31 63  34 36 33 66  63 64 66 63  |nt17_d1c463fcdfc|\n";
//    str::reserve(&mut hd, example.len())

    hd.push_str( format!("{:08x}: ",offset) );

    for c in b.iter() {
        if offset % 4 == 0 { hd.push_str( " " ); }
        hd.push_str( format!("{:02x} ",*c as uint) );
        offset += 1;
    }

    assert!( b.len() != 16 );

    for i in range(b.len(), 16) {
        if i != 0 && i % 4 == 0 { hd.push_str(" "); }
        hd.push_str("   ");
    }

    hd.push_str(" |");

    for c in b.iter() {
        hd.push_str( format!("{}",if isprint(*c) {*c as char}else{ '.' }) );
    }

    hd.push_str("|");

    io::println(hd)
}

fn hd(fin: &mut Reader) {

    static bufsiz: uint = 16;

    let mut buf = vec::from_elem(bufsiz, 0u8);
    let mut offset = 0;

    while !fin.eof() {
        let n = match fin.read(buf) {
            None => break,
            Some(..) => buf.len()
        };

        format(buf.slice(0, n), offset);
        offset += n
    }
}

fn main() {

    let args = os::args();

    if args.len() == 1 {
        hd(&mut io::stdin());
        return
    }

    let mut it = args.slice(1, args.len()).iter();

    for arg in it {
        match File::open(&Path::new(arg.to_owned())) {
          Some(mut f) => { hd(&mut f) }
          None => {
            let err = format!("{}: unable to open {}\n",args[0],*arg);
            io::stderr().write_str(err)
          }
        }
    }
}
