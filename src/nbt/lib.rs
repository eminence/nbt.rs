#![crate_type = "lib"]
#![crate_id = "nbt#0.2"]

pub mod NBT {
    use std::io::Reader;
    use std::vec::Vec;
    
    #[deriving(Eq, Show, FromPrimitive)]
    pub enum TagType {
        TAGEnd = 0,
        TAGByte = 1,
        TAGShort = 2,
        TAGInt = 3,
        TAGLong = 4,
        TAGFloat = 5,
        TAGDouble = 6,
        TAGByteArray = 7,
        TAGString = 8,
        TAGList = 9,
        TAGCompound = 10,
        TAGUnknown
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
        ListTag(TagType, uint, Vec<NBTTag>),
        CompoundTag(Vec<NamedTag>)
    }
    
    impl NBTTag {
        fn get_type(&self) -> TagType {
            match *self {
                ByteTag(_) => TAGByte,
                ShortTag(_) => TAGShort,
                IntTag(_) => TAGInt,
                LongTag(_) => TAGLong,
                FloatTag(_) => TAGFloat,
                DoubleTag(_) => TAGDouble,
                ByteArrayTag(_) =>TAGByteArray,
                StringTag(_) => TAGString,
                ListTag(_, _, _) => TAGList,
                CompoundTag(_) => TAGCompound
            }
        }
    }

    impl NBTTag {
        fn build(r: &mut Reader, tt: TagType) -> NBTTag {
            match tt {
                TAGEnd => fail!("Cannot build a TAGEnd"),
                TAGByte => ByteTag(r.read_i8().unwrap()),
                TAGShort => ShortTag(r.read_be_i16().unwrap()),
                TAGInt => IntTag(r.read_be_i32().unwrap()),
                TAGLong => LongTag(r.read_be_i64().unwrap()),
                TAGFloat => FloatTag(r.read_be_f32().unwrap()),
                TAGDouble => DoubleTag(r.read_be_f64().unwrap()),
                TAGByteArray => {
                    let len = r.read_be_u32().unwrap() as uint;
                    let bytes = r.read_exact(len).unwrap();
                    ByteArrayTag(bytes)
                },
                TAGString => {
                    let len = r.read_be_u16().unwrap() as uint;
                    println!("TAGString trying to read {} bytes", len);
                    let name: ~str = ::std::str::from_utf8_owned(r.read_exact(len).unwrap()).unwrap();
                    println!("Got name!");
                    StringTag(name)
                },
                TAGList => {
                    let tt: TagType = FromPrimitive::from_u8(r.read_u8().unwrap()).unwrap();
                    let num_elems :uint = r.read_be_u32().unwrap() as uint;
                    let mut elems: Vec<NBTTag> = ::std::vec::Vec::with_capacity(num_elems);
                    let mut counter: uint = 0;
                    while counter < num_elems {
                        elems.push(NBTTag::build(r, tt));
                        counter += 1;
                    }
                    ListTag(tt, num_elems, elems)
                },
                TAGCompound => {
                    let mut elems: Vec<NamedTag> = ::std::vec::Vec::with_capacity(5);
                    loop {
                        let tt: TagType = FromPrimitive::from_u8(r.read_u8().unwrap()).unwrap();
                        if tt == TAGEnd { break; }
                        let len = r.read_be_u16().unwrap() as uint;
                        let name: ~str = ::std::str::from_utf8_owned(r.read_exact(len).unwrap()).unwrap();
                        elems.push(NamedTag{_name: name, _value: NBTTag::build(r, tt)});
                    }
                    CompoundTag(elems)
                }
                _ => fail!("Unknown tag {:s}", tt.to_str())
            }
            
        }
        fn _pretty_print(&self, name: Option<&str>, indent: uint) -> ~str {
            let indent_vec: Vec<char> = Vec::from_elem(indent, ' ');
            let indent_str = ::std::str::from_chars(indent_vec.as_slice());
            let mut s: ~str = ~"";
            let name_str = match name {
                None => ~"",
                Some(s) => format!("(\"{}\")", s)
            };
            s.push_str(indent_str);
            s.push_str(match *self {
                ByteTag(v) => format!("TAGByte{}: {}", name_str, v.to_str()),
                ShortTag(v) => format!("TAGShort{}: {}", name_str, v.to_str()),
                IntTag(v) => format!("TAGInt{}: {}", name_str, v.to_str()),
                LongTag(v) => format!("TAGLong{}: {}", name_str, v.to_str()),
                FloatTag(v) => format!("TAGFloat{}: {}", name_str, v.to_str()),
                DoubleTag(v) => format!("TAGDouble{}: {}", name_str, v.to_str()),
                StringTag(ref v) => format!("TAGString{}: {}", name_str, v.to_str()),
                ByteArrayTag(ref v) => format!("TAGByteArray{}: {}", name_str, v.to_str()),
                ListTag(tt, len, ref vs) => {
                    let mut r = format!("TAGList{}: {} entries of type {}\n", name_str, len, tt.to_str() );
                    r.push_str(format!("{}\\{\n", indent_str));
                    for v in vs.iter() {
                        r.push_str(v._pretty_print(None, indent+4));
                        r.push_str("\n");
                    }
                    r.push_str(format!("{}\\}", indent_str));

                    r
                }
                CompoundTag(ref vs) => {
                    let mut r = format!("TAGCompound{}: {} entries\n", name_str, vs.len());
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
            let len = self._reader.read_be_u16().unwrap() as uint;
            ::std::str::from_utf8_owned(self._reader.read_exact(len).unwrap()).unwrap()
        }

        pub fn parse(&mut self) -> ~NamedTag {
            let tt: TagType = FromPrimitive::from_u8(self._reader.read_u8().unwrap()).unwrap();
            if tt != TAGCompound { fail!("Expected a TAGCompound for first tag in NBT file"); }
            let name = self.read_name();
            let tag : NBTTag = NBTTag::build(self._reader, TAGCompound);
            box NamedTag { _name: name, _value: tag}
        }
    }

}

