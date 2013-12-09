
#[crate_type = "lib"];
#[link(name="nbt", vers="0.1")];

mod NBT {
#[deriving(Eq, ToStr)]
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

    impl TagType {
        fn from_u8(n: u8) -> TagType {
            match n {
                0 => TAG_End,
                1 => TAG_Byte,
                2 => TAG_Short,
                3 => TAG_Int,
                4 => TAG_Long,
                5 => TAG_Float,
                6 => TAG_Double,
                7 => TAG_Byte_Array,
                8 => TAG_String,
                9 => TAG_List,
                10=> TAG_Compound,
                _ => fail!(format!("Unknown tag type {}", n))
            }
        }
    }

    //impl ToStr for TagType {
    //    fn to_str(&self) -> ~str {
    //        ~"a tag"
    //    }
    //}

    pub trait Tag<'a> {
        fn get_type(&self) -> TagType;
        fn get_name(&'a self) -> &'a str;
    }

    struct StringTag {
        _name: ~str,
        _str: ~str
    }
    impl<'a> Tag<'a> for StringTag {
        fn get_type(&self) -> TagType { TAG_String}
        fn get_name(&'a self) -> &'a str {self._name.as_slice()}
    }

    struct ByteTag {
        _name: ~str,
        _val: i8
    }
    impl<'a> Tag<'a> for ByteTag {
        fn get_type(&self) -> TagType { TAG_Byte}
        fn get_name(&'a self) -> &'a str {self._name.as_slice()}
    }
     
    
    struct ShortTag {
        _name: ~str,
        _val: i16
    }
    impl<'a> Tag<'a> for ShortTag {
        fn get_type(&self) -> TagType { TAG_Short}
        fn get_name(&'a self) -> &'a str {self._name.as_slice()}
    }
    
    struct IntTag {
        _name: ~str,
        _val: i32
    }
    impl<'a> Tag<'a> for IntTag {
        fn get_type(&self) -> TagType { TAG_Int }
        fn get_name(&'a self) -> &'a str {self._name.as_slice()}
    }
    
    struct LongTag {
        _name: ~str,
        _val: i64
    }

    impl<'a> Tag<'a> for LongTag {
        fn get_type(&self) -> TagType { TAG_Long}
        fn get_name(&'a self) -> &'a str {self._name.as_slice()}
    }

    struct FloatTag {
        _name: ~str,
        _val: f32
    }
    impl<'a> Tag<'a> for FloatTag {
        fn get_type(&self) -> TagType { TAG_Float}
        fn get_name(&'a self) -> &'a str {self._name.as_slice()}
    }

    struct DoubleTag {
        _name: ~str,
        _val: f64
    }
    impl<'a> Tag<'a> for DoubleTag {
        fn get_type(&self) -> TagType { TAG_Double}
        fn get_name(&'a self) -> &'a str {self._name.as_slice()}
}

    struct ListTag<'a> {
        _name: ~str,
        _item_type: TagType,
        _items: ~[~Tag<'a>]
    }
    impl<'a> Tag<'a> for ListTag<'a> {
        fn get_type(&self) -> TagType { TAG_List}
        fn get_name(&'a self) -> &'a str {self._name.as_slice()}
}


    struct CompoundTag<'a> {
        _name: ~str,
        _items: ~[~Tag<'a>]
    }
    impl<'a> Tag<'a> for CompoundTag<'a> {
        fn get_type(&self) -> TagType { TAG_Compound}
        fn get_name(&'a self) -> &'a str {self._name.as_slice()}
    }

    struct EndTag;
    impl<'a> Tag<'a> for EndTag {
        fn get_type(&self) -> TagType { TAG_End}
        fn get_name(&'a self) -> &'a str {Default::default()}
}


    pub struct Parser<T> {
        _reader: ~T,
    }
    impl<T: ::std::io::Reader> Parser<T> {
        pub fn new(p: ~T) -> Parser<T> {
            Parser{_reader: p}
        }

        pub fn parse(&mut self) -> ~Tag {
            self._parse(TAG_Unknown, true) 
        }
        fn _parse(&mut self, tag: TagType, named: bool) -> ~Tag {
            let first_tag = match tag {
                TAG_Unknown => TagType::from_u8(self._reader.read_byte().unwrap()),
                _ => tag
            };

            match first_tag {
                TAG_Compound => self._build_compound(named) as ~Tag,
                TAG_Byte => self._build_byte(named) as ~Tag,
                TAG_Short => self._build_short(named) as ~Tag,
                TAG_Int => self._build_int(named) as ~Tag,
                TAG_Long => self._build_long(named) as ~Tag,
                TAG_List => self._build_list(named) as ~Tag,
                TAG_String => self._build_string(named) as ~Tag,
                TAG_Float => self._build_float(named) as ~Tag,
                TAG_Double => self._build_double(named) as ~Tag,
                TAG_End => self._build_end() as ~Tag,
                _ => fail!(format!("Unknown tag type {}", first_tag.to_str()))
            }

        }

