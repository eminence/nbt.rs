
#[crate_type = "lib"];
#[link(name="nbt", vers="0.1")];

mod NBT {
    use std::io::Reader;
    
    #[deriving(Eq, ToStr, FromPrimitive)]
    pub enum TagType {
        TAG_End = 0,
        TAG_Byte = 1,
        TAG_Short = 2,
        TAG_Int = 3,
        TAG_Long = 4,
        TAG_Float = 5,
        TAG_Double = 6,
        TAG_Byte_Array = 7,
        TAG_String = 8,
        TAG_List = 9,
        TAG_Compound = 10,
        TAG_Unknown
    }

    pub enum NBTTag {
        ByteTag(i8),
        ShortTag(i16),
        IntTag(i32),
        LongTag(i64),
        FloatTag(f32),
        DoubleTag(f64),
        ByteArrayTag(~[u8]),
        StringTag(~str),
        ListTag(TagType, uint, ~[~NBTTag]),
        CompoundTag(~[~NamedTag])
    }
    
    impl NBTTag {
        fn get_type(&self) -> TagType {
            match *self {
                ByteTag(_) => TAG_Byte,
                ShortTag(_) => TAG_Short,
                IntTag(_) => TAG_Int,
                LongTag(_) => TAG_Long,
                FloatTag(_) => TAG_Float,
                DoubleTag(_) => TAG_Double,
                ByteArrayTag(_) =>TAG_Byte_Array,
                StringTag(_) => TAG_String,
                ListTag(_, _, _) => TAG_List,
                CompoundTag(_) => TAG_Compound
            }
        }
    }

    impl NBTTag {
        fn build(r: &mut Reader, tt: TagType) -> NBTTag {
            match tt {
                TAG_End => fail!("Cannot build a TAG_End"),
                TAG_Byte => ByteTag(r.read_i8()),
                TAG_Short => ShortTag(r.read_be_i16()),
                TAG_Int => IntTag(r.read_be_i32()),
                TAG_Long => LongTag(r.read_be_i64()),
                TAG_Float => FloatTag(r.read_be_f32()),
                TAG_Double => DoubleTag(r.read_be_f64()),
                TAG_Byte_Array => {
                    let len = r.read_be_u32() as uint;
                    let bytes = r.read_bytes(len);
                    ByteArrayTag(bytes)
                },
                TAG_String => {
                    let len = r.read_be_u16() as uint;
                    let name: ~str = ::std::str::from_utf8_owned(r.read_bytes(len));
                    StringTag(name)
                },
                TAG_List => {
                    let tt: TagType = FromPrimitive::from_u8(r.read_u8()).unwrap();
                    let num_elems :uint = r.read_be_u32() as uint;
                    let mut elems: ~[~NBTTag] = ::std::vec::with_capacity(num_elems);
                    let mut counter: uint = 0;
                    while counter < num_elems {
                        elems.push(box NBTTag::build(r, tt));
                        counter += 1;
                    }
                    ListTag(tt, num_elems, elems)
                },
                TAG_Compound => {
                    let mut elems: ~[~NamedTag] = ::std::vec::with_capacity(5);
                    loop {
                        let tt: TagType = FromPrimitive::from_u8(r.read_u8()).unwrap();
                        if tt == TAG_End { break; }
                        let len = r.read_be_u16() as uint;
                        let name: ~str = ::std::str::from_utf8_owned(r.read_bytes(len));
                        elems.push(box NamedTag{_name: name, _value: NBTTag::build(r, tt)});
                    }
                    CompoundTag(elems)
                }
                _ => fail!("Unknown tag {:s}", tt.to_str())
            }
            
        }
        fn _pretty_print(&self, name: Option<&str>, indent: uint) -> ~str {
            let indent_vec: ~[char] = ::std::vec::from_elem(indent, ' ');
            let indent_str = ::std::str::from_chars(indent_vec);
            let mut s: ~str = ~"";
            let name_str = match name {
                None => ~"",
                Some(s) => format!("(\"{}\")", s)
            };
            s.push_str(indent_str);
            s.push_str(match *self {
                ByteTag(v) => format!("TAG_Byte{}: {}", name_str, v.to_str()),
                ShortTag(v) => format!("TAG_Short{}: {}", name_str, v.to_str()),
                IntTag(v) => format!("TAG_Int{}: {}", name_str, v.to_str()),
                LongTag(v) => format!("TAG_Long{}: {}", name_str, v.to_str()),
                FloatTag(v) => format!("TAG_Float{}: {}", name_str, v.to_str()),
                DoubleTag(v) => format!("TAG_Double{}: {}", name_str, v.to_str()),
                StringTag(ref v) => format!("TAG_String{}: {}", name_str, v.to_str()),
                ByteArrayTag(ref v) => format!("TAG_ByteArray{}: {}", name_str, v.to_str()),
                ListTag(tt, len, ref vs) => {
                    let mut r = format!("TAG_List{}: {} entries of type {}\n", name_str, len, tt.to_str() );
                    r.push_str(format!("{}\\{\n", indent_str));
                    for v in vs.iter() {
                        r.push_str(v._pretty_print(None, indent+4));
                        r.push_str("\n");
                    }
                    r.push_str(format!("{}\\}", indent_str));

                    r
                }
                CompoundTag(ref vs) => {
                    let mut r = format!("TAG_Compound{}: {} entries\n", name_str, vs.len());
                    r.push_str(format!("{}\\{\n", indent_str));
                    for v in vs.iter() {
                        r.push_str(v._pretty_print(indent+4));
                        r.push_str("\n");
                    }
                    r.push_str(format!("{}\\}", indent_str));

                    r
                }
            });
            s
        }
    }

