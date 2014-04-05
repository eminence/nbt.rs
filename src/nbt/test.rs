extern crate nbt;
use nbt::NBT;

#[test]
fn test_byte() {
    let data: ~str = ~"\x0a\x00\x04abcd\x01\x00\x04test\x01\x00";
    let bytes = ~std::io::MemReader::new(data.into_bytes());
    let mut parser = NBT::Parser::new(bytes as ~Reader);
    let root: ~NBT::NamedTag = parser.parse();
    assert!(root.get_name() == "abcd");
    match *root.get_tag() {
        NBT::CompoundTag(ref vs) => {
            assert!(vs.len() == 1);
            let sub_tag : &NBT::NamedTag = vs.get(0);
            assert!(sub_tag.get_name() == "test");
            match *sub_tag.get_tag() {
                NBT::ByteTag(v) => { assert!(v == 1); }
                _ => fail!("Unexpected subtag!")
            }
        },
        _ => fail!("Unexpected tag type!")
    }
}

#[test]
fn test_short() {
    let data: ~str = ~"\x0a\x00\x04abcd\x02\x00\x05hello\x12\x34\x00";
    let bytes = ~std::io::MemReader::new(data.into_bytes());
    let mut parser = NBT::Parser::new(bytes as ~Reader);
    let root: ~NBT::NamedTag = parser.parse();
    assert!(root.get_name() == "abcd");
    match *root.get_tag() {
        NBT::CompoundTag(ref vs) => {
            assert!(vs.len() == 1);
            let sub_tag : &NBT::NamedTag = vs.get(0);
            assert!(sub_tag.get_name() == "hello");
            match *sub_tag.get_tag() {
                NBT::ShortTag(v) => { assert!(v == 4660); }
                _ => fail!("Unexpected subtag!")
            }
        },
        _ => fail!("Unexpected tag type!")
    }
}

#[test]
fn test_int() {
    let data: ~str = ~"\x0a\x00\x04abcd\x03\x00\x05world\x12\x34\x56\x78\x00";
    let bytes = ~std::io::MemReader::new(data.into_bytes());
    let mut parser = NBT::Parser::new(bytes as ~Reader);
    let root: ~NBT::NamedTag = parser.parse();
    assert!(root.get_name() == "abcd");
    match *root.get_tag() {
        NBT::CompoundTag(ref vs) => {
            assert!(vs.len() == 1);
            let sub_tag : &NBT::NamedTag = vs.get(0);
            assert!(sub_tag.get_name() == "world");
            match *sub_tag.get_tag() {
                NBT::IntTag(v) => { assert!(v == 305419896); }
                _ => fail!("Unexpected subtag!")
            }
        },
        _ => fail!("Unexpected tag type!")
    }
}


#[test]
fn test_long() {
    let data: ~str = ~"\x0a\x00\x04abcd\x04\x00\x05world\x12\x34\x56\x78\x12\x34\x56\x78\x00";
    let bytes = ~std::io::MemReader::new(data.into_bytes());
    let mut parser = NBT::Parser::new(bytes as ~Reader);
    let root: ~NBT::NamedTag = parser.parse();
    assert!(root.get_name() == "abcd");
    match *root.get_tag() {
        NBT::CompoundTag(ref vs) => {
            assert!(vs.len() == 1);
            let sub_tag : &NBT::NamedTag = vs.get(0);
            assert!(sub_tag.get_name() == "world");
            match *sub_tag.get_tag() {
                NBT::LongTag(v) => { assert!(v == 1311768465173141112); }
                _ => fail!("Unexpected subtag!")
            }
        },
        _ => fail!("Unexpected tag type!")
    }
}

#[test]
fn test_bytearray() {
    let data: ~str = ~"\x0a\x00\x04abcd\x07\x00\x05world\x00\x00\x00\x0a\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x00";
    let bytes = ~std::io::MemReader::new(data.into_bytes());
    let mut parser = NBT::Parser::new(bytes as ~Reader);
    let root: ~NBT::NamedTag = parser.parse();
    assert!(root.get_name() == "abcd");
    match *root.get_tag() {
        NBT::CompoundTag(ref vs) => {
            assert!(vs.len() == 1);
            let sub_tag : &NBT::NamedTag = vs.get(0);
            assert!(sub_tag.get_name() == "world");
            match *sub_tag.get_tag() {
                NBT::ByteArrayTag(ref v) => { assert!(*v == ~[0,1,2,3,4,5,6,7,8,9]); }
                _ => fail!("Unexpected subtag!")
            }
        },
        _ => fail!("Unexpected tag type!")
    }
}

#[test]
fn test_string() {
    let data: ~str = ~"\x0a\x00\x04abcd\x08\x00\x05world\x00\x0chello world!\x00";
    let bytes = ~std::io::MemReader::new(data.into_bytes());
    let mut parser = NBT::Parser::new(bytes as ~Reader);
    let root: ~NBT::NamedTag = parser.parse();
    println!("pasred!");
    assert!(root.get_name() == "abcd");
    match *root.get_tag() {
        NBT::CompoundTag(ref vs) => {
            assert!(vs.len() == 1);
            let sub_tag : &NBT::NamedTag = vs.get(0);
            println!("Have subtag!");
            assert!(sub_tag.get_name() == "world");
            match *sub_tag.get_tag() {
                NBT::StringTag(ref v) => { assert!(*v == ~"hello world!"); }
                _ => fail!("Unexpected subtag!")
            }
        },
        _ => fail!("Unexpected tag type!")
    }
}


#[test]
fn test_e_dat() {
    let levelp = std::path::Path::new("e.dat");
    let level: std::io::File = std::io::File::open(&levelp).unwrap();

    let mut parser = NBT::Parser::new(~level as ~Reader);
    let root: ~NBT::NamedTag = parser.parse();
    assert!(root.get_type() == NBT::TAGCompound);
    //let n: &u8 = iter.next().unwrap();
    //println(format!("byte 1 is {}\n", n.to_str()));

    //NBT::parse(iter);
    //let l: uint = bytes.len();
    //println(format!("byte 1 is {}\n", bytes[0]));

}

#[test]
fn test_print_e_data() {
    let levelp = std::path::Path::new("e.dat");
    let level: std::io::File = std::io::File::open(&levelp).unwrap();

    let mut parser = NBT::Parser::new(~level as ~Reader);
    let root: ~NBT::NamedTag = parser.parse();
    assert!(root.get_type() == NBT::TAGCompound);
    let s = root.pretty_print();

    // if  you actually want to see the pretty-printed tree, set the NBT_PRETTYPRINT envvar
    let xx : std::c_str::CString = "NBT_PRETTYPRINT".to_c_str();
    unsafe {
        if ! std::libc::funcs::c95::stdlib::getenv(xx.unwrap()).is_null() {
            println!("{}", s);
        }
    }
}
