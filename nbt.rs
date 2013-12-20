
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

    trait TagName {
        fn build(r: &mut Reader) -> Self;
    }
    impl TagName for ~str {
        fn build(r: &mut Reader) -> ~str { 
            let len = r.read_be_u16() as uint;
            ::std::str::from_utf8_owned(r.read_bytes(len))
            }
    }
    impl TagName for () {
        fn build(r: &mut Reader) {}
    }

    pub trait Tag<T: TagName> {
        fn get_type(&self) -> TagType;
    }

    pub trait NamedTag<'a> : Tag<~str> {
        fn get_name(&'a self) -> &'a ~str;
        fn build(r: &mut Reader) -> ~Self;
    }
    pub trait UnnamedTag : Tag<()> {
        fn build(r: &mut Reader) -> ~Self;
    }


    struct StringTag {
        _value: ~str,
        _name: ~str
    }
    impl<'a> NamedTag<'a> for StringTag {
        fn get_name(&'a self) -> &'a ~str { &'a self._name }
        fn build(r: &mut Reader) -> ~StringTag {
            let name: ~str = TagName::build(r);
            let len = r.read_be_u16() as uint;
            let value = ::std::str::from_utf8_owned(r.read_bytes(len));
            ~StringTag{_value: value, _name: name}
        }
    }
    impl<T: TagName> Tag<T> for StringTag { fn get_type(&self) -> TagType { TAG_String } }
    impl UnnamedTag for StringTag {
        fn build(r: &mut ::std::io::Reader) -> ~StringTag {
            let len = r.read_be_u16() as uint;
            let value = ::std::str::from_utf8_owned(r.read_bytes(len));
            ~StringTag{_value: value, _name: ~""}
        }
    }

    struct ByteTag { _value: i8, _name: ~str }
    impl<'a> NamedTag<'a> for ByteTag {
        fn get_name(&'a self) -> &'a ~str { &'a self._name }
        fn build(r: &mut Reader) -> ~ByteTag {
            let name: ~str = TagName::build(r);
            ~ByteTag{_value: r.read_i8(), _name: name} 
        }
    }
    impl<T: TagName> Tag<T> for ByteTag { fn get_type(&self) -> TagType { TAG_Byte } }
    impl UnnamedTag for ByteTag {
        fn build(r: &mut Reader) -> ~ByteTag {
            ~ByteTag{_value: r.read_i8(), _name: ~""} 
        }
    }

    struct LongTag { _value: i64, _name: ~str }
    impl<'a> NamedTag<'a> for LongTag {
        fn get_name(&'a self) -> &'a ~str { &'a self._name }
        fn build(r: &mut Reader) -> ~LongTag {
            let name: ~str = TagName::build(r);
            let mut me : ~LongTag = UnnamedTag::build(r);
            me._name = name; me
        }
    }
    impl<T: TagName> Tag<T> for LongTag { fn get_type(&self) -> TagType { TAG_Long } }
    impl UnnamedTag for LongTag {
        fn build(r: &mut Reader) -> ~LongTag { ~LongTag{_value: r.read_be_i64(), _name: ~""} }
    }
    
    struct IntTag { _value: i32, _name: ~str }
    impl<'a> NamedTag<'a> for IntTag {
        fn get_name(&'a self) -> &'a ~str { &'a self._name }
        fn build(r: &mut Reader) -> ~IntTag {
            let name: ~str = TagName::build(r);
            let mut me : ~IntTag = UnnamedTag::build(r);
            me._name = name; me
        }
    }
    impl<T: TagName> Tag<T> for IntTag { fn get_type(&self) -> TagType { TAG_Int } }
    impl UnnamedTag for IntTag {
        fn build(r: &mut Reader) -> ~IntTag { ~IntTag{_value: r.read_be_i32(), _name: ~""} }
    }
    
    struct ShortTag { _value: i16, _name: ~str }
    impl<'a> NamedTag<'a> for ShortTag {
        fn get_name(&'a self) -> &'a ~str { &'a self._name }
        fn build(r: &mut Reader) -> ~ShortTag {
            let name: ~str = TagName::build(r);
            let mut me : ~ShortTag = UnnamedTag::build(r);
            me._name = name; me
        }
    }
    impl<T: TagName> Tag<T> for ShortTag { fn get_type(&self) -> TagType { TAG_Short } }
    impl UnnamedTag for ShortTag {
        fn build(r: &mut Reader) -> ~ShortTag { ~ShortTag{_value: r.read_be_i16(), _name: ~""} }
    }
    
    struct DoubleTag { _value: f64, _name: ~str }
    impl<'a> NamedTag<'a> for DoubleTag {
        fn get_name(&'a self) -> &'a ~str { &'a self._name }
        fn build(r: &mut Reader) -> ~DoubleTag {
            let name: ~str = TagName::build(r);
            let mut me : ~DoubleTag = UnnamedTag::build(r);
            me._name = name; me
        }
    }
    impl<T: TagName> Tag<T> for DoubleTag { fn get_type(&self) -> TagType { TAG_Double } }
    impl UnnamedTag for DoubleTag {
        fn build(r: &mut Reader) -> ~DoubleTag { ~DoubleTag{_value: r.read_be_f64(), _name: ~""} }
    }
    
    struct FloatTag { _value: f32, _name: ~str }
    impl<'a> NamedTag<'a> for FloatTag {
        fn get_name(&'a self) -> &'a ~str { &'a self._name }
        fn build(r: &mut Reader) -> ~FloatTag {
            let name: ~str = TagName::build(r);
            let mut me : ~FloatTag = UnnamedTag::build(r);
            me._name = name; me
        }
    }
    impl<T: TagName> Tag<T> for FloatTag { fn get_type(&self) -> TagType { TAG_Float } }
    impl UnnamedTag for FloatTag {
        fn build(r: &mut Reader) -> ~FloatTag { ~FloatTag{_value: r.read_be_f32(), _name: ~""} }
    }

    struct ListTag {
        _name: ~str,
        _type: TagType,
        _values: ~[~UnnamedTag]
    }
    impl<'a> NamedTag<'a> for ListTag {
        fn get_name(&'a self) -> &'a ~str {&'a self._name}
        fn build(r: &mut Reader) -> ~ListTag {
            let name: ~str = TagName::build(r);
            let mut me : ~ListTag = UnnamedTag::build(r);
            me._name = name; me
        }
    }
    impl<T: TagName> Tag<T> for ListTag{ fn get_type(&self) -> TagType { TAG_List } }
    impl UnnamedTag for ListTag {
        fn build(r: &mut Reader) -> ~ListTag {
            let tt: TagType = FromPrimitive::from_u8(r.read_u8()).unwrap();
            let len: uint = r.read_be_i32() as uint;
            let mut elems : ~[~UnnamedTag] = ::std::vec::with_capacity(len);
            let mut c = 0;
            while c < len {
                let element: ~UnnamedTag = match tt {
                    TAG_Byte => {let _t: ~ByteTag = UnnamedTag::build(r); _t as ~UnnamedTag},
                    TAG_Int => {let _t: ~IntTag = UnnamedTag::build(r); _t as ~UnnamedTag},
                    TAG_Long => {let _t: ~LongTag = UnnamedTag::build(r); _t as ~UnnamedTag},
                    TAG_Compound => {let _t: ~CompoundTag = UnnamedTag::build(r); _t as ~UnnamedTag},
                    TAG_Double => { let _t : ~DoubleTag = UnnamedTag::build(r); _t as ~UnnamedTag},
                    TAG_List => { let _t : ~ListTag = UnnamedTag::build(r); _t as ~UnnamedTag},
                    TAG_Float => { let _t : ~FloatTag = UnnamedTag::build(r); _t as ~UnnamedTag},
                    _ => fail!(format!("Failed to build list with unknown tag type: {}", tt.to_str()))
                };
                elems.push(element);
                c += 1;
            }
            ~ListTag{_name: ~"", _type: tt, _values: elems}
        
        }
    }
    
    struct CompoundTag<'a> {
        _name: ~str,
        _values: ~[~NamedTag<'a>]
    }
    impl<'a> NamedTag<'a> for CompoundTag<'a> {
        fn get_name(&'a self) -> &'a ~str {&'a self._name}
        fn build(r: &mut Reader) -> ~CompoundTag {
            let name: ~str = TagName::build(r);
            let mut me : ~CompoundTag = UnnamedTag::build(r);
            me._name = name; me
        }
    }
    impl<'a, T: TagName> Tag<T> for CompoundTag<'a> { fn get_type(&self) -> TagType { TAG_Compound } }
    impl<'a> UnnamedTag for CompoundTag<'a> {
        fn build(r: &mut Reader) -> ~CompoundTag {
            let mut elems : ~[~NamedTag] = ::std::vec::with_capacity(10);
            loop {
                let tt: TagType = FromPrimitive::from_u8(r.read_u8()).unwrap();
                if tt == TAG_End { break; }
                let tag : ~NamedTag = match tt {
                    TAG_Byte => { let _t : ~ByteTag = NamedTag::build(r); _t as ~NamedTag},
                    TAG_String => { let _t : ~StringTag = NamedTag::build(r); _t as ~NamedTag},
                    TAG_List => { let _t : ~ListTag = NamedTag::build(r); _t as ~NamedTag},
                    TAG_Compound => { let _t : ~CompoundTag = NamedTag::build(r); _t as ~NamedTag}, 
                    TAG_Long => { let _t : ~LongTag = NamedTag::build(r); _t as ~NamedTag},
                    TAG_Int => { let _t : ~IntTag = NamedTag::build(r); _t as ~NamedTag},
                    TAG_Double => { let _t : ~DoubleTag = NamedTag::build(r); _t as ~NamedTag},
                    TAG_Float => { let _t : ~FloatTag = NamedTag::build(r); _t as ~NamedTag},
                    TAG_Short => { let _t : ~ShortTag = NamedTag::build(r); _t as ~NamedTag},
                    _ => fail!(format!("Unknown tag type in compount_tag::build() -- {}", tt.to_str()))
                };
                elems.push(tag);
            }
            ~CompoundTag{_name: ~"",  _values: elems}
        
        }
    }




    pub struct Parser {
        _reader: ~Reader,
    }
    impl Parser {
        pub fn new(p: ~Reader) -> Parser {
            Parser{_reader: p}
        }

        pub fn parse(&mut self) -> ~NamedTag {
            let tt: TagType = FromPrimitive::from_u8(self._reader.read_u8()).unwrap();
            let tag : ~NamedTag = match tt {
                TAG_Byte => { let _t : ~ByteTag = NamedTag::build(self._reader); _t as ~NamedTag},
                TAG_String => { let _t : ~StringTag = NamedTag::build(self._reader); _t as ~NamedTag},
                TAG_List => { let _t : ~ListTag = NamedTag::build(self._reader); _t as ~NamedTag},
                TAG_Compound => { let _t : ~CompoundTag = NamedTag::build(self._reader); _t as ~NamedTag}, 
                TAG_Long => { let _t : ~LongTag = NamedTag::build(self._reader); _t as ~NamedTag},
                TAG_Int => { let _t : ~IntTag = NamedTag::build(self._reader); _t as ~NamedTag},
                _ => fail!(format!("Unknown tag type in parse() -- {}", tt.to_str()))
            };

            return tag;
            // We are expecting at the first tag to be a compound tag
            //let t : CompoundTag::build(self._reader);
            //self._parse(TAG_Unknown, true) 
        }
    }
   
}

#[test]
fn test_byte() {
    let data: ~str = ~"\x01\x00\x04test4";
    let bytes = ~std::io::mem::MemReader::new(data.into_bytes());
    let mut parser = NBT::Parser::new(bytes as ~Reader);
    let root: ~NBT::NamedTag = parser.parse();
    assert!(root.get_type() == NBT::TAG_Byte);
    //let s = root.get_name();
    //assert!(root.get_name() == ~"test");

}

#[test]
fn test1() {
    let levelp = std::path::Path::new("e.dat");
    let level: std::io::File = std::io::File::open(&levelp).unwrap();
    //let bytes: ~[u8] = level.read_to_end();
    //let iter = ~bytes.move_iter();

    let mut parser = NBT::Parser::new(~level as ~Reader);
    parser.parse();
    //let n: &u8 = iter.next().unwrap();
    //println(format!("byte 1 is {}\n", n.to_str()));

    //NBT::parse(iter);
    //let l: uint = bytes.len();
    //println(format!("byte 1 is {}\n", bytes[0]));

}