    pub struct NamedTag {
        _name: ~str,
        _value: NBTTag
    }
    impl<'a> NamedTag {
        pub fn get_type(&self) -> TagType { self._value.get_type() }
        pub fn get_name(&'a self) -> &'a str { self._name.as_slice() }
        pub fn get_tag(&'a self) -> &'a NBTTag { &self._value }
        pub fn pretty_print(&self) -> ~str { self._pretty_print(0) }
        fn _pretty_print(&self, indent: uint) -> ~str { self._value._pretty_print(Some(self._name.as_slice()), indent) }
    }

    pub struct Parser {
        _reader: ~Reader,
    }
    impl Parser {
        pub fn new(p: ~Reader) -> Parser {
            Parser{_reader: p}
        }

        fn read_name(&mut self) -> ~str {
            // read short to get name length;
            let len = self._reader.read_be_u16() as uint;
            ::std::str::from_utf8_owned(self._reader.read_bytes(len))
        }

        pub fn parse(&mut self) -> ~NamedTag {
            let tt: TagType = FromPrimitive::from_u8(self._reader.read_u8()).unwrap();
            if tt != TAG_Compound { fail!("Expected a TAG_Compound for first tag in NBT file"); }
            let name = self.read_name();
            let tag : NBTTag = NBTTag::build(self._reader, TAG_Compound);
            box NamedTag { _name: name, _value: tag}
        }
    }

}


#[test]
fn test_byte() {
    let data: ~str = ~"\x0a\x00\x04abcd\x01\x00\x04test\x01\x00";
    let bytes = ~std::io::mem::MemReader::new(data.into_bytes());
    let mut parser = NBT::Parser::new(bytes as ~Reader);
    let root: ~NBT::NamedTag = parser.parse();
    assert!(root.get_name() == "abcd");
    match *root.get_tag() {
        NBT::CompoundTag(ref vs) => {
            assert!(vs.len() == 1);
            let sub_tag : &NBT::NamedTag = vs[0];
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
    let bytes = ~std::io::mem::MemReader::new(data.into_bytes());
    let mut parser = NBT::Parser::new(bytes as ~Reader);
    let root: ~NBT::NamedTag = parser.parse();
    assert!(root.get_name() == "abcd");
    match *root.get_tag() {
        NBT::CompoundTag(ref vs) => {
            assert!(vs.len() == 1);
            let sub_tag : &NBT::NamedTag = vs[0];
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
    let bytes = ~std::io::mem::MemReader::new(data.into_bytes());
    let mut parser = NBT::Parser::new(bytes as ~Reader);
    let root: ~NBT::NamedTag = parser.parse();
    assert!(root.get_name() == "abcd");
    match *root.get_tag() {
        NBT::CompoundTag(ref vs) => {
            assert!(vs.len() == 1);
            let sub_tag : &NBT::NamedTag = vs[0];
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
    let bytes = ~std::io::mem::MemReader::new(data.into_bytes());
    let mut parser = NBT::Parser::new(bytes as ~Reader);
    let root: ~NBT::NamedTag = parser.parse();
    assert!(root.get_name() == "abcd");
    match *root.get_tag() {
        NBT::CompoundTag(ref vs) => {
            assert!(vs.len() == 1);
            let sub_tag : &NBT::NamedTag = vs[0];
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
    let bytes = ~std::io::mem::MemReader::new(data.into_bytes());
    let mut parser = NBT::Parser::new(bytes as ~Reader);
    let root: ~NBT::NamedTag = parser.parse();
    assert!(root.get_name() == "abcd");
    match *root.get_tag() {
        NBT::CompoundTag(ref vs) => {
            assert!(vs.len() == 1);
            let sub_tag : &NBT::NamedTag = vs[0];
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
    let data: ~str = ~"\x0a\x00\x04abcd\x08\x00\x05world\x00\x0chello world!";
    let bytes = ~std::io::mem::MemReader::new(data.into_bytes());
    let mut parser = NBT::Parser::new(bytes as ~Reader);
    let root: ~NBT::NamedTag = parser.parse();
    assert!(root.get_name() == "abcd");
    match *root.get_tag() {
        NBT::CompoundTag(ref vs) => {
            assert!(vs.len() == 1);
            let sub_tag : &NBT::NamedTag = vs[0];
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
    assert!(root.get_type() == NBT::TAG_Compound);
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
    assert!(root.get_type() == NBT::TAG_Compound);
    let s = root.pretty_print();

    // if  you actually want to see the pretty-printed tree, set the NBT_PRETTYPRINT envvar
    let xx : std::c_str::CString = "NBT_PRETTYPRINT".to_c_str();
    unsafe {
        if ! std::libc::funcs::c95::stdlib::getenv(xx.unwrap()).is_null() {
            println(s);
        }
    }
}