        fn _read_name(&mut self) -> ~str {
            let name_len = self._reader.read_be_i16() as uint;
            self._read_string(name_len)
        }
    
        fn _read_string(&mut self, len: uint) -> ~str {
            let bytes = self._reader.read_bytes(len);
            ::std::str::from_utf8_owned(bytes)
        }

        fn _build_string(&mut self, named: bool) -> ~StringTag {
            let name: ~str = match named {
                true => self._read_name(),
                false => Default::default()
            };
            // get length of string
            let len = self._reader.read_be_u16() as uint;
            let this_str = self._read_string(len);

            ~StringTag{_name: name, _str: this_str}
        }

        fn _build_byte(&mut self, named:bool) -> ~ByteTag {
            let name: ~str = match named {
                true => self._read_name(),
                false => Default::default()
            };
            ~ByteTag{_name: name, _val: self._reader.read_i8()}
        }
        
        fn _build_short(&mut self, named: bool) -> ~ShortTag {
            let name: ~str = match named {
                true => self._read_name(),
                false => Default::default()
            };
            ~ShortTag{_name: name, _val: self._reader.read_be_i16()}
        }
        fn _build_int(&mut self, named: bool) -> ~IntTag {
            let name: ~str = match named {
                true => self._read_name(),
                false => Default::default()
            };
            ~IntTag{_name: name, _val: self._reader.read_be_i32()}
        }

        fn _build_long(&mut self, named: bool) -> ~LongTag {
            let name: ~str = match named {
                true => self._read_name(),
                false => Default::default()
            };
            ~LongTag{_name: name, _val: self._reader.read_be_i64()}
        }
        fn _build_float(&mut self, named: bool) -> ~FloatTag {
            let name: ~str = match named {
                true => self._read_name(),
                false => Default::default()
            };
            ~FloatTag{_name: name, _val: self._reader.read_be_f32()}
        }
        fn _build_double(&mut self, named: bool) -> ~DoubleTag {
            let name: ~str = match named {
                true => self._read_name(),
                false => Default::default()
            };
            ~DoubleTag{_name: name, _val: self._reader.read_be_f64()}
        }




        fn _build_list(&mut self, named: bool) -> ~ListTag {
            let name: ~str = match named {
                true => self._read_name(),
                false => Default::default()
            };
            let tag_type : TagType = TagType::from_u8(self._reader.read_u8());
            let num_entries: uint = self._reader.read_be_u32() as uint;
            let mut items : ~[~Tag] = ::std::vec::with_capacity(num_entries); 

            let mut c = 0;
            while (c < num_entries) {
                let t : ~Tag = self._parse(tag_type, false);
                if t.get_type() != tag_type {
                    fail!(format!("Got an unepxected tag type! Wanted {} but found {}", tag_type.to_str(), t.get_type().to_str()));
                }
                items.push(t);
                c += 1;
            }
            ~ListTag {_name: name,
                _item_type: tag_type,
                _items: items,
            }


        }

        fn _build_end(&mut self) -> ~EndTag { ~EndTag }

        fn _build_compound(&mut self, named: bool) -> ~CompoundTag {
            let name: ~str = match named {
                true => self._read_name(),
                false => Default::default()
            };

            let mut items : ~[~Tag] = ::std::vec::with_capacity(5); 

            loop {
                let tag: ~Tag = self._parse(TAG_Unknown, true);
                if tag.get_type() == TAG_End {
                    break;
                }
                items.push(tag);
                // read until we find an end tag
            }
            
            ~CompoundTag {_name: name, _items: items}

        }
    }
   
}

#[test]
fn test_byte() {
    let data: ~str = ~"\x01\x00\x04test4";
    let bytes = ~std::io::mem::MemReader::new(data.into_bytes());
    let mut parser = NBT::Parser::new(bytes);
    let root: ~NBT::Tag = parser.parse();
    assert!(root.get_type() == NBT::TAG_Byte);
    let s = root.get_name();
    //assert!(root.get_name() == ~"test");

}

#[test]
fn test1() {
    let levelp = std::path::Path::new("e.dat");
    let level: std::io::File = std::io::File::open(&levelp).unwrap();
    //let bytes: ~[u8] = level.read_to_end();
    //let iter = ~bytes.move_iter();

    let mut parser = NBT::Parser::new(~level);
    parser.parse();
    //let n: &u8 = iter.next().unwrap();
    //println(format!("byte 1 is {}\n", n.to_str()));

    //NBT::parse(iter);
    //let l: uint = bytes.len();
    //println(format!("byte 1 is {}\n", bytes[0]));

}
